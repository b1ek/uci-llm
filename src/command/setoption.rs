use tokio::fs;

use crate::command::{CommandResult, ICommand};
use crate::outputln;
use crate::state::State;

pub struct SetOptionCommand;

impl ICommand for SetOptionCommand {
    async fn execute(&self, args: Vec<String>, state: &mut State) -> CommandResult {
        let name = args.iter().position(|x| x == "name").and_then(|x| args.get(x + 1).cloned());
        let value = args.iter().position(|x| x == "value").and_then(|x| Some(args.iter().cloned().skip(x + 1).collect::<Vec<String>>().join(" ")));

        if name.is_none() {
            outputln!("info string error: setoption name is required");
        }
        if value.is_none() {
            outputln!("info string error: setoption value is required");
        }

        if name.is_none() || value.is_none() {
            return Err("invalid setoption syntax".into())
        }

        let name = name.unwrap();
        let value = value.unwrap();

        if name == "AdditionalInstructionsFile" {
            let instructions = fs::read_to_string(&value).await;
            if let Err(err) = instructions {
                outputln!("info string error: couldn't get additional instructions from {value}: {err}");
            } else {
                state.options.additional_instructions = instructions.unwrap();
            }
        }

        state.options.set_by_name_value(&name, &value).map_err(|x| format!("when setting option {name}: {x}"))?;
        
        Ok(())
    }
}