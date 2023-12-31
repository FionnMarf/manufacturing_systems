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
use crate::machine::Machine;
use crate::markov::MarkovChain;
use crate::queue::Buffer;
use uuid::Uuid;

/// A struct representing a transfer line in a manufacturing system.
pub struct TransferLine {
    pub id: Uuid,
    /// The machines in the transfer line.
    pub machines: Vec<Machine>,
    /// The buffers in the transfer line.
    pub buffers: Vec<Buffer>,
    /// The processing times of the machines.
    pub processing_times: Vec<f64>,
    /// The capacities of the buffers.
    pub capacities: Vec<usize>,
    /// The number of items in the transfer line.
    pub num_items: usize,
    /// The current time step
    pub time_step: usize,
}

impl TransferLine {
    /// Creates a new transfer line.
    pub fn new(processing_times: Vec<f64>, capacities: Vec<usize>, throughputs: Vec<Option<f64>>) -> TransferLine {
        let mut machines = Vec::new();
        let mut buffers = Vec::new();
        let num_machines = processing_times.len();
        let num_buffers = capacities.len();
        for i in 0..num_machines {
            machines.push(Machine::new(MarkovChain::new(), processing_times[i], None));
        }
        for i in 0..num_buffers {
            buffers.push(Buffer::new(capacities[i], throughputs[i], None));
        }
        TransferLine {
            id: Uuid::new_v4(),
            machines: machines,
            buffers: buffers,
            processing_times: processing_times,
            capacities: capacities,
            num_items: 0,
            time_step: 1,
        }
    }

    /// Adds a machine to the transfer line.
    pub fn add_machine(&mut self, processing_time: f64, output: Option<String>) {
        self.machines.push(Machine::new(MarkovChain::new(), processing_time, output));
    }

    /// Adds a buffer to the transfer line.
    pub fn add_buffer(&mut self, capacity: usize, throughput: Option<f64>) {
        self.buffers.push(Buffer::new(capacity, throughput, None));
    }

    /// Adds an item to the transfer line.
    pub fn add_item(&mut self) {
        self.num_items += 1;
    }

    /// Removes an item from the transfer line.
    pub fn remove_item(&mut self) {
        self.num_items -= 1;
    }

    /// Returns the number of items in the transfer line.
    pub fn num_items(&self) -> usize {
        self.num_items
    }

    // Steps the transfer line forward one time step.
    // pub fn step(&mut self) {
    //     // step the machines forward
    //     for machine in &mut self.machines {
    //         self.step(machine);
    //     }
    // }     
}
