use crate::command::{CommandResult, ICommand};
use crate::outputln;
use crate::state::State;

pub struct DebugCommand;

impl ICommand for DebugCommand {
    async fn execute(&self, _args: Vec<String>, state: &mut State) -> CommandResult {
        outputln!("{state:?}");
        Ok(())
    }
}