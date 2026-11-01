use super::model::{
    ConnectFourState, GameKind, GameState, MemoryMatchState, TicTacToeState, WordScrambleState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GameAction {
    TicTacToeMove { row: usize, column: usize },
    ConnectFourDrop { column: usize },
    MemoryReveal { position: usize },
    WordAnswer { answer: String },
}

impl GameAction {
    pub fn parse(game: GameKind, input: &str) -> Result<Self, GameError> {
        match game {
            GameKind::TicTacToe => {
                let mut parts = input.split(',').map(str::trim);
                let row = parts
                    .next()
                    .ok_or(GameError::InvalidAction)?
                    .parse::<usize>()
                    .map_err(|_| GameError::InvalidAction)?;
                let column = parts
                    .next()
                    .ok_or(GameError::InvalidAction)?
                    .parse::<usize>()
                    .map_err(|_| GameError::InvalidAction)?;
                if parts.next().is_some() || !(1..=3).contains(&row) || !(1..=3).contains(&column) {
                    return Err(GameError::InvalidAction);
                }
                Ok(Self::TicTacToeMove {
                    row: row - 1,
                    column: column - 1,
                })
            }
            GameKind::ConnectFour => {
                let column = input.trim().parse::<usize>().map_err(|_| GameError::InvalidAction)?;
                if !(1..=7).contains(&column) { return Err(GameError::InvalidAction); }
                Ok(Self::ConnectFourDrop { column: column - 1 })
            }
            GameKind::MemoryMatch => {
                let position = input
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| GameError::InvalidAction)?;
                if !(1..=8).contains(&position) {
                    return Err(GameError::InvalidAction);
                }
                Ok(Self::MemoryReveal {
                    position: position - 1,
                })
            }
            GameKind::WordScramble => {
                let answer = input.trim();
                if answer.is_empty() || answer.chars().count() > 32 {
                    return Err(GameError::InvalidAction);
                }
                Ok(Self::WordAnswer {
                    answer: answer.to_owned(),
                })
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GameResolution {
    pub winner_index: Option<usize>,
    pub is_draw: bool,
    pub is_complete: bool,
    pub message: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GameError {
    UnknownGame,
    NotJoinable,
    AlreadyJoined,
    WrongGuild,
    NotActive,
    NotParticipant,
    NotYourTurn,
    AlreadyFinished,
    InvalidAction,
    OccupiedCell,
    FullColumn,
    AlreadyMatched,
    SamePosition,
    RuntimeUnavailable,
}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::UnknownGame => "That game is not available. Use .games to browse playable games.",
            Self::NotJoinable => {
                "That session is not waiting for another player or could not be found."
            }
            Self::AlreadyJoined => "You are already in this session.",
            Self::WrongGuild => "Sessions are private to the Discord server where they were created.",
            Self::NotActive => "This session is not accepting game actions.",
            Self::NotParticipant => "You are not a participant in this session.",
            Self::NotYourTurn => "It is not your turn.",
            Self::AlreadyFinished => "This session has already ended.",
            Self::InvalidAction => {
                "That action is not valid. Check the controls shown on the session."
            }
            Self::OccupiedCell => "That square is already occupied.",
            Self::FullColumn => "That column is full. Choose another column.",
            Self::AlreadyMatched => "That position is already matched. Choose a face-down position.",
            Self::SamePosition => "Choose a different second position.",
            Self::RuntimeUnavailable => "The game runtime is temporarily unavailable. Try again.",
        })
    }
}

impl std::error::Error for GameError {}

pub(super) fn apply_action(
    state: &mut GameState,
    player_index: usize,
    action: GameAction,
) -> Result<GameResolution, GameError> {
    match (state, action) {
        (GameState::TicTacToe(state), GameAction::TicTacToeMove { row, column }) => {
            apply_tic_tac_toe(state, player_index, row, column)
        }
        (GameState::ConnectFour(state), GameAction::ConnectFourDrop { column }) => {
            apply_connect_four(state, player_index, column)
        }
        (GameState::MemoryMatch(state), GameAction::MemoryReveal { position }) => {
            apply_memory_match(state, position)
        }
        (GameState::WordScramble(state), GameAction::WordAnswer { answer }) => {
            apply_word_scramble(state, answer)
        }
        _ => Err(GameError::InvalidAction),
    }
}

fn apply_tic_tac_toe(
    state: &mut TicTacToeState,
    player_index: usize,
    row: usize,
    column: usize,
) -> Result<GameResolution, GameError> {
    if row >= 3 || column >= 3 {
        return Err(GameError::InvalidAction);
    }
    if state.board[row][column] != 0 {
        return Err(GameError::OccupiedCell);
    }
    let marker = (player_index + 1) as u8;
    state.board[row][column] = marker;
    if let Some(winner) = state.winner() {
        return Ok(GameResolution {
            winner_index: Some(usize::from(winner)),
            is_draw: false,
            is_complete: true,
            message: "Three in a row. The game is complete.",
        });
    }
    if state.is_full() {
        return Ok(GameResolution {
            winner_index: None,
            is_draw: true,
            is_complete: true,
            message: "The board is full. The game is a draw.",
        });
    }
    state.current_player = 1 - state.current_player;
    Ok(GameResolution {
        winner_index: None,
        is_draw: false,
        is_complete: false,
        message: "Move accepted.",
    })
}

fn apply_connect_four(
    state: &mut ConnectFourState,
    player_index: usize,
    column: usize,
) -> Result<GameResolution, GameError> {
    if column >= 7 {
        return Err(GameError::InvalidAction);
    }
    let height = usize::from(state.heights[column]);
    if height >= 6 { return Err(GameError::FullColumn); }
    let marker = (player_index + 1) as u8;
    state.columns[column][height] = marker;
    state.heights[column] += 1;
    if state.has_four(column, height, marker) {
        return Ok(GameResolution {
            winner_index: Some(player_index),
            is_draw: false,
            is_complete: true,
            message: "Four connected. The game is complete.",
        });
    }
    if state.is_full() {
        return Ok(GameResolution {
            winner_index: None,
            is_draw: true,
            is_complete: true,
            message: "The board is full. The game is a draw.",
        });
    }
    state.current_player = 1 - state.current_player;
    Ok(GameResolution {
        winner_index: None,
        is_draw: false,
        is_complete: false,
        message: "Disc placed.",
    })
}

fn apply_memory_match(
    state: &mut MemoryMatchState,
    position: usize,
) -> Result<GameResolution, GameError> {
    if position >= 8 {
        return Err(GameError::InvalidAction);
    }
    if state.matched & (1 << position) != 0 {
        return Err(GameError::AlreadyMatched);
    }
    let matched_pair = if let Some(first) = state.first_reveal {
        if usize::from(first) == position { return Err(GameError::SamePosition); }
        state.attempts = state.attempts.saturating_add(1);
        state.last_pair = Some((first, position as u8));
        let matched = state.pairs[usize::from(first)] == state.pairs[position];
        if matched {
            state.matched |= (1 << usize::from(first)) | (1 << position);
            state.pairs_found += 1;
        }
        state.first_reveal = None;
        matched
    } else {
        state.last_pair = None;
        state.first_reveal = Some(position as u8);
        false
    };
    let complete = state.pairs_found == 4;
    Ok(GameResolution {
        winner_index: None,
        is_draw: false,
        is_complete: complete,
        message: if complete {
            "All four pairs found. Excellent memory."
        } else if matched_pair {
            "Pair found. Choose another face-down position."
        } else if state.last_pair.is_some() {
            "No match. Both tiles remain visible until your next reveal."
        } else {
            "Position revealed. Choose a second position."
        },
    })
}

fn apply_word_scramble(
    state: &mut WordScrambleState,
    answer: String,
) -> Result<GameResolution, GameError> {
    if state.attempts_left == 0 {
        return Err(GameError::AlreadyFinished);
    }
    if answer.trim().eq_ignore_ascii_case(state.answer) {
        state.attempts_left = 0;
        return Ok(GameResolution {
            winner_index: Some(0),
            is_draw: false,
            is_complete: true,
            message: "Correct. You solved the scramble.",
        });
    }
    state.attempts_left -= 1;
    let complete = state.attempts_left == 0;
    Ok(GameResolution {
        winner_index: None,
        is_draw: false,
        is_complete: complete,
        message: if complete {
            "No attempts remain. The answer was ORBIT."
        } else {
            "Not quite. Try another arrangement."
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_parser_rejects_out_of_range_and_malformed_actions() {
        assert_eq!(
            GameAction::parse(GameKind::TicTacToe, "1,4"),
            Err(GameError::InvalidAction)
        );
        assert_eq!(
            GameAction::parse(GameKind::TicTacToe, "1"),
            Err(GameError::InvalidAction)
        );
        assert_eq!(
            GameAction::parse(GameKind::ConnectFour, "8"),
            Err(GameError::InvalidAction)
        );
        assert_eq!(
            GameAction::parse(GameKind::MemoryMatch, "0"),
            Err(GameError::InvalidAction)
        );
        assert_eq!(
            GameAction::parse(GameKind::WordScramble, " "),
            Err(GameError::InvalidAction)
        );
    }

    #[test]
    fn tic_tac_toe_detects_a_horizontal_win() {
        let mut state = GameState::TicTacToe(TicTacToeState::default());
        for (player, row, column) in [(0, 0, 0), (1, 1, 0), (0, 0, 1), (1, 1, 1), (0, 0, 2)] {
            let result = apply_action(
                &mut state,
                player,
                GameAction::TicTacToeMove { row, column },
            )
            .expect("legal move");
            if player == 0 && row == 0 && column == 2 {
                assert_eq!(result.winner_index, Some(0));
                assert!(result.is_complete);
            }
        }
    }

    #[test]
    fn occupied_tic_tac_toe_cells_are_rejected_without_advancing() {
        let mut state = GameState::TicTacToe(TicTacToeState::default());
        apply_action(
            &mut state,
            0,
            GameAction::TicTacToeMove { row: 1, column: 1 },
        )
        .expect("first move");
        assert_eq!(
            apply_action(&mut state, 1, GameAction::TicTacToeMove { row: 1, column: 1 }),
            Err(GameError::OccupiedCell)
        );
        assert_eq!(state.current_player_index(), 1);
    }

    #[test]
    fn connect_four_detects_a_vertical_win() {
        let mut state = GameState::ConnectFour(ConnectFourState::default());
        for (turn, player) in [0, 1, 0, 1, 0, 1, 0].into_iter().enumerate() {
            let result = apply_action(
                &mut state,
                player,
                GameAction::ConnectFourDrop {
                    column: if player == 0 { 0 } else { 1 },
                },
            )
            .expect("legal drop");
            if turn == 6 {
                assert_eq!(result.winner_index, Some(0));
                assert!(result.is_complete);
            } else {
                assert_eq!(result.winner_index, None);
            }
        }
    }

    #[test]
    fn memory_match_completes_only_after_all_pairs_are_found() {
        let mut state = GameState::MemoryMatch(MemoryMatchState::default());
        for (a, b) in [(0, 4), (1, 5), (2, 6), (3, 7)] {
            apply_action(&mut state, 0, GameAction::MemoryReveal { position: a })
                .expect("first reveal");
            let result = apply_action(&mut state, 0, GameAction::MemoryReveal { position: b })
                .expect("pair");
            if b == 7 {
                assert!(result.is_complete);
            } else {
                assert!(!result.is_complete);
            }
        }
    }

    #[test]
    fn word_scramble_has_bounded_attempts_and_case_insensitive_answer() {
        let mut state = GameState::WordScramble(WordScrambleState::default());
        apply_action(
            &mut state,
            0,
            GameAction::WordAnswer {
                answer: "wrong".into(),
            },
        )
        .expect("wrong guess");
        let result = apply_action(
            &mut state,
            0,
            GameAction::WordAnswer {
                answer: "orbit".into(),
            },
        )
        .expect("correct guess");
        assert_eq!(result.winner_index, Some(0));
        assert!(result.is_complete);
    }
}
