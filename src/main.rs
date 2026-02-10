use tokio::io::{AsyncBufReadExt, BufReader};

use crate::command::Command;
use crate::state::State;

mod command;
mod state;
mod utils;
mod api;
mod fen2md;

#[tokio::main]
async fn main() {
    let mut state = State::default();

    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    println!("UCI Rust boilerplate");
    println!("This software is GPL-3.0-only; Type license to learn more");

    while let Ok(Some(line)) = lines.next_line().await {
        Command::process_line(line, &mut state).await;
    }

    state.quit().await;
}
