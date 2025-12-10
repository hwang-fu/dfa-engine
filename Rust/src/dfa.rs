//! DFA implementation with validation.

use std::collections::{HashMap, HashSet};

use crate::error::ValidationError;
use crate::types::{Config, ExecutionStep, ExecutionTrace};

#[derive(Debug, Clone)]
pub struct DFA {
    states: HashSet<String>,
    alphabet: HashSet<char>,
    transitions: HashMap<String, HashMap<char, String>>,
    start_state: String,
    accepting_states: HashSet<String>,
}

impl DFA {
    pub fn new(config: Config) -> Result<Self, ValidationError> {
        let dfa = Self {
            states: config.states.into_iter().collect(),
            alphabet: config.alphabet.into_iter().collect(),
            transitions: config.transitions,
            start_state: config.start_state,
            accepting_states: config.accepting_states.into_iter().collect(),
        };

        dfa.validate()?;
        Ok(dfa)
    }

    fn validate(&self) -> Result<(), ValidationError> {
        if !self.states.contains(&self.start_state) {
            return Err(ValidationError::InvalidStartState(self.start_state.clone()))
        }

        for state in &self.accepting_states {
            if !self.states.contains(state) {
                return Err(ValidationError::InvalidAcceptingState(state.clone()));
            }
        }

        for state in &self.states {
            let Some(state_transitions) = self.transitions.get(state) else {
                return Err(ValidationError::MissingStateTransitions(state.clone()));
            };

            for symbol in &self.alphabet {
                let Some(target) = state_transitions.get(symbol) else {
                    return Err(ValidationError::MissingTransition {
                        state: state.clone(),
                        symbol: *symbol,
                    });
                };

                if !self.states.contains(target) {
                    return Err(ValidationError::InvalidTransitionTarget {
                        from: state.clone(),
                        symbol: *symbol,
                        to: target.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    pub fn run(&self, input: &str) -> bool {
        let mut current_state = &self.start_state;

        for symbol in input.chars() {
            if !self.alphabet.contains(&symbol) {
                return false;
            }

            let Some(state_transitions) = self.transitions.get(current_state) else {
                return false;
            };

            let Some(next_state) = state_transitions.get(&symbol) else {
                return false;
            };

            current_state = next_state;
        }

        self.accepting_states.contains(current_state)
    }

    pub fn run_with_trace(&self, input: &str) -> ExecutionTrace {
        let mut steps = Vec::new();
        let mut current_state = &self.start_state;

        for symbol in input.chars() {
            if !self.alphabet.contains(&symbol) {
                return ExecutionTrace {
                    input: input.to_string(),
                    start_state: self.start_state.clone(),
                    steps,
                    final_state: current_state.clone(),
                    result: false,
                };
            }

            let Some(state_transitions) = self.transitions.get(current_state) else {
                return ExecutionTrace {
                    input: input.to_string(),
                    start_state: self.start_state.clone(),
                    steps,
                    final_state: current_state.clone(),
                    result: false,
                };
            };

            let Some(next_state) = state_transitions.get(&symbol) else {
                return ExecutionTrace {
                    input: input.to_string(),
                    start_state: self.start_state.clone(),
                    steps,
                    final_state: current_state.clone(),
                    result: false,
                };
            };

            steps.push(ExecutionStep { from_state: current_state.clone(), to_state: next_state.clone(), symbol, });

            current_state = next_state;
        }

        ExecutionTrace {
            input: input.to_string(),
            start_state: self.start_state.clone(),
            steps,
            final_state: current_state.clone(),
            result: self.accepting_states.contains(current_state),
        }
    }
}

