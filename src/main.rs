use rand::*;
use std::collections::HashSet;
use crate::markov::{MarkovChain};
use crate::transfer_lines::TransferLine;

mod markov;
mod transfer_lines;
mod queue;
mod machine;
// test again
fn main() {
    // create a markov chain for testing
    let mut machine_1 = markov::MarkovChain {
        states: Vec::new(),
        transitions: Vec::new(),
    };
    // add some states
    let state_0 = machine_1.add_state("Idle".to_string());
    let state_1 = machine_1.add_state("Working".to_string());
    let state_2 = machine_1.add_state("Broken".to_string());
    // add some transitions
    machine_1.add_transition(state_0, state_1, 0.5);
    machine_1.add_transition(state_0, state_2, 0.5);
    machine_1.add_transition(state_1, state_2, 0.5);
    machine_1.add_transition(state_1, state_0, 0.5);
    machine_1.add_transition(state_2, state_0, 0.5);
    machine_1.add_transition(state_2, state_1, 0.5);

    let state_0 = machine_1.get_state_name(0);
    println!("State 0: {}", state_0);

    MarkovChain::step_chain(&mut machine_1);
    let current_state = machine_1.get_current_state_name();
    println!("Current state: {}", current_state);

    let processing_times = vec![3.0, 2.0, 1.0];
    let capacities = vec![3, 3];
    let throughputs = vec![None, None];
    let mut transfer_line = TransferLine::new(processing_times, capacities, throughputs);
}