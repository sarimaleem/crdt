// TODO add more stuff here

pub trait Runnable {
    fn run(&mut self);
}

pub trait Counter: Runnable {
    fn read(&self) -> i32;
    fn increment(&self);
}

