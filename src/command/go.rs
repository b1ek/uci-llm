use async_openai::config::OpenAIConfig;
use rand::rng;
use rand::seq::SliceRandom;
use std::str::FromStr;

use async_openai::Client;
use async_openai::types::responses::{CreateResponseArgs, InputContent, InputItem, InputMessage, InputParam, InputTextContent, Response, ResponseTextParam};
use chess::{Board, ChessMove, MoveGen};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_util::sync::CancellationToken;

use crate::command::{CommandResult, ICommand};
use crate::fen2md::fen2md;
use crate::state::options::Options;
use crate::state::{GoStoppedNotification, State};
use crate::outputln;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitEval {
    ponder: Vec<String>,
    eval: f32,
    mate: Option<f32>,
    depth: Option<f32>,
    reasoning: Option<String>
}

pub struct GoCommand;

fn process_keyword(args: &Vec<String>, keyword: String) -> Option<String> {
    let keywords = vec![
        "searchmoves",
        "ponder",
        "wtime",
        "btime",
        "winc",
        "binc",
        "movestogo",
        "depth",
        "nodes",
        "mate",
        "movetime",
        "infinite",
    ];

    let mut value = String::new();
    let mut value_start = false;
    for arg in args.iter() {
        if *arg == keyword {
            value_start = true;
            continue;
        }
        if value_start {
            if keywords.iter().find(|x| arg == **x).is_some() {
                break;
            }
            value = value + &arg + " ";
        }
    }

    if value == "" {
        None
    } else {
        Some(value.trim_end_matches(" ").to_string())
    }
}

fn legal_moves(board: Board) -> Vec<ChessMove> {
    MoveGen::new_legal(&board).collect()
}

fn best(mov: ChessMove) {
    outputln!("bestmove {mov}");
}

async fn try_get_bestmove(options: Options, board: Board, legal_moves: Vec<ChessMove>) -> Option<(ChessMove, String, String, u32)> {
    let mut submit_eval_schema = json!({
        "type": "object",
        "properties": {
            "ponder": {
                "type": "array",
                "items": {
                    "type": "string",
                    "enum": legal_moves.iter().map(|x| x.to_string()).collect::<Vec<String>>()
                },
                "minItems": 1,
                "description": "This is the list of moves you think is the best line. For example, if you think that the best line from a starting position is 1. d4 d5, this field should be [\"d2d4\",\"d7d5\"], provided \"d2d4\" is one of the legal moves."
            },
            "eval": {
                "type": "number",
                "description": "The evalutaion score in centipawns. For example, if this is 0.9, white has a 0.9 pawn in advantage and if it is -1, black has a 1 pawn advantage, and if this is 0, the game is equal."
            },
            "mate": {
                "type": [ "number", "null" ],
                "description": "If there is a forced mate line, set this to the amount of moves until mate. Additionally, if the current player is going to get mated, this should be a negative value. For example, if it is currently black to move and black is going to get mated in 3 moves, this should be -3, and if it is white's move and black is going to be mated in 3 moves, this should be 3. If this field is set, the \"eval\" field will be ignored."
            },
            "depth": {
                "type": [ "number", "null" ],
                "description": "An approximate depth in half-moves you have analysed for. Do not keep track of exact moves you had analysed."
            }
        },
        "required": ["ponder", "eval"]
    });
    if options.output_reasoning {
        let props = submit_eval_schema.get_mut("properties").unwrap().as_object_mut().unwrap();
        props.insert("reasoning".to_string(), json!({
            "type": "string",
            "description": "Explain why you think this is the best move and why the evaluation is what it is"
        }));

        submit_eval_schema.as_object_mut().unwrap().get_mut("required").unwrap().as_array_mut().unwrap().push("reasoning".into());
    }

    let fen = if options.fenasmd {
        let fen = fen2md(board.to_string());
        if let Err(fen) = fen {
            outputln!("info string error: could not parse fen: {fen:?}");
            return None;
        }
        fen.unwrap()
    } else {
        "FEN: ".to_string() + &board.to_string()
    };

    if options.debug {
        outputln!("info string going to send off a request now");
    }

    let input_data = vec![
        InputItem::Item(async_openai::types::responses::Item::Message(
            async_openai::types::responses::MessageItem::Input(InputMessage {
                content: vec![InputContent::InputText(InputTextContent {
                    text: include_str!("../assets/go-simple.md").to_string(),
                })],
                role: async_openai::types::responses::InputRole::System,
                status: None,
            }),
        )),
        InputItem::Item(async_openai::types::responses::Item::Message(
            async_openai::types::responses::MessageItem::Input(InputMessage {
                content: vec![InputContent::InputText(InputTextContent { text: fen })],
                role: async_openai::types::responses::InputRole::User,
                status: None,
            }),
        )),
        InputItem::Item(async_openai::types::responses::Item::Message(
            async_openai::types::responses::MessageItem::Input(InputMessage {
                content: vec![InputContent::InputText(InputTextContent {
                    text: "Legal moves: ".to_string()
                        + legal_moves
                            .iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                            .as_str(),
                })],
                role: async_openai::types::responses::InputRole::User,
                status: None,
            }),
        )),
    ];

    let req = CreateResponseArgs::default()
        .model(options.apimodel)
        .input(InputParam::Items(input_data.clone()))
        .text(ResponseTextParam {
            format: async_openai::types::responses::TextResponseFormatConfiguration::JsonSchema(
                async_openai::types::responses::ResponseFormatJsonSchema { description: Some("Evaluation output schema".into()), name: "Evaluation output".into(), schema: Some(submit_eval_schema), strict: Some(true) }
            ),
            verbosity: Some(async_openai::types::responses::Verbosity::Low)
        })
        .build()
        .unwrap();

    if options.debug {
        outputln!("info string debug going to send this: {}", serde_json::to_string(&req).unwrap());
    }

    let client = Client::with_config(
        OpenAIConfig::new()
            .with_api_base(options.apibaseurl)
            .with_api_key(options.apikey),
    );
    let res = client.responses().create(req).await;
    if let Err(res) = res {
        outputln!("info string error network error while fetching response: {res}");
        return None;
    }

    let res = res.unwrap();

    if options.debug {
        outputln!("info string debug received response: {}", serde_json::to_string(&res).unwrap());
    }

    let eval = serde_json::from_str(&res.output_text().unwrap());
    if let Err(err) = &eval {
        outputln!("info string error: could not parse ai's response: {err}, {}", &res.output_text().unwrap());
        return None;
    }
    let eval: SubmitEval = eval.unwrap();
    
    let depth = eval.depth.unwrap_or(1.0) as u32;
    let score = {
        if let Some(mate) = eval.mate {
            format!("mate {mate}")
        } else {
            let cp = eval.eval;
            format!("cp {cp}")
        }
    };

    if let Some(exp) = eval.reasoning {
        if exp.len() != 0 {
            outputln!("info string reasoning: {exp}");
        }
    }

    if eval.ponder.len() == 0 {
        outputln!("info string error: ai returned no ponder");
        return None;
    }

    let bm = (&eval.ponder).get(0).unwrap();
    if legal_moves.iter().find(|x| x.to_string() == *bm).is_none() {
        outputln!("info string error: ai returned an illegal move");
        return None;
    }
        
    let bm = ChessMove::from_str(bm).unwrap();
    let pv = eval.ponder.join(" ");

    Some((bm, pv, score, depth))
}

async fn go(
    args: Vec<String>,
    board: Board,
    threads: u16,
    _cancellation: CancellationToken,
    stopped_notification: GoStoppedNotification,
    options: Options,
) {
    let _ = threads;

    if options.debug {
        outputln!("info string go command worker entered with these options: {options:?}");
    }

    #[allow(unused)]
    let (
        searchmoves,
        ponder,
        wtime,
        btime,
        winc,
        binc,
        movestogo,
        depth,
        nodes,
        mate,
        movetime,
        infinite,
    ) = (
        process_keyword(&args, "searchmoves".into()),
        process_keyword(&args, "ponder".into()),
        process_keyword(&args, "wtime".into()),
        process_keyword(&args, "btime".into()),
        process_keyword(&args, "winc".into()),
        process_keyword(&args, "binc".into()),
        process_keyword(&args, "movestogo".into()),
        process_keyword(&args, "depth".into()),
        process_keyword(&args, "nodes".into()),
        process_keyword(&args, "mate".into()),
        process_keyword(&args, "movetime".into()),
        process_keyword(&args, "infinite".into()),
    );

    let legal_moves = legal_moves(board);
    if legal_moves.len() == 0 {
        outputln!(
            "info string error: refusing to evaluate on a board with no legal moves, considering the position draw by stalemate"
        );
        outputln!("info depth 1 score cp 0");
        return;
    }

    if legal_moves.len() == 1 {
        // not going to evaluate a forced position
        return best(legal_moves.get(0).unwrap().clone());
    }

    for i in 0..=options.apimaxtries {
        if let Some((bm, pv, score, depth)) = try_get_bestmove(options.clone(), board, legal_moves.clone()).await {
            outputln!("info depth {depth} score {score} pv {pv}");

            stopped_notification.lock().await.notify_waiters();

                outputln!("bestmove {bm}");
                return;
        } else {
            outputln!("info error: no move found, going to try again ({}/{})", i+1, options.apimaxtries);
        }
    }

    let mut legal_moves = legal_moves;
    legal_moves.shuffle(&mut rng());

    outputln!("info error: no bestmove was found in 3 tries, going to pick a random move");
    outputln!("bestmove {}", legal_moves.first().unwrap());
}

impl ICommand for GoCommand {
    async fn execute(&self, args: Vec<String>, state: &mut State) -> CommandResult {
        let cancel_go = state.cancel_go.clone();

        {
            let guard = cancel_go.lock().await;
            if guard.is_some() {
                outputln!("info string seems another go is running, refusing to run another one");
                return Ok(());
            }
        }

        let options = state.options.clone();

        let token = CancellationToken::new();
        let task_token = token.clone();

        {
            let mut guard = cancel_go.lock().await;
            *guard = Some(token);
        }

        let board = state.board;
        let threads = state.options.threads;
        let go_stop_notify = state.go_stopped_notification.clone();

        tokio::spawn(async move {
            tokio::select! {
                _ = task_token.cancelled() => {}
                _ = go(args, board, threads, task_token.clone(), go_stop_notify, options) => {}
            }

            let mut guard = cancel_go.lock().await;
            *guard = None;
        });

        Ok(())
    }
}
