//! Error types for DFA validation.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    InvalidStartState(String),
    InvalidAcceptingState(String),
    InvalidTransitionTarget { from: String, symbol: char, to: String, },
    MissingTransition { state: String, symbol: char, },
    MissingStateTransitions(String),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidStartState(state) => {
                write!(f, "Start state \"{state}\" is not in the set of states")
            }
            Self::InvalidAcceptingState(state) => {
                write!(f, "Accepting state \"{state}\" is not in the set of states")
            }
            Self::InvalidTransitionTarget { from, symbol, to } => {
                write!(f, "Transition from \"{from}\" on '{symbol}' leads to unknown state \"{to}\"")
            }
            Self::MissingTransition { state, symbol } => {
                write!(f, "Missing transition for state \"{state}\" on symbol '{symbol}'")
            }
            Self::MissingStateTransitions(state) => {
                write!(f, "State \"{state}\" has no transitions defined")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

