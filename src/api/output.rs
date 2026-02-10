use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMOutput {
    pub lines: Vec<LLMLines>,
    /// This must be the score of the position in centipawns. For example, 100 means a 1 pawn advantage for white and -100 means a 1 pawn advantage for black and 0 means no advantage for anyone.
    pub eval: f64,
    /// If you see a forced mate sequence, you must fill out this field to the longest number of moves (not plies) before mate. For example, if white can checkmate black in 2 moves but black can delay the checkmate for 5 moves, this must be 5. And if black can checkmate white, this would be a negative number. The eval score is irrelevant if this is not null. This must be null if there is no forced mate sequence.
    pub mate: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LLMLines {
    /// You must always output 3 variations, where in each variation you think through what would happen if a move X would be played. Each variation's first move MUST be in the legal moves list to be a valid variation. One exception in which you can output less than 3 variations is when the legal moves list has less than 3 legal moves. The lines are not sorted in any particular order.
    pub ponder: Vec<String>,
    /// This is an optional field for you to report how much plies down you had analysed. Leave this blank if you don't have concrete information on that. You must not count how many plies you have analysed in order to fill this field, the priority is analyzing the position itself. Only concern yourself with this parameter at the stage when you have completed the analysis.
    pub depth: Option<serde_json::Value>,
}
