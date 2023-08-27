fn main() {
    println!("Hello, world!");
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