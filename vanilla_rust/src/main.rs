mod Model;
use crate::Model::Counter;

fn main() {
    println!("Wubudaba-lab-lab!")
}

#[test]
pub fn setup(){
    let mut counter = Counter::new(0);
}

#[test]
pub fn when_called_increment_should_increase(){
    let mut counter = Counter::new(0);
    counter.increment();
    assert_eq!(counter.count, 1)
}

#[test]
pub fn when_called_reset_should_reset(){
    let mut counter = Counter::new(1);
    counter.reset();
    assert_eq!(counter.count,0)
}
