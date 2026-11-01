use std::fmt;

use serde::{Deserialize, Serialize};

use super::engine::{GameAction, GameError, apply_action};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameKind {
    TicTacToe,
    ConnectFour,
    MemoryMatch,
    WordScramble,
}

impl GameKind {
    pub const ALL: [Self; 4] = [
        Self::TicTacToe,
        Self::ConnectFour,
        Self::MemoryMatch,
        Self::WordScramble,
    ];
    pub const fn definition(self) -> GameDefinition {
        match self {
            Self::TicTacToe => GameDefinition {
                id: "tic-tac-toe",
                name: "Tic-tac-toe",
                description: "A two-player strategy game on a 3×3 board.",
                players: "2 players",
                controls: "Use .action <session> <row>,<column> with 1-based coordinates.",
            },
            Self::ConnectFour => GameDefinition {
                id: "connect-four",
                name: "Connect Four",
                description: "Drop discs into seven columns and connect four in a row.",
                players: "2 players",
                controls: "Use .action <session> <column> with a column from 1 to 7.",
            },
            Self::MemoryMatch => GameDefinition {
                id: "memory-match",
                name: "Memory Match",
                description: "Find four pairs by remembering where each symbol appears.",
                players: "1 player",
                controls: "Use .action <session> <position> with a position from 1 to 8.",
            },
            Self::WordScramble => GameDefinition {
                id: "word-scramble",
                name: "Word Scramble",
                description: "Reconstruct an original five-letter word from its scrambled letters.",
                players: "1 player",
                controls: "Use .action <session> <answer> to submit your guess.",
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
            "word-scramble" | "scramble" => Ok(Self::WordScramble),
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
    WordScramble(WordScrambleState),
}

impl GameState {
    fn new(kind: GameKind) -> Self {
        match kind {
            GameKind::TicTacToe => Self::TicTacToe(TicTacToeState::default()),
            GameKind::ConnectFour => Self::ConnectFour(ConnectFourState::default()),
            GameKind::MemoryMatch => Self::MemoryMatch(MemoryMatchState::default()),
            GameKind::WordScramble => Self::WordScramble(WordScrambleState::default()),
        }
    }
    pub(super) fn current_player_index(&self) -> usize {
        match self {
            Self::TicTacToe(s) => usize::from(s.current_player),
            Self::ConnectFour(s) => usize::from(s.current_player),
            Self::MemoryMatch(_) | Self::WordScramble(_) => 0,
        }
    }
    fn render(&self) -> String {
        match self {
            Self::TicTacToe(s) => s.render(),
            Self::ConnectFour(s) => s.render(),
            Self::MemoryMatch(s) => s.render(),
            Self::WordScramble(s) => s.render(),
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
            for col in 0..7 {
                if col > 0 {
                    out.push(' ');
                }
                out.push(match self.columns[col][row] {
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(super) struct WordScrambleState {
    pub(super) answer: String,
    pub(super) attempts_left: u8,
}

impl Default for WordScrambleState {
    fn default() -> Self {
        Self {
            answer: String::from("ORBIT"),
            attempts_left: 3,
        }
    }
}

impl WordScrambleState {
    fn render(&self) -> String {
        format!(
            "Scrambled word: BITOR\nAttempts remaining: {}",
            self.attempts_left
        )
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
    pub fn create(id: String, game: GameKind, owner_id: String, guild_id: Option<String>) -> Self {
        let phase = if game.requires_opponent() {
            GamePhase::WaitingForPlayers
        } else {
            GamePhase::Active
        };
        Self {
            id,
            game,
            guild_id,
            players: vec![owner_id],
            phase,
            state: GameState::new(game),
            winner: None,
            action_count: 0,
            last_result: String::from("Session created."),
        }
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
