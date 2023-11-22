use crate::traits::Runnable;

pub struct Client {
    id: i32,
    n_requests: i32,
}

impl Client {
    pub fn new() -> Self {
        Self {
            id: 0,
            n_requests: 0,
        }
    }
}

impl Runnable for Client {
    // add code here
    fn run(&self) {
        loop {}
    }
}
