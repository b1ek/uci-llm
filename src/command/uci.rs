use crate::command::{CommandResult, ICommand};
use crate::outputln;
use crate::state::State;

pub struct UciCommand;

impl ICommand for UciCommand {
    async fn execute(&self, _args: Vec<String>, state: &mut State) -> CommandResult {
        outputln!("id name UCI proxy for an LLM");
        outputln!("id author b1ek");
        outputln!();
        outputln!("{}", state.options.format_uci_options().join("\n"));
        outputln!();
        outputln!("uciok");
        Ok(())
    }
}