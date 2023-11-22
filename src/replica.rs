use crate::traits::Runnable;

pub struct Replica {
    id: i32,
    counter: i32,
}

impl Replica {
    pub fn new() -> Self {
        Self { id: 0, counter: 0 }
    }

    fn handleRead(&self) {

    }

    fn handleAdd(&self) {

    }
}

impl Runnable for Replica {
    fn run(&self) {
        loop {}
    }
}
