use std::fmt::Debug;
use std::process::exit;
use std::sync::Arc;
use std::time::Duration;

use chess::Board;
use tokio::sync::Notify;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::state::options::Options;

mod options;

pub type CancelToken = Arc<Mutex<Option<CancellationToken>>>;
pub type GoStoppedNotification = Arc<Mutex<Notify>>;

#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub cancel_go: CancelToken,
    pub go_stopped_notification: GoStoppedNotification,
    pub options: Options
}

impl State {
    /// Try to quit gracefully, force exit(0) within 500 ms
    pub async fn quit(&self) {
        if let Some(token) = self.cancel_go.lock().await.take() {
            token.cancel();
            
            // allow for up to 500 milliseconds for the go command to stop gracefully
            let mut times = 0;
            while times < 250 {
                times += 1;
                if self.cancel_go.lock().await.is_none() {
                    break;
                }
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        }

        exit(0);
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Board FEN: {}\n", self.board))?;
        f.write_fmt(format_args!("The go command is {}",
            match self.cancel_go.try_lock() {
                Ok(cancel) => if cancel.is_some() { "running" } else { "not running" },
                Err(_) => "<mutex is locked, try again>"
            }
        ))?;
        Ok(())
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            board: Board::default(),
            cancel_go: Arc::new(Mutex::new(None)),
            go_stopped_notification: Arc::new(Mutex::new(Notify::new())),
            options: Options::default()
        }
    }
}