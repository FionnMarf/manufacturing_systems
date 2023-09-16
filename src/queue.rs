// a struct representing an M/M/1 queue
// $lambda: the arrival rate
// $mu: the service rate
// $size: the number of items in the queue
pub struct Queue {
    lambda: f64,
    mu: f64,
    size: usize,
}

impl Queue {
    pub fn new(lambda: f64, mu: f64, size: usize) -> Queue {
        Queue {
            lambda: lambda,
            mu: mu,
            size: size,
        }
    }

    // returns the average number of items in the queue
    pub fn avg_num_items(&self) -> f64 {
        self.lambda / (self.mu - self.lambda)
    }

    // returns the average time an item spends in the queue
    pub fn avg_time_in_queue(&self) -> f64 {
        1.0 / (self.mu - self.lambda)
    }

    // returns the probability that the queue is empty
    pub fn prob_empty(&self) -> f64 {
        1.0 - (self.lambda / self.mu)
    }

    // returns the probability that the queue is full
    pub fn prob_full(&self) -> f64 {
        (self.lambda / self.mu).powi(self.size as i32)
    }

    // returns the probability that the queue is at a given level
    pub fn prob_level(&self, level: usize) -> f64 {
        (self.lambda / self.mu).powi(level as i32)
    }

    // returns the probability that the queue is at a given level or higher
    pub fn prob_level_or_higher(&self, level: usize) -> f64 {
        let mut prob = 0.0;
        for i in level..self.size + 1 {
            prob += self.prob_level(i);
        }
        prob
    }

    // returns the probability that the queue is at a given level or lower
    pub fn prob_level_or_lower(&self, level: usize) -> f64 {
        let mut prob = 0.0;
        for i in 0..level + 1 {
            prob += self.prob_level(i);
        }
        prob
    }

    // returns the probability that the queue is at a given level or between two levels
    pub fn prob_level_between(&self, level1: usize, level2: usize) -> f64 {
        let mut prob = 0.0;
        for i in level1..level2 + 1 {
            prob += self.prob_level(i);
        }
        prob
    }
}

pub struct Buffer {
    pub name: Option<String>,
    pub capacity: usize,
    pub num_items: usize,
    pub throughput: Option<f64>,
}

impl Buffer {
    pub fn new(capacity: usize, throughput: Option<f64>, name: Option<String>) -> Buffer {
        Buffer {
            name: name,
            capacity: capacity,
            num_items: 0,
            throughput: None,
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