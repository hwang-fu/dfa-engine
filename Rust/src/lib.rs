//! DFA Engine - A Deterministic Finite Automaton implementation

pub mod dfa;
pub mod error;
pub mod types;

pub use dfa::DFA;
pub use error::ValidationError;
pub use types::{Config, ExecutionStep, ExecutionTrace};
