use std::{collections::HashMap, sync::RwLock};

use petgraph::visit::EdgeRef;

use crate::error::NfaError;

use super::{Nfa, State};

pub struct Dfa {
    nfa: Nfa,
    next_state_cache: RwLock<HashMap<State, Vec<State>>>,
}

impl Dfa {
    pub fn from_str(expr: &str) -> Result<Dfa, NfaError> {
        let dfa = Self {
            nfa: Nfa::from_str(expr)?,
            next_state_cache: RwLock::new(HashMap::new()),
        };
        Ok(dfa)
    }
}

impl Dfa {
    pub fn to_json(&self) -> String {
        self.nfa.to_json()
    }
}

impl Dfa {
    fn get_next_states(&self, cur: State) -> Vec<State> {
        let next_state_cache = self.next_state_cache.read().unwrap();
        if let Some(states) = next_state_cache.get(&cur) {
            return states.clone();
        }
        let states = self.nfa.get_next_states(cur);

        let mut next_state_cache = self.next_state_cache.write().unwrap();
        next_state_cache.insert(cur, states.clone());
        states
    }

    pub fn test(&self, str: &str) -> bool {
        let nfa = &self.nfa;
        let mut current_states = self.get_next_states(nfa.initial_state);
        for c in str.chars() {
            let mut next_states = Vec::new();
            for state in current_states {
                for edge in nfa.graph.edges(state) {
                    if let Some(c_) = edge.weight() {
                        if c == *c_ {
                            next_states.extend(self.get_next_states(edge.target()));
                        }
                    }
                }
            }
            current_states = next_states;
        }
        current_states
            .iter()
            .any(|&s| nfa.accepted_states.contains(&s))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_test() {
        let nfa = Nfa::from_str("abb.+.a.").unwrap();
        assert!(nfa.test("abba"));
        assert!(nfa.test("abbbbbbbba"));
        assert!(!nfa.test("abbb"));
        assert!(!nfa.test("ab"));

        let nfa = Nfa::from_str("abab...abbb...|").unwrap();
        assert!(nfa.test("abab"));
        assert!(nfa.test("abbb"));
    }
}
