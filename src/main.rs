use rand::*;

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
}

// this macro generates a markov machine called $name with n states and a transition matrix
// $name: the name of the machine
// $n: the number of states
// $matrix: the transition matrix
macro_rules! markov_machine {
    ($name:ident, $n:expr, $matrix:expr) => {
        let mut $name = markov::MarkovChain {
            states: Vec::new(),
            transitions: Vec::new(),
        };
        // add states
        for i in 0..$n {
            $name.add_state(format!("State {}", i));
        }
        // add transitions
        for i in 0..$n {
            for j in 0..$n {
                $name.add_transition(i, j, $matrix[i][j]);
            }
        }
    };
}

// generate_transition_matrix takes a MarkovChain and generates a matrix mapping the transitions
// $machine: the MarkovChain
fn generate_transition_matrix(machine: &markov::MarkovChain) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; machine.states.len()]; machine.states.len()];
    for i in 0..machine.states.len() {
        let mut successors = machine.successors(i);
        for j in 0..machine.states.len() {
            if let Some(_) = successors.next() {
                matrix[i][j] = 1.0;
            }
        }
    }
    matrix
}

// random_transition_matrix generates a random transition matrix for a MarkovChain to test the function generate_transition_matrix
// random_transition_matrix takes a MarkovChain with no transitions as input and returns a valid transition matrix
// $machine: the MarkovChain
fn random_transition_matrix(machine: &mut markov::MarkovChain) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; machine.states.len()]; machine.states.len()];
    let mut rng = thread_rng();
    for i in 0..machine.states.len() {
        let mut successors = machine.successors(i);
        for j in 0..machine.states.len() {
            if let Some(_) = successors.next() {
                matrix[i][j] = rng.gen_range(0.0, 1.0);
            }
        }
    }
    matrix
}
