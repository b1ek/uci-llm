use crate::command::{CommandResult, ICommand};
use crate::outputln;
use crate::state::State;

pub struct LicenseCommand;

impl ICommand for LicenseCommand {
    async fn execute(&self, _args: Vec<String>, _state: &mut State) -> CommandResult {
        outputln!("{}", include_str!("../assets/LICENSE.txt"));
        Ok(())
    }
}