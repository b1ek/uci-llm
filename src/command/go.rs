use std::str::FromStr;
use std::time::Duration;

use chess::{Board, ChessMove};
use tokio_util::sync::CancellationToken;

use crate::command::{CommandResult, ICommand};
use crate::outputln;
use crate::state::{GoStoppedNotification, State};

pub struct GoCommand;

fn process_keyword(args: &Vec<String>, keyword: String) -> Option<String> {
    let keywords = vec![ "searchmoves", "ponder", "wtime", "btime", "winc", "binc", "movestogo", "depth", "nodes", "mate", "movetime", "infinite" ];
    
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

async fn go(args: Vec<String>, board: Board, threads: u16, cancellation: CancellationToken, stopped_notification: GoStoppedNotification) {
    let _ = threads;
    let mut bestmove = ChessMove::from_str("e2e4").unwrap();

    #[allow(unused)]
    let (searchmoves, ponder, wtime, btime, winc, binc, movestogo, depth, nodes, mate, movetime, infinite) = (
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
        process_keyword(&args, "infinite".into())
    );

    async fn single_iteration_of_move_finding_algorithm(args: Vec<String>, board: Board) -> (ChessMove, bool) {
        let _ = (args, board);

        // a single iteration of your best move finding algorithm would go here, but it defaults to d4 for now.
        tokio::time::sleep(Duration::from_secs(2)).await;

        // the true at the end means that it should stop
        (ChessMove::from_str("d2d4").unwrap(), true)
    }

    loop {
        if cancellation.is_cancelled() {
            break;
        }

        tokio::select! {
            _ = cancellation.cancelled() => {}
            (mov, done) = single_iteration_of_move_finding_algorithm(args.clone(), board) => {
                bestmove = mov;
                if done {
                    break;
                }
            }
        };
    }

    stopped_notification.lock().await.notify_waiters();

    outputln!("bestmove {}", bestmove);
}

impl ICommand for GoCommand {
    async fn execute(&self, args: Vec<String>, state: &mut State) -> CommandResult {
        let cancel_go = state.cancel_go.clone();

        {
            let guard = cancel_go.lock().await;
            if guard.is_some() {
                return Ok(());
            }
        }

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
                _ = go(args, board, threads, task_token.clone(), go_stop_notify) => {}
            }

            let mut guard = cancel_go.lock().await;
            *guard = None;
        });

        Ok(())
    }
}