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

mod validation {
    use super::*;

    #[test]
    fn rejects_invalid_start_state() {
        let config = Config {
            states: vec!["q0".into(), "q1".into()],
            alphabet: vec!['0', '1'],
            transitions: transitions(&[
                ("q0", &[('0', "q1"), ('1', "q0")]),
                ("q1", &[('0', "q1"), ('1', "q0")]),
            ]),
            start_state: "q2".into(), // Invalid!
            accepting_states: vec!["q1".into()],
        };

        let result = DFA::new(config);
        assert!(matches!(result, Err(ValidationError::InvalidStartState(_))));
    }

    #[test]
    fn rejects_invalid_accepting_state() {
        let config = Config {
            states: vec!["q0".into(), "q1".into()],
            alphabet: vec!['0', '1'],
            transitions: transitions(&[
                ("q0", &[('0', "q1"), ('1', "q0")]),
                ("q1", &[('0', "q1"), ('1', "q0")]),
            ]),
            start_state: "q0".into(),
            accepting_states: vec!["q2".into()], // Invalid!
        };

        let result = DFA::new(config);
        assert!(matches!(result, Err(ValidationError::InvalidAcceptingState(_))));
    }

    #[test]
    fn rejects_missing_state_transitions() {
        let config = Config {
            states: vec!["q0".into(), "q1".into()],
            alphabet: vec!['0', '1'],
            transitions: transitions(&[
                ("q0", &[('0', "q1"), ('1', "q0")]),
                // q1 missing!
            ]),
            start_state: "q0".into(),
            accepting_states: vec!["q1".into()],
        };

        let result = DFA::new(config);
        assert!(matches!(result, Err(ValidationError::MissingStateTransitions(_))));
    }

    #[test]
    fn rejects_missing_transition_for_symbol() {
        let config = Config {
            states: vec!["q0".into(), "q1".into()],
            alphabet: vec!['0', '1'],
            transitions: transitions(&[
                ("q0", &[('0', "q1")]), // Missing '1'!
                ("q1", &[('0', "q1"), ('1', "q0")]),
            ]),
            start_state: "q0".into(),
            accepting_states: vec!["q1".into()],
        };

        let result = DFA::new(config);
        assert!(matches!(result, Err(ValidationError::MissingTransition { .. })));
    }

    #[test]
    fn rejects_invalid_transition_target() {
        let config = Config {
            states: vec!["q0".into(), "q1".into()],
            alphabet: vec!['0', '1'],
            transitions: transitions(&[
                ("q0", &[('0', "q2"), ('1', "q0")]), // q2 doesn't exist!
                ("q1", &[('0', "q1"), ('1', "q0")]),
            ]),
            start_state: "q0".into(),
            accepting_states: vec!["q1".into()],
        };

        let result = DFA::new(config);
        assert!(matches!(result, Err(ValidationError::InvalidTransitionTarget { .. })));
    }

    #[test]
    fn accepts_valid_config() {
        let config = Config {
            states: vec!["q0".into(), "q1".into()],
            alphabet: vec!['0', '1'],
            transitions: transitions(&[
                ("q0", &[('0', "q1"), ('1', "q0")]),
                ("q1", &[('0', "q1"), ('1', "q0")]),
            ]),
            start_state: "q0".into(),
            accepting_states: vec!["q1".into()],
        };

        let result = DFA::new(config);
        assert!(result.is_ok());
    }
}

mod binary_division_by_2 {
    use super::*;

    fn even_binary_dfa() -> DFA {
        DFA::new(Config {
            states: vec!["q0".into(), "q1".into()],
            alphabet: vec!['0', '1'],
            transitions: transitions(&[
                ("q0", &[('0', "q1"), ('1', "q0")]),
                ("q1", &[('0', "q1"), ('1', "q0")]),
            ]),
            start_state: "q0".into(),
            accepting_states: vec!["q1".into()],
        })
        .unwrap()
    }

    #[test]
    fn accepts_0() {
        assert!(even_binary_dfa().run("0"));
    }

    #[test]
    fn accepts_10() {
        assert!(even_binary_dfa().run("10"));
    }

    #[test]
    fn accepts_1010() {
        assert!(even_binary_dfa().run("1010"));
    }

    #[test]
    fn rejects_1() {
        assert!(!even_binary_dfa().run("1"));
    }

    #[test]
    fn rejects_11() {
        assert!(!even_binary_dfa().run("11"));
    }

    #[test]
    fn rejects_1011() {
        assert!(!even_binary_dfa().run("1011"));
    }

    #[test]
    fn rejects_empty_string() {
        assert!(!even_binary_dfa().run(""));
    }

    #[test]
    fn rejects_invalid_symbol() {
        assert!(!even_binary_dfa().run("102"));
    }
}

mod contains_ab {
    use super::*;

    fn contains_ab_dfa() -> DFA {
        DFA::new(Config {
            states: vec!["q0".into(), "q1".into(), "q2".into()],
            alphabet: vec!['a', 'b'],
            transitions: transitions(&[
                ("q0", &[('a', "q1"), ('b', "q0")]),
                ("q1", &[('a', "q1"), ('b', "q2")]),
                ("q2", &[('a', "q2"), ('b', "q2")]),
            ]),
            start_state: "q0".into(),
            accepting_states: vec!["q2".into()],
        })
        .unwrap()
    }

    #[test]
    fn accepts_ab() {
        assert!(contains_ab_dfa().run("ab"));
    }

    #[test]
    fn accepts_aab() {
        assert!(contains_ab_dfa().run("aab"));
    }

    #[test]
    fn accepts_bab() {
        assert!(contains_ab_dfa().run("bab"));
    }

    #[test]
    fn accepts_abbb() {
        assert!(contains_ab_dfa().run("abbb"));
    }

    #[test]
    fn rejects_a() {
        assert!(!contains_ab_dfa().run("a"));
    }

    #[test]
    fn rejects_ba() {
        assert!(!contains_ab_dfa().run("ba"));
    }

    #[test]
    fn rejects_bbb() {
        assert!(!contains_ab_dfa().run("bbb"));
    }

    #[test]
    fn rejects_empty() {
        assert!(!contains_ab_dfa().run(""));
    }
}

mod execution_trace {
    use super::*;

    fn dfa() -> DFA {
        DFA::new(Config {
            states: vec!["q0".into(), "q1".into()],
            alphabet: vec!['0', '1'],
            transitions: transitions(&[
                ("q0", &[('0', "q1"), ('1', "q0")]),
                ("q1", &[('0', "q1"), ('1', "q0")]),
            ]),
            start_state: "q0".into(),
            accepting_states: vec!["q1".into()],
        })
        .unwrap()
    }

    #[test]
    fn trace_for_accepted_input() {
        let trace = dfa().run_with_trace("10");

        assert_eq!(trace.input, "10");
        assert_eq!(trace.start_state, "q0");
        assert_eq!(trace.final_state, "q1");
        assert!(trace.result);
        assert_eq!(trace.steps.len(), 2);
        assert_eq!(trace.steps[0].from_state, "q0");
        assert_eq!(trace.steps[0].symbol, '1');
        assert_eq!(trace.steps[0].to_state, "q0");
        assert_eq!(trace.steps[1].from_state, "q0");
        assert_eq!(trace.steps[1].symbol, '0');
        assert_eq!(trace.steps[1].to_state, "q1");
    }

    #[test]
    fn trace_for_rejected_input() {
        let trace = dfa().run_with_trace("11");

        assert_eq!(trace.final_state, "q0");
        assert!(!trace.result);
    }

    #[test]
    fn trace_for_empty_input() {
        let trace = dfa().run_with_trace("");

        assert!(trace.steps.is_empty());
        assert_eq!(trace.final_state, "q0");
        assert!(!trace.result);
    }
}
