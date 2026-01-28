use crate::outputln;
use crate::state::State;
use crate::command::{
    uci::UciCommand,
    position::PositionCommand,
    debug::DebugCommand,
    go::GoCommand,
    stop::StopCommand,
    quit::QuitCommand,
    isready::IsReadyCommand,
    license::LicenseCommand,
    setoption::SetOptionCommand
};
use crate::utils::consume_args;

mod uci;
mod position;
mod debug;
mod go;
mod stop;
mod quit;
mod isready;
mod license;
mod setoption;

pub enum Command {
    Uci(UciCommand),
    Position(PositionCommand),
    Debug(DebugCommand),
    Go(GoCommand),
    Stop(StopCommand),
    Quit(QuitCommand),
    IsReady(IsReadyCommand),
    License(LicenseCommand),
    SetOption(SetOptionCommand),
}

pub type CommandResult = Result<(), String>;

impl Command {
    pub async fn process_args(args: Vec<String>, state: &mut State) {
        let args = args.iter().map(|x| x.trim().to_string()).collect::<Vec<String>>();

        if args.is_empty() {
            return;
        }

        let command = args.first().unwrap().as_str();
        if let Ok(command) = Command::try_from(command) {
            let args = consume_args(args);
            let res = command.execute(args, state).await;
            if let Err(res) = res {
                outputln!("info string error: {res}");
            }
        }
    }

    pub async fn process_line(line: String, state: &mut State) {
        Command::process_args(line.split(' ').map(|x| x.to_string()).collect(), state).await
    }
}

impl<'a> TryFrom<&'a str> for Command {
    type Error = ();
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "uci" => Ok(Command::Uci(UciCommand)),
            "position" => Ok(Command::Position(PositionCommand)),
            "debug" => Ok(Command::Debug(DebugCommand)),
            "go" => Ok(Command::Go(GoCommand)),
            "stop" => Ok(Command::Stop(StopCommand)),
            "quit" => Ok(Command::Quit(QuitCommand)),
            "isready" => Ok(Command::IsReady(IsReadyCommand)),
            "license" => Ok(Command::License(LicenseCommand)),
            "setoption" => Ok(Command::SetOption(SetOptionCommand)),
            _ => Err(())
        }
    }
}

impl ICommand for Command {
    async fn execute(&self, args: Vec<String>, state: &mut State) -> CommandResult {
        match self {
            Command::Uci(command) => command.execute(args, state).await,
            Command::Position(command) => command.execute(args, state).await,
            Command::Debug(command) => command.execute(args, state).await,
            Command::Go(command) => command.execute(args, state).await,
            Command::Stop(command) => command.execute(args, state).await,
            Command::Quit(command) => command.execute(args, state).await,
            Command::IsReady(command) => command.execute(args, state).await,
            Command::License(command) => command.execute(args, state).await,
            Command::SetOption(command) => command.execute(args, state).await,
        }
    }
}

pub trait ICommand {
    async fn execute(&self, args: Vec<String>, state: &mut State) -> CommandResult;
}