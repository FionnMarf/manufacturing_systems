// Rewriting in line with 
// http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

use std::collections::HashSet;

pub type StateIndex = usize;
pub type TransitionIndex = usize;

pub struct MarkovChain {
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
}

pub struct State {
    name: String,
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
    pub fn add_state(&mut self, title: String) -> StateIndex {
        let state_index = self.states.len();
        self.states.push(State { name: title, first_outgoing_transition: None });
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

    pub fn get_state_name(&self, state_index: StateIndex) -> String {
        self.states[state_index].name.clone()
    }

    pub fn get_current_state_name(&self) -> String {
        self.states[0].name.clone()
    }
}

// this macro generates a markov machine called $name with n states and a transition matrix
// $name: the name of the machine
// $n: the number of states
// $matrix: the transition matrix
#[macro_export]
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


// generate_transition_matrix takes a MarkovChain and generates a matrix mapping the transitions
// $machine: the MarkovChain
pub fn generate_transition_matrix(machine: &MarkovChain) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; machine.states.len()]; machine.states.len()];
    for i in 0..machine.states.len() {
        let successors_transitions: Vec<TransitionIndex> = machine.successors(i)
            .map(|target| machine.transitions.iter().position(|trans| trans.target == target).unwrap())
            .collect();
            
        for transition_index in successors_transitions {
            let transition = &machine.transitions[transition_index];
            matrix[i][transition.target] = transition.probability;
        }
    }
    matrix
}

// random_transition_matrix generates a random transition matrix for a MarkovChain to test the function generate_transition_matrix
// random_transition_matrix takes a MarkovChain with no transitions as input and returns a valid transition matrix
// $machine: the MarkovChain
pub fn random_transition_matrix(machine: &mut MarkovChain) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; machine.states.len()]; machine.states.len()];
    let mut rng = rand::thread_rng();
    for i in 0..machine.states.len() {
        let successors: HashSet<usize> = machine.successors(i).collect();  // assuming it returns Iterator<Item=usize>
        for j in 0..machine.states.len() {
            if successors.contains(&j) {
                matrix[i][j] = rand::random::<f64>();
            }
        }
    }
    matrix
}

// a public function which takes an array of markov chains and steps them all forward
// $machines: an array of MarkovChains
pub fn step_chains(machines: &mut Vec<MarkovChain>) {
    for machine in machines {
        step_chain(machine);
    }
}

pub fn step_chain(machine: &mut MarkovChain) {
    let mut rng = rand::thread_rng();
    let mut new_states = Vec::new();
    for i in 0..machine.states.len() {
        let successors: HashSet<usize> = machine.successors(i).collect();  // assuming it returns Iterator<Item=usize>
        let mut random_number = rand::random::<f64>();
        let mut sum = 0.0;
        for j in 0..machine.states.len() {
            if successors.contains(&j) {
                sum += machine.transitions[j].probability;
                if random_number < sum {
                    new_states.push(j);
                    break;
                }
            }
        }
    }
    for i in 0..machine.states.len() {
        machine.states[i].name = machine.states[new_states[i]].name.clone();
    }
}

// a public function to run monte carlo simulations on a markov chain
// $machine: the MarkovChain
// $n: the number of steps to run the simulation for
// $m: the number of simulations to run
pub fn monte_carlo(machine: &mut MarkovChain, n: usize, m: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut matrix = vec![vec![0.0; machine.states.len()]; machine.states.len()];
    for _ in 0..m {
        let mut new_states = Vec::new();
        for i in 0..machine.states.len() {
            let successors: HashSet<usize> = machine.successors(i).collect();  // assuming it returns Iterator<Item=usize>
            let mut random_number = rand::random::<f64>();
            let mut sum = 0.0;
            for j in 0..machine.states.len() {
                if successors.contains(&j) {
                    sum += machine.transitions[j].probability;
                    if random_number < sum {
                        new_states.push(j);
                        break;
                    }
                }
            }
        }
        for i in 0..machine.states.len() {
            matrix[i][new_states[i]] += 1.0;
        }
    }
    for i in 0..machine.states.len() {
        for j in 0..machine.states.len() {
            matrix[i][j] /= m as f64;
        }
    }
    matrix
}

// We need to create a lot more functionality for the MarkovChain struct
// We need to be able to create a graph of markov chains
// We need to be able to step through the graph
// We need to be able to generate a transition matrix for the graph
// and we need to run monte carlo simulations on the graph