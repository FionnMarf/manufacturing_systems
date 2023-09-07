// a struct representing an M/M/1 queue
// $lambda: the arrival rate
// $mu: the service rate
pub fn mm1(lambda: f64, mu: f64) -> MarkovChain {
    let mut states = Vec::new();
    states.push(State::new("0".to_string()));
    states.push(State::new("1".to_string()));
    let mut transitions = Vec::new();
    transitions.push(Transition::new(0, 1, lambda));
    transitions.push(Transition::new(1, 0, mu));
    MarkovChain::new(states, transitions)
}