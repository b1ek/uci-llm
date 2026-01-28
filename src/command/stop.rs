use crate::command::{CommandResult, ICommand};
use crate::state::State;

pub struct StopCommand;

impl ICommand for StopCommand {
    async fn execute(&self, _args: Vec<String>, state: &mut State) -> CommandResult {
        if let Some(x) = state.cancel_go.lock().await.clone() { x.cancel(); }
        Ok(())
    }
}
