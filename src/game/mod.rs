//! Authoritative, transport-independent game rules and PostgreSQL-backed session runtime.
//!
//! Discord handlers call this module; game state never depends on Serenity types.

mod engine;
mod model;
mod runtime;

pub use engine::{GameAction, GameError};
pub use model::{GameDefinition, GameKind, GamePhase, GameSession, SessionSnapshot};
pub use runtime::{GameRuntime, GameRuntimeError};
