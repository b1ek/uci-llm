use crate::command::{CommandResult, ICommand};
use crate::outputln;
use crate::state::State;

pub struct IsReadyCommand;

impl ICommand for IsReadyCommand {
    async fn execute(&self, _args: Vec<String>, state: &mut State) -> CommandResult {
        if state.cancel_go.lock().await.is_none() {
            outputln!("readyok");
        }
        Ok(())
    }
}