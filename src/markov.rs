// Rewriting in line with 
// http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

pub type StateIndex = usize;
pub type TransitionIndex = usize;

pub struct MarkovChain {
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
}

pub struct State {
    first_outgoing_transition: Option<TransitionIndex>,
}

pub struct Transition {
    target: StateIndex,
    probability: f64,
    next_outgoing_transition: Option<TransitionIndex>,
}

pub struct Successors<'markov_chain> {
    markov_chain: &'markov_chain MarkovChain,
    current_transition_index: Option<TransitionIndex>,
}

impl MarkovChain {
    pub fn add_state(&mut self) -> StateIndex {
        let state_index = self.states.len();
        self.states.push(State { first_outgoing_transition: None });
        state_index
    }

    pub fn add_transition(&mut self, source: StateIndex, target: StateIndex, probability: f64) {
        let transition_index = self.transitions.len();
        let state_data = &mut self.states[source];
        self.transitions.push(Transition {
            target: target,
            probability: probability,
            next_outgoing_transition: state_data.first_outgoing_transition,
        });
        state_data.first_outgoing_transition = Some(transition_index);
    }

    pub fn successors(&self, source: StateIndex) -> Successors {
        let first_outgoing_transition = self.states[source].first_outgoing_transition;
        Successors {
            markov_chain: self,
            current_transition_index: first_outgoing_transition,
        }
    }
}

impl<'markov_chain> Iterator for Successors<'markov_chain> {
    type Item = StateIndex;

    fn next(&mut self) -> Option<StateIndex> {
        match self.current_transition_index {
            None => None,
            Some(transition_index) => {
                let transition = &self.markov_chain.transitions[transition_index];
                self.current_transition_index = transition.next_outgoing_transition;
                Some(transition.target)
            }
        }
    }
}
