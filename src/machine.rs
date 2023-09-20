use crate::queue::Buffer;
use crate::markov::MarkovChain;
use crate::create_machine_chain;
use std::sync::Arc;
use tokio::sync::Mutex;
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

// creating the async aware version outside the impl block





pub struct Buffer {
    pub id: Uuid,
    pub name: Option<String>,
    pub capacity: usize,
    pub num_items: usize,
    pub throughput: Option<f64>,
    pub items: Vec<(Arc<Item>, f64)>,
}

impl Buffer {
    pub fn new(capacity: usize, throughput: Option<f64>, name: Option<String>) -> Buffer {
        Buffer {
            id: Uuid::new_v4(),
            name: name,
            capacity: capacity,
            num_items: 0,
            throughput: throughput,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self) {
        if self.num_items < self.capacity {
            self.num_items += 1;
        }
    }

    pub fn remove_item(&mut self) {
        if self.num_items > 0 {
            self.num_items -= 1;
        }
    }

    pub fn is_full(&self) -> bool {
        self.num_items == self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.num_items == 0
    }

    pub fn num_items(&self) -> usize {
        self.num_items
    }

    pub fn set_throughput(&mut self, throughput: f64) {
        self.throughput = Some(throughput);
    }

    pub fn get_throughput(&self) -> Option<f64> {
        self.throughput
    }

    pub fn change_name(&mut self, name: String) {
        self.name = Some(name);
    }
}

enum BufferEvent {
    ChangeItem { item: Item, quantity: f64 },
    CreateBuffer { capacity: usize, throughput: Option<f64>, name: Option<String> },
}

pub async fn change_items_to_buffer_by_id(
    &self,
    tx: &tokio::sync::mpsc::Sender<(Uuid, BufferEvent)>,
    buffer_id: Uuid,
    item: Item,
    quantity: f64,
) -> Result<(), &'static str> {
    // Send a change request event.
    tx.send((buffer_id, BufferEvent::ChangeItem { item, quantity })).await.map_err(|_| "Failed to send change request")
}

pub async fn create_buffer(
    &self,
    tx: &tokio::sync::mpsc::Sender<(Uuid, BufferEvent)>,
    capacity: usize,
    throughput: Option<f64>,
    name: Option<String>,
) {
    // Send a create buffer event.
    tx.send((Uuid::new_v4(), BufferEvent::CreateBuffer { capacity, throughput, name })).await.map_err(|_| "Failed to send create buffer request")
}

pub async fn buffer_event_handler(rx: tokio::sync::mpsc::Receiver<(Uuid, BufferEvent)>, buffers: Vec<tokio::sync::Mutex<Buffer>>) {
    while let Some((buffer_id, event)) = rx.recv().await {
        match event {
            BufferEvent::ChangeItem { item, quantity } => {
                if let Some(buffer) = buffers.iter().find(|b| b.lock().await.id == buffer_id) {
                    let mut locked_buffer = buffer.lock().await;

                    // Try to find the item in the buffer's items list.
                    if let Some((_item_ref, item_quantity)) = locked_buffer.items.iter_mut().find(|(existing_item, _)| existing_item.id == item.id) {
                        if *item_quantity + quantity < 0.0 {
                            // Handle the error, possibly with logging or other mechanisms.
                        } else {
                            *item_quantity += quantity;
                        }
                    } else {
                        if quantity >= 0.0 {
                            locked_buffer.items.push((Arc::new(item), quantity));
                        } else {
                            // Handle the error.
                        }
                    }
                } else {
                    // Handle the buffer not found error.
                }
            }
            BufferEvent::CreateBuffer { capacity, throughput, name } => {
                // Create a new buffer and add it to the buffers vector.
                let mut buffer = Buffer::new(capacity, throughput, name);
                buffers.push(tokio::sync::Mutex::new(buffer));
            }
        }
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