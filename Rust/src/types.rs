//! Core types for DFA definition and execution.
//!
//! DFA = (Q, Σ, δ, q₀, F)
//! - Q: Set of states
//! - Σ: Input alphabet
//! - δ: Transition function Q × Σ → Q
//! - q₀: Start state
//! - F: Set of accepting states

use std::collections::HashMap;

/// The configuration to construct a DFA.
#[derive(Debug, Clone)]
pub struct Config {
    // Q: Set of states
    pub states: Vec<String>,

    // Σ: Input alphabet
    pub alphabet: Vec<char>,

    // δ: Transition function Q × Σ → Q
    pub transitions: HashMap<String, HashMap<char, String>>,

    // q₀: Start state
    pub start_state: String,

    // F: Set of accepting states
    pub accepting_states: Vec<String>,
}

/// A single step in the execution trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionStep {
    pub from_state: String,
    pub to_state: String,
    pub symbol: char,
}

/// The full execution trace for debugging.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionTrace {
    pub input: String,
    pub start_state: String,
    pub steps: Vec<ExecutionStep>,
    pub final_state: String,
    pub result: bool,
}
