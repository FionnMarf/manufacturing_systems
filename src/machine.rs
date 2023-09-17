use crate::queue::Buffer;
use crate::markov::MarkovChain;
use crate::create_machine_chain;
use std::sync::{Arc, Mutex};

pub struct Machine {
    pub markov_chain: MarkovChain,
    pub processing_time: f64,
    pub num_items: usize,
    pub output_name: Option<String>,
    pub input_buffer: Option<Arc<Mutex<Buffer>>>,
    pub output_buffer: Option<Arc<Mutex<Buffer>>>,
}

impl Machine {
    pub fn new(markov_chain: MarkovChain, processing_time: f64, output: Option<String>) -> Machine {
        Machine {
            markov_chain: markov_chain,
            processing_time: processing_time,
            num_items: 0,
            output_name: output,
            input_buffer: None,
            output_buffer: None,
        }
    }
    
    /// Creates a machine with a default markov chain. 1% failure rate
    pub fn new_default_machine(name: String, processing_time: f64) -> Machine {
        let markov_chain = create_machine_chain!(name);
        Machine {
            markov_chain: markov_chain,
            processing_time: processing_time,
            num_items: 0,
            output_name: None,
            input_buffer: None,
            output_buffer: None,
        }
    }

    pub fn create_and_add_input_buffer(&mut self, capacity: usize, throughput: Option<f64>) {
        let buffer = Buffer::new(capacity, throughput, None);
        self.input_buffer = Some(Arc::new(Mutex::new(buffer)));
    }

    pub fn create_and_add_output_buffer(&mut self, capacity: usize, throughput: Option<f64>) {
        let buffer = Buffer::new(capacity, throughput, self.output_name.clone());
        self.output_buffer = Some(Arc::new(Mutex::new(buffer)));
    }

    pub fn set_input_buffer(&mut self, buffer: Arc<Mutex<Buffer>>) {
        self.input_buffer = Some(buffer);
    }

    pub fn set_output_buffer(&mut self, buffer: Arc<Mutex<Buffer>>) {
        self.output_buffer = Some(buffer);
    }

    pub fn add_item(&mut self) {
        self.num_items += 1;
    }

    pub fn remove_item(&mut self) {
        self.num_items -= 1;
    }

    pub fn num_items(&self) -> usize {
        self.num_items
    }

    pub fn num_items_in_input_buffer(&self) -> usize {
        match &self.input_buffer {
            Some(buffer) => {
                let locked_buffer = buffer.lock().unwrap();
                locked_buffer.num_items()
            },
            None => 0,
        }
    }

    pub fn num_items_in_output_buffer(&self) -> usize {
        match &self.output_buffer {
            Some(buffer) => {
                let locked_buffer = buffer.lock().unwrap();
                locked_buffer.num_items()
            },
            None => 0,
        }
    }

    pub fn is_input_buffer_full(&self) -> bool {
        match &self.input_buffer {
            Some(buffer) => {
                let locked_buffer = buffer.lock().unwrap();
                locked_buffer.is_full()
            },
            None => false,
        }
    }

    pub fn is_output_buffer_full(&self) -> bool {
        match &self.output_buffer {
            Some(buffer) => {
                let locked_buffer = buffer.lock().unwrap();
                locked_buffer.is_full()
            },
            None => false,
        }
    }

    pub fn is_input_buffer_empty(&self) -> bool {
        match &self.input_buffer {
            Some(buffer) => {
                let locked_buffer = buffer.lock().unwrap();
                locked_buffer.is_empty()
            },
            None => false,
        }
    }

    pub fn is_output_buffer_empty(&self) -> bool {
        match &self.output_buffer {
            Some(buffer) => {
                let locked_buffer = buffer.lock().unwrap();
                locked_buffer.is_empty()
            },
            None => false,
        }
    }

    pub fn add_item_to_input_buffer(&mut self) {
        match &mut self.input_buffer {
            Some(ref mut buffer) => {
                let mut locked_buffer = buffer.lock().unwrap();
                locked_buffer.add_item();
            },
            None => (),
        }
    }

    pub fn add_item_to_output_buffer(&mut self) {
        match &mut self.output_buffer {
            Some(ref mut buffer) => {
                let mut locked_buffer = buffer.lock().unwrap();
                locked_buffer.add_item();
            },
            None => (),
        }
    }

    pub fn remove_item_from_input_buffer(&mut self) {
        match &mut self.input_buffer {
            Some(ref mut buffer) => {
                let mut locked_buffer = buffer.lock().unwrap();
                locked_buffer.remove_item();
            },
            None => (),
        }
    }

    pub fn remove_item_from_output_buffer(&mut self) {
        match &mut self.output_buffer {
            Some(ref mut buffer) => {
                let mut locked_buffer = buffer.lock().unwrap();
                locked_buffer.remove_item();
            },
            None => (),
        }
    }

    pub fn step(&mut self) {
        // step the markov chain forward
        MarkovChain::step_chain(&mut self.markov_chain);
        // step the input buffer forward
        match &mut self.input_buffer {
            Some(ref mut buffer) => self.remove_item_from_input_buffer(),
            None => (),
        }
        // step the output buffer forward
        match &mut self.output_buffer {
            Some(ref mut buffer) => self.add_item_to_output_buffer(),
            None => (),
        }
    }

    pub fn set_output_name(&mut self, name: String) {
        self.output_name = Some(name);
    }

    pub fn get_output_name(&self) -> Option<String> {
        self.output_name.clone()
    }
}

pub struct Item {
    pub name: String,
    pub size: f64,
    pub cost: Option<f64>,
}

impl Item {
    pub fn new(name: String, size: f64, cost: Option<f64>) -> Item {
        Item {
            name: name,
            size: size,
            cost: cost,
        }
    }
}