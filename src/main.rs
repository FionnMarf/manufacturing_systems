fn main() {
    // create a markov chain for testing
    let mut markov_chain = markov_chain::new();
    markov_chain.add_state("A".to_string());
    markov_chain.add_state("B".to_string());
    markov_chain.add_state("C".to_string());

    // now add transitions
    markov_chain.add_transition("A->B".to_string(), 0.5, markov_chain.get_state("B".to_string()).unwrap());
    markov_chain.add_transition("A->C".to_string(), 0.5, markov_chain.get_state("C".to_string()).unwrap());
    markov_chain.add_transition("B->A".to_string(), 0.5, markov_chain.get_state("A".to_string()).unwrap());
    markov_chain.add_transition("B->C".to_string(), 0.5, markov_chain.get_state("C".to_string()).unwrap());
    markov_chain.add_transition("C->A".to_string(), 0.5, markov_chain.get_state("A".to_string()).unwrap());
    markov_chain.add_transition("C->B".to_string(), 0.5, markov_chain.get_state("B".to_string()).unwrap());

}

pub struct markov_chain {
    pub states: Vec<state>,
    pub transitions: Vec<transition>,
}

pub struct state {
    pub name: String,
    pub transitions: Vec<transition>,
}

pub struct transition {
    pub name: String,
    pub probability: f64,
    pub next_state: state,
}

impl markov_chain {
    pub fn new() -> markov_chain {
        markov_chain {
            states: Vec::new(),
            transitions: Vec::new(),
        }
    }

    pub fn add_state(&mut self, name: String) {
        self.states.push(state {
            name: name,
            transitions: Vec::new(),
        });
    }

    pub fn add_transition(&mut self, name: String, probability: f64, next_state: state) {
        self.transitions.push(transition {
            name: name,
            probability: probability,
            next_state: next_state,
        });
    }

    pub fn get_state(&self, name: String) -> Option<&state> {
        for state in &self.states {
            if state.name == name {
                return Some(state);
            }
        }
        None
    }

    pub fn get_transition(&self, name: String) -> Option<&transition> {
        for transition in &self.transitions {
            if transition.name == name {
                return Some(transition);
            }
        }
        None
    }
}

// creating a macro to generate a new state
macro_rules! new_state {
    ($name:expr) => {
        state {
            name: $name,
            transitions: Vec::new(),
        }
    };
}

// creating a macro to generate a new transition
macro_rules! new_transition {
    ($name:expr, $probability:expr, $next_state:expr) => {
        transition {
            name: $name,
            probability: $probability,
            next_state: $next_state,
        }
    };
}