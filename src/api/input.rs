use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMInput {
    /// This is either a valid FEN or the constant "startpos". If this is "startpos", assume that this is the starting position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fen: Option<String>,
    #[serde(rename = "legalMoves")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_moves: Option<Vec<String>>,
}
