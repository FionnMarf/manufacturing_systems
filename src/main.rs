mod markov;

fn main() {
    // create a markov chain for testing
    let mut markov_chain = markov::MarkovChain {
        states: Vec::new(),
        transitions: Vec::new(),
    };
    // add some states
    let state_0 = markov_chain.add_state();
    let state_1 = markov_chain.add_state();
    let state_2 = markov_chain.add_state();
    // add some transitions
    markov_chain.add_transition(state_0, state_1, 0.5);
    markov_chain.add_transition(state_0, state_2, 0.5);
    markov_chain.add_transition(state_1, state_2, 0.5);
    markov_chain.add_transition(state_1, state_0, 0.5);
    markov_chain.add_transition(state_2, state_0, 0.5);
    markov_chain.add_transition(state_2, state_1, 0.5);
    // test the successors function
    let mut successors = markov_chain.successors(state_0);
}

