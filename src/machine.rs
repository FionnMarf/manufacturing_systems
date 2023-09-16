use crate::queue::Buffer;
use crate::markov::MarkovChain;
use crate::create_machine_chain;

pub struct Machine {
    pub markov_chain: MarkovChain,
    pub processing_time: f64,
    pub num_items: usize,
    pub input_buffer: Option<Buffer>,
    pub output_buffer: Option<Buffer>,
}

impl Machine {
    pub fn new(markov_chain: MarkovChain, processing_time: f64) -> Machine {
        Machine {
            markov_chain: markov_chain,
            processing_time: processing_time,
            num_items: 0,
            output_name: Option<String>,
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

    pub fn add_input_buffer(&mut self, capacity: usize, throughput: Option<f64>) {
        self.input_buffer = Some(Buffer::new(capacity, throughput));
    }

    pub fn add_output_buffer(&mut self, capacity: usize, throughput: Option<f64>) {
        self.output_buffer = Some(Buffer::new(capacity, throughput));
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
        match self.input_buffer {
            Some(ref buffer) => buffer.num_items(),
            None => 0,
        }
    }

    pub fn num_items_in_output_buffer(&self) -> usize {
        match self.output_buffer {
            Some(ref buffer) => buffer.num_items(),
            None => 0,
        }
    }

    pub fn is_input_buffer_full(&self) -> bool {
        match self.input_buffer {
            Some(ref buffer) => buffer.is_full(),
            None => false,
        }
    }

    pub fn is_output_buffer_full(&self) -> bool {
        match self.output_buffer {
            Some(ref buffer) => buffer.is_full(),
            None => false,
        }
    }

    pub fn is_input_buffer_empty(&self) -> bool {
        match self.input_buffer {
            Some(ref buffer) => buffer.is_empty(),
            None => false,
        }
    }

    pub fn is_output_buffer_empty(&self) -> bool {
        match self.output_buffer {
            Some(ref buffer) => buffer.is_empty(),
            None => false,
        }
    }

    pub fn add_item_to_input_buffer(&mut self) {
        match self.input_buffer {
            Some(ref mut buffer) => buffer.add_item(),
            None => (),
        }
    }

    pub fn add_item_to_output_buffer(&mut self) {
        match self.output_buffer {
            Some(ref mut buffer) => buffer.add_item(),
            None => (),
        }
    }

    pub fn remove_item_from_input_buffer(&mut self) {
        match self.input_buffer {
            Some(ref mut buffer) => buffer.remove_item(),
            None => (),
        }
    }

    pub fn remove_item_from_output_buffer(&mut self) {
        match self.output_buffer {
            Some(ref mut buffer) => buffer.remove_item(),
            None => (),
        }
    }

    pub fn step(&mut self) {
        // step the markov chain forward
        MarkovChain::step_chain(&mut self.markov_chain);
        // step the input buffer forward
        match self.input_buffer {
            Some(ref mut buffer) => self.remove_item_from_input_buffer(),
            None => (),
        }
        // step the output buffer forward
        match self.output_buffer {
            Some(ref mut buffer) => self.add_item_to_output_buffer(),
            None => (),
        }
    }

    pub fn set_output_name(&mut self, name: String) {
        self.output_name = Some(name);
    }

    pub fn get_output_name(&self) -> Option<String> {
        self.output_name
    }
}

pub struct Item {
    pub name: String,
    pub size: f64,
    pub cost: Option<f64>,
}