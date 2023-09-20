use crate::queue::Buffer;
use crate::markov::MarkovChain;
use crate::create_machine_chain;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct Machine {
    pub id: Uuid,
    pub markov_chain: MarkovChain,
    pub processing_time: f64,
    pub num_items: usize,
    pub output_name: Option<String>,
    pub input_buffer: Vec<Arc<Mutex<Buffer>>>,
    pub output_buffer: Vec<Arc<Mutex<Buffer>>>,
}

impl Machine {
    pub fn new(markov_chain: MarkovChain, processing_time: f64, output: Option<String>) -> Machine {
        Machine {
            id: Uuid::new_v4(),
            markov_chain: markov_chain,
            processing_time: processing_time,
            num_items: 0,
            output_name: output,
            input_buffer: Vec::new(),
            output_buffer: Vec::new(),
        }
    }
    
    /// Creates a machine with a default markov chain. 1% failure rate
    pub fn new_default_machine(name: String, processing_time: f64) -> Machine {
        let markov_chain = create_machine_chain!(name);
        Machine {
            id: Uuid::new_v4(),
            markov_chain: markov_chain,
            processing_time: processing_time,
            num_items: 0,
            output_name: None,
            input_buffer: Vec::new(),
            output_buffer: Vec::new(),
        }
    }

    pub fn create_and_add_input_buffer(&mut self, capacity: usize, throughput: Option<f64>) {
        let buffer = Buffer::new(capacity, throughput, None);
        self.add_input_buffer(Arc::new(Mutex::new(buffer)));
    }

    pub fn create_and_add_output_buffer(&mut self, capacity: usize, throughput: Option<f64>) {
        let buffer = Buffer::new(capacity, throughput, self.output_name.clone());
        self.add_output_buffer(Arc::new(Mutex::new(buffer)));
    }

    pub fn add_input_buffer(&mut self, buffer: Arc<Mutex<Buffer>>) {
        let buffer_id_to_add = buffer.lock().unwrap().id;

        if let Some(index) = self.input_buffer.iter().position(|x| x.lock().unwrap().id == buffer_id_to_add) {
            self.input_buffer[index] = buffer;
        } else {
            self.input_buffer.push(buffer);
        }
    }

    pub fn remove_input_buffer(&mut self, buffer: Arc<Mutex<Buffer>>) {
        let buffer_id_to_remove = buffer.lock().unwrap().id;
    
        if let Some(index) = self.input_buffer.iter().position(|x| x.lock().unwrap().id == buffer_id_to_remove) {
            self.input_buffer.remove(index);
        }
    }

    pub fn add_output_buffer(&mut self, buffer: Arc<Mutex<Buffer>>) {
        self.output_buffer.push(buffer);
    }

    pub fn remove_output_buffer(&mut self, buffer: &Arc<Mutex<Buffer>>) {
        let buffer_id_to_remove = buffer.lock().unwrap().id;
    
        if let Some(index) = self.output_buffer.iter().position(|x| x.lock().unwrap().id == buffer_id_to_remove) {
            self.output_buffer.remove(index);
        }
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

    pub fn change_items_to_buffer_by_id(&mut self, buffer_id: Uuid, item: Item, quantity: f64) -> Result<(), &'static str> {
        if let Some(buffer) = self.output_buffer.iter().find(|b| b.lock().unwrap().id == buffer_id) {
            let mut locked_buffer = buffer.lock().unwrap();

            // Try to find the item in the buffer's items list.
            if let Some((_item_ref, item_quantity)) = locked_buffer.items.iter_mut().find(|(existing_item, _)| existing_item.id == item.id) {
                if *item_quantity + quantity < 0.0 {
                    return Err("Quantity is negative, buffer quantity cannot drop below zero.");
                } else {
                *item_quantity += quantity;  // If found, increase the quantity.
                }
                Ok(())
            } else {
                // If not found, add the item to the buffer's items list.
                if quantity >= 0.0 {
                locked_buffer.items.push((Arc::new(item), quantity));
                } else {
                    return Err("Quantity is negative, buffer quantity cannot drop below zero.");
                }
                Ok(())
            }

        } else {
            Err("Buffer not found.")
        }
    }

    pub fn step(&mut self) {
        todo!();
    }

    pub fn set_output_name(&mut self, name: String) {
        self.output_name = Some(name);
    }

    pub fn get_output_name(&self) -> Option<String> {
        self.output_name.clone()
    }
}

pub struct Item {
    id: Uuid,
    pub name: String,
    pub size: f64,
    pub cost: Option<f64>,
}

impl Item {
    pub fn new(name: String, size: f64, cost: Option<f64>) -> Item {
        Item {
            id: Uuid::new_v4(),
            name: name,
            size: size,
            cost: cost,
        }
    }
}