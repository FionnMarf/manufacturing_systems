use std::mem;

fn main() {
    // create a markov chain for testing
    let mut markov_chain = MarkovChain::new();
    markov_chain.add_state("A".to_string());
    markov_chain.add_state("B".to_string());
    markov_chain.add_state("C".to_string());

    let state_a = markov_chain.get_state("A".to_string()).unwrap();
    let state_b = markov_chain.get_state("B".to_string()).unwrap();
    let state_c = markov_chain.get_state("C".to_string()).unwrap();

    markov_chain.add_transition("A to B".to_string(), 0.5, state_b);


}

pub struct MarkovChain {
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
}

pub struct State {
    pub name: String,
    pub transitions: Vec<Transition>,
}

pub struct Transition {
    pub name: String,
    pub probability: f64,
    pub next_state: &State,
}

impl MarkovChain {
    pub fn new() -> MarkovChain {
        MarkovChain {
            states: Vec::new(),
            transitions: Vec::new(),
        }
    }

    pub fn add_state(&mut self, name: String) {
        self.states.push(State {
            name: name,
            transitions: Vec::new(),
        });
    }

    pub fn get_state(&mut self, name: String) -> Option<&State> {
        for state in &self.states {
            if state.name == name {
                return Some(state);
            }
        }
        None
    }

    pub fn add_transition(&mut self, name: String, probability: f64, next_state: &State) {
        self.transitions.push(Transition {
            name: name,
            probability: probability,
            next_state: next_state,
        });
    }
}