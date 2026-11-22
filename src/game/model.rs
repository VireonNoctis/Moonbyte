use std::fmt;

use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};

use super::engine::{GameAction, GameError, apply_action, is_valid_word_guess};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameKind {
    TicTacToe,
    ConnectFour,
    MemoryMatch,
    Minesweeper,
    WordScramble,
    SequenceRecall,
    TwentyFortyEight,
}

impl GameKind {
    pub const ALL: [Self; 7] = [
        Self::TicTacToe,
        Self::ConnectFour,
        Self::MemoryMatch,
        Self::Minesweeper,
        Self::WordScramble,
        Self::SequenceRecall,
        Self::TwentyFortyEight,
    ];
    pub const fn definition(self) -> GameDefinition {
        match self {
            Self::TicTacToe => GameDefinition {
                id: "tic-tac-toe",
                name: "Tic-tac-toe",
                description: "A two-player strategy game on a 3×3 board.",
                players: "2 players",
                controls: "Use .action <session> <row>,<column> with 1-based coordinates.",
                rules: "Take turns placing X or O in an empty square. The first player to make a line of three horizontally, vertically, or diagonally wins. If all nine squares are filled without a line of three, the game is a draw.",
            },
            Self::ConnectFour => GameDefinition {
                id: "connect-four",
                name: "Connect Four",
                description: "Drop discs into seven columns and connect four in a row.",
                players: "2 players",
                controls: "Use .action <session> <column> with a column from 1 to 7.",
                rules: "Take turns dropping a disc into one of seven columns. Each disc falls to the lowest open space. Connect four of your discs horizontally, vertically, or diagonally to win. If the board fills without a connected four, the game is a draw.",
            },
            Self::MemoryMatch => GameDefinition {
                id: "memory-match",
                name: "Memory Match",
                description: "Find four pairs by remembering where each symbol appears.",
                players: "1 player",
                controls: "Use .action <session> <position> with a position from 1 to 8.",
                rules: "Reveal two positions at a time on the 2×4 board. Matching pairs stay visible. When the two positions do not match, both remain visible until your next reveal. Find all four pairs to complete the game. The starter layout is fixed and is not shuffled.",
            },

            Self::Minesweeper => GameDefinition {
                id: "minesweeper",
                name: "Minesweeper",
                description: "Solve an 8×8 minefield using number clues and careful reveals.",
                players: "1 player",
                controls: "Use .action <session> reveal <row>,<column> or flag <row>,<column> with coordinates 1–8. Repeat flag to remove it.",
                rules: "The board contains ten hidden mines. Your first reveal is always safe. Numbers show adjacent mines; reveal every safe cell to win. Flags are optional markers and can be toggled off. Revealing a mine ends the session. Mine placement uses operating-system-backed randomness and is not fairness-certified.",
            },
            Self::WordScramble => GameDefinition {
                id: "word-scramble",
                name: "Word Scramble",
                description: "Reconstruct an original five-letter word from its scrambled letters.",
                players: "1 player",
                controls: "Use .action <session> <answer> with 1–32 ASCII letters.",
                rules: "Rearrange BITOR into the hidden five-letter word. You have three distinct guesses; a repeated guess does not consume an attempt. Guesses are case-insensitive and accept 1–32 ASCII letters. The answer is revealed when all attempts are used.",
            },
            Self::SequenceRecall => GameDefinition {
                id: "sequence-recall",
                name: "Sequence Recall",
                description: "Memorise a growing four-symbol pattern, then repeat it.",
                players: "1 player",
                controls: "Use .action <session> hide, then submit the full pattern with comma-separated values 1–4.",
                rules: "Memorise the pattern shown at each level, then use hide to conceal it. Repeat the full pattern in order with values 1–4 separated by commas. Each correct answer adds one symbol; an incorrect answer ends the session. Complete all eight levels to finish. Sessions rotate through four authored patterns selected deterministically from the session ID; outcomes are not random or fairness-certified.",
            },
            Self::TwentyFortyEight => GameDefinition {
                id: "2048",
                name: "2048",
                description: "Slide numbered tiles, combine matches, and build toward 2048.",
                players: "1 player",
                controls: "Use .action <session> up, down, left, or right. W/A/S/D also work.",
                rules: "Slide every tile in one direction. Matching adjacent values merge once per move and add their value to your score. A new 2 tile appears after each valid move; a 4 tile appears one time in ten. Reach 2048 to complete the game, or continue until no legal moves remain. Initial and spawned tiles use operating-system-backed randomness. This score-only game has no public fairness transcript.",
            },
        }
    }
    pub const fn requires_opponent(self) -> bool {
        matches!(self, Self::TicTacToe | Self::ConnectFour)
    }
    pub fn parse(value: &str) -> Result<Self, GameError> {
        match value.trim().to_ascii_lowercase().as_str() {
            "tic-tac-toe" | "tictactoe" | "xo" => Ok(Self::TicTacToe),
            "connect-four" | "connect4" | "c4" => Ok(Self::ConnectFour),
            "memory-match" | "memory" => Ok(Self::MemoryMatch),
            "minesweeper" | "mines" => Ok(Self::Minesweeper),
            "word-scramble" | "scramble" => Ok(Self::WordScramble),
            "sequence-recall" | "sequence" | "recall" => Ok(Self::SequenceRecall),
            "2048" | "twenty-forty-eight" | "twentyfortyeight" => Ok(Self::TwentyFortyEight),
            _ => Err(GameError::UnknownGame),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GameDefinition {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub players: &'static str,
    pub controls: &'static str,
    pub rules: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    WaitingForPlayers,
    Active,
    Completed,
    Abandoned,
}

impl fmt::Display for GamePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::WaitingForPlayers => "waiting for players",
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Abandoned => "ended",
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(super) enum GameState {
    TicTacToe(TicTacToeState),
    ConnectFour(ConnectFourState),
    MemoryMatch(MemoryMatchState),
    Minesweeper(MinesweeperState),
    WordScramble(WordScrambleState),
    SequenceRecall(SequenceRecallState),
    TwentyFortyEight(TwentyFortyEightState),
}

impl GameState {
    fn new(kind: GameKind, session_id: &str) -> Result<Self, GameError> {
        Ok(match kind {
            GameKind::TicTacToe => Self::TicTacToe(TicTacToeState::default()),
            GameKind::ConnectFour => Self::ConnectFour(ConnectFourState::default()),
            GameKind::MemoryMatch => Self::MemoryMatch(MemoryMatchState::default()),
            GameKind::Minesweeper => Self::Minesweeper(MinesweeperState::default()),
            GameKind::WordScramble => Self::WordScramble(WordScrambleState::default()),
            GameKind::SequenceRecall => {
                Self::SequenceRecall(SequenceRecallState::for_session(session_id))
            }
            GameKind::TwentyFortyEight => Self::TwentyFortyEight(TwentyFortyEightState::new()?),
        })
    }
    pub(super) fn current_player_index(&self) -> usize {
        match self {
            Self::TicTacToe(s) => usize::from(s.current_player),
            Self::ConnectFour(s) => usize::from(s.current_player),
            Self::MemoryMatch(_)
            | Self::Minesweeper(_)
            | Self::WordScramble(_)
            | Self::SequenceRecall(_)
            | Self::TwentyFortyEight(_) => 0,
        }
    }
    fn render(&self) -> String {
        match self {
            Self::TicTacToe(s) => s.render(),
            Self::ConnectFour(s) => s.render(),
            Self::MemoryMatch(s) => s.render(),
            Self::Minesweeper(s) => s.render(),
            Self::WordScramble(s) => s.render(),
            Self::SequenceRecall(s) => s.render(),
            Self::TwentyFortyEight(s) => s.render(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
pub(super) struct TicTacToeState {
    pub(super) board: [[u8; 3]; 3],
    pub(super) current_player: u8,
}

impl TicTacToeState {
    fn render(&self) -> String {
        let mut out = String::new();
        for (ri, row) in self.board.iter().enumerate() {
            if ri > 0 {
                out.push_str("───┼───┼───\n");
            }
            for (ci, cell) in row.iter().enumerate() {
                if ci > 0 {
                    out.push_str(" │ ");
                }
                out.push(match cell {
                    0 => '·',
                    1 => 'X',
                    _ => 'O',
                });
            }
            out.push('\n');
        }
        out
    }
    pub(super) fn winner(&self) -> Option<u8> {
        let b = &self.board;
        let lines = [
            [b[0][0], b[0][1], b[0][2]],
            [b[1][0], b[1][1], b[1][2]],
            [b[2][0], b[2][1], b[2][2]],
            [b[0][0], b[1][0], b[2][0]],
            [b[0][1], b[1][1], b[2][1]],
            [b[0][2], b[1][2], b[2][2]],
            [b[0][0], b[1][1], b[2][2]],
            [b[0][2], b[1][1], b[2][0]],
        ];
        lines.into_iter().find_map(|line| {
            (line[0] != 0 && line[0] == line[1] && line[1] == line[2]).then(|| line[0] - 1)
        })
    }
    pub(super) fn is_full(&self) -> bool {
        self.board.iter().flatten().all(|cell| *cell != 0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default, Serialize, Deserialize)]
pub(super) struct ConnectFourState {
    pub(super) columns: [[u8; 6]; 7],
    pub(super) heights: [u8; 7],
    pub(super) current_player: u8,
}

impl ConnectFourState {
    fn render(&self) -> String {
        let mut out = String::new();
        for row in (0..6).rev() {
            for (col, column) in self.columns.iter().enumerate() {
                if col > 0 {
                    out.push(' ');
                }
                out.push(match column[row] {
                    0 => '·',
                    1 => 'X',
                    _ => 'O',
                });
            }
            out.push('\n');
        }
        out.push_str("1 2 3 4 5 6 7");
        out
    }
    pub(super) fn count_from(&self, column: i8, row: i8, dc: i8, dr: i8, marker: u8) -> usize {
        let mut count = 0;
        let mut x = column + dc;
        let mut y = row + dr;
        while (0..7).contains(&x)
            && (0..6).contains(&y)
            && self.columns[x as usize][y as usize] == marker
        {
            count += 1;
            x += dc;
            y += dr;
        }
        count
    }
    pub(super) fn has_four(&self, column: usize, row: usize, marker: u8) -> bool {
        const DIRS: [(i8, i8); 4] = [(1, 0), (0, 1), (1, 1), (1, -1)];
        DIRS.into_iter().any(|(dc, dr)| {
            1 + self.count_from(column as i8, row as i8, dc, dr, marker)
                + self.count_from(column as i8, row as i8, -dc, -dr, marker)
                >= 4
        })
    }
    pub(super) fn is_full(&self) -> bool {
        self.heights.iter().all(|height| *height == 6)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(super) struct MemoryMatchState {
    pub(super) pairs: [u8; 8],
    pub(super) matched: u8,
    pub(super) first_reveal: Option<u8>,
    pub(super) last_pair: Option<(u8, u8)>,
    pub(super) attempts: u16,
    pub(super) pairs_found: u8,
}

impl Default for MemoryMatchState {
    fn default() -> Self {
        Self {
            pairs: [0, 1, 2, 3, 0, 1, 2, 3],
            matched: 0,
            first_reveal: None,
            last_pair: None,
            attempts: 0,
            pairs_found: 0,
        }
    }
}

impl MemoryMatchState {
    fn render(&self) -> String {
        let symbols = ['◆', '●', '▲', '■'];
        let mut out = String::new();
        for (index, pair) in self.pairs.iter().enumerate() {
            if index > 0 && index % 4 == 0 {
                out.push('\n');
            }
            let visible = self.matched & (1 << index) != 0
                || self.first_reveal == Some(index as u8)
                || self
                    .last_pair
                    .is_some_and(|(first, second)| first == index as u8 || second == index as u8);
            out.push(if visible {
                symbols[usize::from(*pair)]
            } else {
                '□'
            });
            out.push(' ');
        }
        out
    }
}

pub(super) const MINESWEEPER_SIZE: usize = 8;
pub(super) const MINESWEEPER_MINE_COUNT: usize = 10;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(super) struct MinesweeperState {
    pub(super) board: [[u8; MINESWEEPER_SIZE]; MINESWEEPER_SIZE],
    pub(super) revealed: [[bool; MINESWEEPER_SIZE]; MINESWEEPER_SIZE],
    pub(super) flagged: [[bool; MINESWEEPER_SIZE]; MINESWEEPER_SIZE],
    pub(super) mines_placed: bool,
    pub(super) lost: bool,
    pub(super) won: bool,
    pub(super) revealed_safe: u8,
}

impl Default for MinesweeperState {
    fn default() -> Self {
        Self {
            board: [[0; MINESWEEPER_SIZE]; MINESWEEPER_SIZE],
            revealed: [[false; MINESWEEPER_SIZE]; MINESWEEPER_SIZE],
            flagged: [[false; MINESWEEPER_SIZE]; MINESWEEPER_SIZE],
            mines_placed: false,
            lost: false,
            won: false,
            revealed_safe: 0,
        }
    }
}

impl MinesweeperState {
    fn render(&self) -> String {
        let mut output = String::from("    1 2 3 4 5 6 7 8\n");
        let show_mines = self.lost || self.won;

        for row in 0..MINESWEEPER_SIZE {
            output.push_str(&format!("{:>2} ", row + 1));
            for column in 0..MINESWEEPER_SIZE {
                let value = self.board[row][column];
                let show_mine = show_mines && value == 9;
                let symbol = if show_mine {
                    '✹'
                } else if self.flagged[row][column] && !self.revealed[row][column] {
                    '⚑'
                } else if !self.revealed[row][column] {
                    '□'
                } else if value == 0 {
                    '·'
                } else {
                    char::from_digit(u32::from(value), 10).unwrap_or('?')
                };
                output.push(symbol);
                output.push(' ');
            }
            output.push('\n');
        }

        output
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(super) struct WordScrambleState {
    pub(super) answer: String,
    pub(super) attempts_left: u8,
    #[serde(default)]
    pub(super) guesses: Vec<String>,
}

impl Default for WordScrambleState {
    fn default() -> Self {
        Self {
            answer: String::from("ORBIT"),
            attempts_left: 3,
            guesses: Vec::new(),
        }
    }
}

impl WordScrambleState {
    fn render(&self) -> String {
        let guesses = if self.guesses.is_empty() {
            String::from("none")
        } else {
            self.guesses
                .iter()
                .map(|guess| {
                    if is_valid_word_guess(guess) {
                        guess.as_str()
                    } else {
                        "[invalid guess omitted]"
                    }
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        format!(
            "Scrambled word: BITOR\nAttempts remaining: {}\nPrevious guesses: {}",
            self.attempts_left, guesses
        )
    }
}

const SEQUENCE_RECALL_PATTERNS: [[u8; 8]; 4] = [
    [0, 2, 1, 3, 2, 0, 3, 1],
    [3, 1, 0, 2, 1, 3, 2, 0],
    [1, 3, 2, 0, 3, 1, 0, 2],
    [2, 0, 3, 1, 0, 2, 1, 3],
];

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(super) struct SequenceRecallState {
    pub(super) sequence: [u8; 8],
    pub(super) level: u8,
    pub(super) showing: bool,
}

impl SequenceRecallState {
    pub(super) fn for_session(session_id: &str) -> Self {
        let session_number = session_id
            .rsplit('-')
            .next()
            .and_then(|suffix| suffix.parse::<u64>().ok())
            .unwrap_or(0);
        let pattern_index = (session_number % SEQUENCE_RECALL_PATTERNS.len() as u64) as usize;

        Self {
            sequence: SEQUENCE_RECALL_PATTERNS[pattern_index],
            level: 2,
            showing: true,
        }
    }

    fn render(&self) -> String {
        const SYMBOLS: [char; 4] = ['◆', '●', '▲', '■'];
        let length = usize::from(self.level).min(self.sequence.len());
        let pattern = if self.showing {
            self.sequence[..length]
                .iter()
                .map(|value| {
                    SYMBOLS
                        .get(usize::from(*value))
                        .copied()
                        .unwrap_or('?')
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join(" ")
        } else {
            String::from("hidden")
        };
        let instruction = if self.showing {
            "Memorise the pattern, then use .action <session> hide."
        } else {
            "Pattern hidden. Repeat the full sequence with comma-separated values 1–4."
        };

        format!(
            "Level {}/8\nPattern: {}\n{}",
            self.level, pattern, instruction
        )
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub(super) struct TwentyFortyEightState {
    pub(super) board: [[u16; 4]; 4],
    pub(super) score: u64,
}

impl TwentyFortyEightState {
    fn new() -> Result<Self, GameError> {
        let mut rng = OsRng;
        Self::new_with_rng(&mut rng)
    }

    fn new_with_rng<R: RngCore>(rng: &mut R) -> Result<Self, GameError> {
        let mut state = Self::default();
        state.spawn_tile(rng)?;
        state.spawn_tile(rng)?;
        Ok(state)
    }

    pub(super) fn spawn_tile<R: RngCore>(&mut self, rng: &mut R) -> Result<(), GameError> {
        let empty: Vec<(usize, usize)> = self
            .board
            .iter()
            .enumerate()
            .flat_map(|(row, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .filter_map(move |(column, tile)| (*tile == 0).then_some((row, column)))
            })
            .collect();

        if empty.is_empty() {
            return Ok(());
        }

        let index = random_below(rng, empty.len() as u64)? as usize;
        let (row, column) = empty[index];
        self.board[row][column] = if random_below(rng, 10)? == 0 { 4 } else { 2 };
        Ok(())
    }

    pub(super) fn has_moves(&self) -> bool {
        for row in 0..4 {
            for column in 0..4 {
                let tile = self.board[row][column];
                if tile == 0
                    || (column + 1 < 4 && tile == self.board[row][column + 1])
                    || (row + 1 < 4 && tile == self.board[row + 1][column])
                {
                    return true;
                }
            }
        }
        false
    }

    fn render(&self) -> String {
        let mut output = format!("Score: {}\n\x60\x60\x60text\n", self.score);
        for row in &self.board {
            output.push('|');
            for tile in row {
                let value = if *tile == 0 {
                    String::from("·")
                } else {
                    tile.to_string()
                };
                output.push_str(&format!("{value:>5}|"));
            }
            output.push('\n');
        }
        output.push_str("\x60\x60\x60");
        output
    }
}

pub(super) fn random_below<R: RngCore>(rng: &mut R, upper: u64) -> Result<u64, GameError> {
    if upper == 0 {
        return Err(GameError::InvalidAction);
    }

    let threshold = upper.wrapping_neg() % upper;
    loop {
        let mut bytes = [0_u8; 8];
        rng.try_fill_bytes(&mut bytes)
            .map_err(|_| GameError::RandomnessUnavailable)?;
        let value = u64::from_le_bytes(bytes);
        if value >= threshold {
            return Ok(value % upper);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameSession {
    id: String,
    game: GameKind,
    guild_id: Option<String>,
    players: Vec<String>,
    phase: GamePhase,
    state: GameState,
    winner: Option<String>,
    action_count: u32,
    last_result: String,
}

impl GameSession {
    pub fn create(
        id: String,
        game: GameKind,
        owner_id: String,
        guild_id: Option<String>,
    ) -> Result<Self, GameError> {
        let phase = if game.requires_opponent() {
            GamePhase::WaitingForPlayers
        } else {
            GamePhase::Active
        };
        let state = GameState::new(game, &id)?;
        Ok(Self {
            id,
            game,
            guild_id,
            players: vec![owner_id],
            phase,
            state,
            winner: None,
            action_count: 0,
            last_result: String::from("Session created."),
        })
    }
    pub fn game(&self) -> GameKind {
        self.game
    }

    pub(super) fn id(&self) -> &str {
        &self.id
    }
    pub fn guild_id(&self) -> Option<&str> {
        self.guild_id.as_deref()
    }
    pub fn join(&mut self, player_id: &str, guild_id: Option<&str>) -> Result<(), GameError> {
        if self.phase != GamePhase::WaitingForPlayers {
            return Err(GameError::NotJoinable);
        }
        if self.players.iter().any(|player| player == player_id) {
            return Err(GameError::AlreadyJoined);
        }
        if self.guild_id.as_deref() != guild_id {
            return Err(GameError::WrongGuild);
        }
        self.players.push(player_id.to_owned());
        self.phase = GamePhase::Active;
        self.last_result = String::from("The lobby is full. The game is ready.");
        Ok(())
    }
    pub fn act(&mut self, player_id: &str, action: GameAction) -> Result<(), GameError> {
        if self.phase != GamePhase::Active {
            return Err(GameError::NotActive);
        }
        let player_index = self
            .players
            .iter()
            .position(|player| player == player_id)
            .ok_or(GameError::NotParticipant)?;
        if self.state.current_player_index() != player_index {
            return Err(GameError::NotYourTurn);
        }
        let result = apply_action(&mut self.state, player_index, action)?;
        self.action_count = self.action_count.saturating_add(1);
        self.last_result = result.message.to_owned();
        if let Some(winner_index) = result.winner_index {
            self.winner = self.players.get(winner_index).cloned();
            self.phase = GamePhase::Completed;
        } else if result.is_draw || result.is_complete {
            self.phase = GamePhase::Completed;
        }
        Ok(())
    }
    pub fn leave(&mut self, player_id: &str) -> Result<(), GameError> {
        if !self.players.iter().any(|player| player == player_id) {
            return Err(GameError::NotParticipant);
        }
        if matches!(self.phase, GamePhase::Completed | GamePhase::Abandoned) {
            return Err(GameError::AlreadyFinished);
        }
        self.phase = GamePhase::Abandoned;
        self.last_result = String::from("A participant ended the session.");
        Ok(())
    }
    pub fn snapshot(&self) -> SessionSnapshot {
        SessionSnapshot {
            id: self.id.clone(),
            game: self.game,
            guild_id: self.guild_id.clone(),
            players: self.players.clone(),
            phase: self.phase,
            winner: self.winner.clone(),
            action_count: self.action_count,
            last_result: self.last_result.clone(),
            board: self.state.render(),
            controls: self.game.definition().controls,
        }
    }
    pub fn is_visible_to(&self, player_id: &str, guild_id: Option<&str>) -> bool {
        self.players.iter().any(|player| player == player_id)
            && self.guild_id.as_deref() == guild_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionSnapshot {
    pub id: String,
    pub game: GameKind,
    pub guild_id: Option<String>,
    pub players: Vec<String>,
    pub phase: GamePhase,
    pub winner: Option<String>,
    pub action_count: u32,
    pub last_result: String,
    pub board: String,
    pub controls: &'static str,
}

impl SessionSnapshot {
    pub fn render(&self) -> String {
        let players = self
            .players
            .iter()
            .enumerate()
            .map(|(i, player)| format!("P{}: {}", i + 1, player))
            .collect::<Vec<_>>()
            .join(" · ");
        let winner = self
            .winner
            .as_deref()
            .map(|value| format!("\nWinner: {value}"))
            .unwrap_or_default();
        format!(
            "**{}** · {} · {}\nPlayers: {}{}\nActions: {}\n{}\n{}\n{}",
            self.game.definition().name,
            self.id,
            self.phase,
            if players.is_empty() { "none" } else { &players },
            winner,
            self.action_count,
            self.last_result,
            self.board,
            self.controls
        )
    }
}

impl fmt::Display for GameKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.definition().id)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ConnectFourState, GameKind, GameSession, MinesweeperState, TwentyFortyEightState,
        WordScrambleState,
    };
    use crate::game::engine::{GameAction, GameError};

    #[test]
    fn twenty_forty_eight_sessions_start_with_two_valid_tiles() {
        let mut rng = rand::rngs::mock::StepRng::new(1, 1);
        let state = TwentyFortyEightState::new_with_rng(&mut rng).expect("create 2048 state");
        let tiles: Vec<u16> = state
            .board
            .iter()
            .flatten()
            .copied()
            .filter(|tile| *tile != 0)
            .collect();
        assert_eq!(tiles.len(), 2);
        assert!(tiles.iter().all(|tile| *tile == 2 || *tile == 4));
        assert!(state.has_moves());
    }

    #[test]
    fn connect_four_render_preserves_column_order_and_bottom_row() {
        let mut state = ConnectFourState::default();
        state.columns[0][0] = 1;
        state.columns[1][0] = 2;

        let expected = [
            "· · · · · · ·",
            "· · · · · · ·",
            "· · · · · · ·",
            "· · · · · · ·",
            "· · · · · · ·",
            "X O · · · · ·",
            "1 2 3 4 5 6 7",
        ]
        .join("\n");

        assert_eq!(state.render(), expected);
    }

    #[test]
    fn legacy_word_scramble_snapshots_default_guess_history() {
        let state: WordScrambleState =
            serde_json::from_str(r#"{"answer":"ORBIT","attempts_left":2}"#)
                .expect("legacy session snapshot");
        assert!(state.guesses.is_empty());
    }

    #[test]
    fn legacy_unsafe_guesses_are_not_replayed_into_discord_messages() {
        let state: WordScrambleState = serde_json::from_str(
            r#"{"answer":"ORBIT","attempts_left":2,"guesses":["@everyone","PLANET"]}"#,
        )
        .expect("legacy session snapshot");

        let rendered = state.render();
        assert!(!rendered.contains("@everyone"));
        assert!(rendered.contains("[invalid guess omitted]"));
        assert!(rendered.contains("PLANET"));
    }

    #[test]
    fn duplicate_word_scramble_guess_does_not_advance_session() {
        let mut session = GameSession::create(
            String::from("MB-TEST"),
            GameKind::WordScramble,
            String::from("player"),
            None,
        )
        .expect("create session");
        session
            .act(
                "player",
                GameAction::WordAnswer {
                    answer: "planet".into(),
                },
            )
            .expect("first guess");
        let before = session.snapshot();

        assert_eq!(
            session.act(
                "player",
                GameAction::WordAnswer {
                    answer: "PLANET".into(),
                },
            ),
            Err(GameError::AlreadyGuessed)
        );
        assert_eq!(session.snapshot(), before);
    }

    #[test]
    fn minesweeper_session_state_survives_serialization() {
        let mut session = GameSession::create(
            String::from("MB-MINES"),
            GameKind::Minesweeper,
            String::from("player"),
            None,
        )
        .expect("create Minesweeper session");
        session
            .act("player", GameAction::MinesweeperFlag { row: 7, column: 7 })
            .expect("flag a cell");
        session
            .act(
                "player",
                GameAction::MinesweeperReveal { row: 0, column: 0 },
            )
            .expect("place mines and reveal the first safe cell");

        let encoded = serde_json::to_string(&session).expect("serialize session");
        let restored: GameSession = serde_json::from_str(&encoded).expect("restore session");

        assert_eq!(restored.snapshot(), session.snapshot());
        assert_eq!(
            serde_json::to_string(&restored).expect("serialize restored session"),
            encoded
        );
        assert_eq!(GameKind::parse("mines"), Ok(GameKind::Minesweeper));
    }

    #[test]
    fn minesweeper_render_hides_mines_until_the_session_ends() {
        let mut state = MinesweeperState::default();
        state.mines_placed = true;
        state.board[0][0] = 9;
        state.flagged[0][0] = true;

        let active = state.render();
        assert_eq!(active.lines().count(), 9);
        assert!(active.contains('⚑'));
        assert!(!active.contains('✹'));

        state.lost = true;
        let ended = state.render();
        assert!(ended.contains('✹'));
        assert!(!ended.contains('⚑'));
    }

    #[test]
    fn sequence_recall_hides_and_progresses_through_session_snapshots() {
        let mut session = GameSession::create(
            String::from("MB-000000"),
            GameKind::SequenceRecall,
            String::from("player"),
            None,
        )
        .expect("create session");
        let alternate = GameSession::create(
            String::from("MB-000001"),
            GameKind::SequenceRecall,
            String::from("other-player"),
            None,
        )
        .expect("create session");
        assert_ne!(session.snapshot().board, alternate.snapshot().board);
        assert!(session.snapshot().board.contains("Pattern: ◆ ▲"));

        assert_eq!(
            session.act("player", GameAction::SequenceAnswer { values: vec![0, 2] }),
            Err(GameError::InvalidAction)
        );
        session
            .act("player", GameAction::SequenceHide)
            .expect("hide the pattern");
        let hidden = session.snapshot();
        assert!(hidden.board.contains("Pattern: hidden"));
        assert!(!hidden.board.contains("◆"));

        session
            .act("player", GameAction::SequenceAnswer { values: vec![0, 2] })
            .expect("repeat the correct sequence");
        let next_level = session.snapshot();
        assert!(next_level.board.contains("Level 3/8"));
        assert!(next_level.board.contains("Pattern: ◆ ▲ ●"));

        session
            .act("player", GameAction::SequenceHide)
            .expect("hide the longer pattern");
        session
            .act(
                "player",
                GameAction::SequenceAnswer {
                    values: vec![3, 3, 3],
                },
            )
            .expect("an incorrect sequence completes the session");
        let finished = session.snapshot();
        assert_eq!(finished.phase, super::GamePhase::Completed);
        assert!(finished.board.contains("Pattern: ◆ ▲ ●"));
    }
}
