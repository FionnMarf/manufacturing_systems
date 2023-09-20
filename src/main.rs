use rand::*;
use std::collections::HashSet;
use crate::markov::{MarkovChain};
use crate::transfer_lines::TransferLine;
use uuid::Uuid;
use tokio::time::{delay_for, Duration};
use tokio::sync::mpsc::{channel, Sender, Receiver};

mod markov;
mod transfer_lines;
mod queue;
mod machine;

#[tokio::main]
async fn main() {
    // we are writing a tokio based async aware main function for our simulation
    // we will use the tokio::time::delay_for() function to simulate the passage of time
    // we will use the tokio::sync::mpsc::channel() function to create a channel for sending messages
    let (tx, rx) = mpsc::channel(32);
    let mut transfer_line = TransferLine::new(vec![1.0, 1.0], vec![1, 1], vec![None, None]);
    transfer_line.add_machine(1.0, None);
    transfer_line.add_buffer(1, None);
    let ticker = tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(1)).await; // Represents a very small time increment
            // Emit time events or update global state
        }
    });
}
