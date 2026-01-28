use crate::command::{CommandResult, ICommand};
use crate::state::State;

pub struct QuitCommand;

impl ICommand for QuitCommand {
    async fn execute(&self, _args: Vec<String>, state: &mut State) -> CommandResult {
        state.quit().await;
        Ok(())
    }
}
