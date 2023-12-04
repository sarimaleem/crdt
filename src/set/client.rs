pub struct SetsClient {
  id: String,
  n_requests: i32,
  network: Network,
  assigned_replica_id: String,
  rx: Receiver<Message>,
  running: Arc<AtomicBool>,
  operations: Vec<Message>,
}

impl SetsClient {
  pub fn new(
      id: String,
      n_requests: i32,
      network: Network,
      assigned_replica_id: String,
      rx: Receiver<Message>,
      running: Arc<AtomicBool>,
      operations: Vec<Message>,
  ) -> Self {
      Self {
          id,
          n_requests,
          network,
          assigned_replica_id,
          rx,
          running,
          operations,
      }
  }

  fn handle_sets_read_result(&mut self, r: SetGetResult) {
      println!("{}: {:?}", self.id, r.result);
  }
}

impl Runnable for SetsClient {
  fn run(&mut self) {
      for op in &self.operations {
          self.network.send_message(&self.assigned_replica_id, op.clone());
      }

      // TODO figure out timeouts and dropped messages here, do we resend? maybe there should be
      // another strategy and a resend on a timeout. maybe there needs to be timestamps on no
      // ack
      // thread::sleep(Duration::from_millis(10));
      // let message = Message::create_counter_read_request(self.id.clone());
      // self.network
      //     .send_message(&self.assigned_replica_id, message);
      // thread::sleep(Duration::from_millis(10));

      while self.running.load(Ordering::SeqCst) {
          let r = self.rx.try_recv();
          if let Ok(message) = r {
              match message {
                  Message::SetGetResult(result) => self.handle_sets_read_result(result),
                  _ => panic!(),
              }
          }
      }
  }
}
