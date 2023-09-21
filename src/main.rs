use rand::*;
use std::collections::HashSet;
use crate::markov::{MarkovChain};
use crate::transfer_lines::TransferLine;
use uuid::Uuid;
use tokio::time::{delay_for, Duration};
use tokio::sync::mpsc::{channel, Sender, Receiver};
use tokio::sync::mpsc;
use time::sleep;

mod markov;
mod transfer_lines;
mod queue;
mod machine;

#[tokio::main]
async fn main() {
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
