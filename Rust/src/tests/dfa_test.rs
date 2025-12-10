use std::collections::HashMap;

use dfa_engine::{Config, ValidationError, DFA};

fn transitions(data: &[(&str, &[(char, &str)])]) -> HashMap<String, HashMap<char, String>> {
    data.iter()
        .map(|(state, trans)| {
            let inner: HashMap<char, String> = trans
                .iter()
                .map(|(symbol, target)| (*symbol, (*target).to_string()))
                .collect();
            ((*state).to_string(), inner)
        })
        .collect()
}


