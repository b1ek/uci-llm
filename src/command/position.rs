use std::str::FromStr;

use chess::{Board, ChessMove};

use crate::command::{CommandResult, ICommand};
use crate::state::State;
use crate::utils::consume_args;

pub struct PositionCommand;

impl PositionCommand {
    fn startpos(&self, state: &mut State) {
        state.board = Board::default();
    }

    fn fen(&self, args: &Vec<String>, state: &mut State) -> CommandResult {
        let fen = args
            .iter()
            .take_while(|x| *x != "moves")
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        match Board::from_str(&fen) {
            Err(err) => Err(format!("{err}")),
            Ok(board) => {
                state.board = board;
                Ok(())
            }
        }
    }
}

impl ICommand for PositionCommand {
    async fn execute(&self, args: Vec<String>, state: &mut State) -> CommandResult {
        if args.is_empty() {
            return Ok(());
        }

        let subarg = args.first().unwrap().clone();
        let args = consume_args(args);

        if subarg == "startpos" {
            self.startpos(state);
        }

        if subarg == "fen" {
            self.fen(&args, state)?;
        }

        if subarg != "startpos" && subarg != "fen" {
            return Ok(());
        }

        let mut moves_start = false;
        for arg in args {
            if arg == "moves" {
                moves_start = true;
                continue;
            }
            if moves_start {
                match ChessMove::from_str(&arg) {
                    Ok(mov) => {
                        if state.board.piece_on(mov.get_source()).is_none() {
                            return Err(format!("Invalid move {arg}: There's no piece on {}!", mov.get_source()));
                        }
                        state.board = state.board.make_move_new(mov)
                    },
                    Err(err) => {
                        return Err(format!("Invalid move {arg}: {err}"));
                    }
                }
            }
        }

        Ok(())
    }
}