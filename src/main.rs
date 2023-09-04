use rand::*;
use std::collections::HashSet;

mod markov;
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
    // test the successors function
    let mut successors = machine_1.successors(state_0);
    let matrix = markov::random_transition_matrix(&mut machine_1);
    // generate a new markov machine using matrix
    markov_machine!(machine_2, 3, matrix);
    // test the generate_transition_matrix function
    let matrix_2 = markov::generate_transition_matrix(&machine_2);
    // print the matrix
    println!("{:?}", matrix_2);
    let random_number = rand::random::<f64>();
    println!("{}", random_number);
}
