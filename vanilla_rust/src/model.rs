
pub struct Counter{
    pub count: u32,
}

impl Counter {
    pub fn new(init_count: u32) -> Self{
        let counter = Self { count: init_count};
        return counter
    }

    pub fn increment(&mut self){
        self.count = self.count.checked_add(1).ok_or(CounterError::Overflow).unwrap()
    }

    pub fn reset(&mut self){
        self.count = 0;
    }
}



#[derive(Debug)]
pub enum CounterError {
    Overflow
}

