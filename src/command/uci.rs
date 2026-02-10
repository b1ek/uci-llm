use crate::command::{CommandResult, ICommand};
use crate::outputln;
use crate::state::State;
use crate::state::options::Options;

pub struct UciCommand;

impl ICommand for UciCommand {
    async fn execute(&self, _args: Vec<String>, _state: &mut State) -> CommandResult {
        outputln!("id name Rust UCI Base");
        outputln!("id author b1ek");
        outputln!();
        outputln!("{}", Options::format_uci_options().join("\n"));
        outputln!();
        outputln!("uciok");
        Ok(())
    }
}