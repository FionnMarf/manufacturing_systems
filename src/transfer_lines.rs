/// The goal of this module is to provide a way to represent a transfer line
/// in a manufacturing system. A transfer line is a set of machines, M_1 to M_n,
/// which are connected by a set of buffers B_1 to B_n-1. Each buffer B_i is
/// connected to machines M_i and M_i+1. The first machine M_1 is connected to
/// a source of items, and the last machine M_n is connected to a sink of items.
/// Each machine M_i has a processing time P_i, and each buffer B_i has a
/// capacity C_i. 
/// Each machine in the transfer line is represented by a Markov chain. The
/// state of the Markov chain is the state of the machine, Idle, Working, or
/// Broken.
/// 
/// The transfer line is represented by a struct called TransferLine.

use rand::*;
use std::collections::HashSet;

use markov::*;

/// A struct representing a transfer line in a manufacturing system.
/// TODO adjust Queue so that $mu is P_i and $size is C_i
pub struct TransferLine {
    /// The machines in the transfer line.
    pub machines: Vec<MarkovChain>,
    /// The buffers in the transfer line.
    pub buffers: Vec<Queue>,
    /// The processing times of the machines.
    pub processing_times: Vec<f64>,
    /// The capacities of the buffers.
    pub capacities: Vec<usize>,
    /// The number of items in the transfer line.
    pub num_items: usize,
}
