use crate::command::{CommandResult, ICommand};
use crate::outputln;
use crate::state::State;

pub struct UciCommand;

impl ICommand for UciCommand {
    async fn execute(&self, _args: Vec<String>, _state: &mut State) -> CommandResult {
        outputln!("id name Rust UCI Base");
        outputln!("id author b1ek");
        outputln!();
        outputln!("option name Threads type spin min 1 max 1024");
        outputln!("uciok");
        Ok(())
    }
}