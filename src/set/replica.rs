use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::{mpsc::Receiver, atomic::AtomicBool};
use crate::network::Network;
use crate::set::vclock::VClock;
use crate::message::Message;
use crate::message::{SetGetRequest, SetInsertRequest, SetRemoveRequest, SetMerge};
use crate::traits::Runnable;

pub struct SetsReplica {
  id: String,
  total_replicas: usize,
  rx: Receiver<Message>,
  network: Network,
  running: Arc<AtomicBool>,
  adds: HashMap<String, VClock>,
  removes: HashMap<String, VClock>,
}

impl SetsReplica {
  pub fn new(
      id: String,
      total_replicas: usize,
      rx: Receiver<Message>,
      network: Network,
      running: Arc<AtomicBool>,
  ) -> Self {
      Self {
          id,
          total_replicas,
          rx,
          network,
          running,
          adds: HashMap::new(),
          removes: HashMap::new(),
      }
  }

  fn handle_set_get(&mut self, msg: SetGetRequest) {
      let mut result: HashSet<String> = HashSet::new();

      for (item, clk) in &self.removes {
          // let otherClk = self.removes.get(item).unwrap().clock;
          match self.adds.get(&*item) {
              Some(add) => {
                  match add.compare(clk) {
                    crate::set::vclock::VClockCompareResult::LESS_THAN => {
                          result.insert(item.clone());
                      }
                      _ => {}
                  };
              }
              None => {}
          }
      }

      self.network.send_message(
          &msg.sender_id,
          Message::create_set_get_result(self.id.clone(), result),
      );
  }

  fn handle_set_insert(&mut self, message: SetInsertRequest) {
      let string_to_add = message.request;
      let add_clk = self.adds.get(&string_to_add);
      let mut remove_clk = self.removes.get(&string_to_add);

      match (add_clk, remove_clk) {
          (Some(v), _) | (_, Some(v)) => {
              let mut clk = VClock::increment(&v, &self.id);
              self.adds.insert(string_to_add.clone(), clk);
              self.removes.remove(&string_to_add);
          }
          (_, _) => {
              let mut clk = VClock::new(self.total_replicas);
              clk = VClock::increment(&clk, &self.id);
              self.adds.insert(string_to_add, clk);
          }
      }

      self.network.broadcast_replicas(Message::create_set_merge(
          self.id.clone(),
          self.adds.clone(),
          self.removes.clone(),
      ));
  }

  fn handle_set_remove(&mut self, message: SetRemoveRequest) {
      let string_to_add = message.request;
      let add_clk = self.adds.get(&string_to_add);
      let mut remove_clk = self.removes.get(&string_to_add);

      match (add_clk, remove_clk) {
          (Some(v), _) | (_, Some(v)) => {
              let mut clk = VClock::increment(&v, &self.id);
              self.adds.remove(&string_to_add);
              self.removes.insert(
                  string_to_add.clone(),
                  VClock::increment(&clk, &string_to_add),
              );
          }
          (_, _) => {
              let mut clk = VClock::new(self.total_replicas);
              clk = VClock::increment(&clk, &self.id);
              self.removes.insert(string_to_add, clk);
          }
      }

      self.network.broadcast_replicas(Message::create_set_merge(
          self.id.clone(),
          self.adds.clone(),
          self.removes.clone(),
      ));
  }

  fn merge_map(
      a: &HashMap<String, VClock>,
      b: &HashMap<String, VClock>,
  ) -> HashMap<String, VClock> {
      let mut result = a.clone();
      for (k, vb) in b {
          let entry = result.entry(k.clone()).or_insert(vb.clone());
          *entry = VClock::merge(entry, vb);
      }
      result
  }

  fn handle_set_merge(&mut self, message: SetMerge) {
      let adds1 = self.adds.clone();
      let removes1 = self.removes.clone();

      let adds2 = message.add_map.clone();
      let removes2 = message.remove_map.clone();

      let addk = SetsReplica::merge_map(&adds1, &adds2);
      let removek = SetsReplica::merge_map(&removes1, &removes2);

      // let add = removek.iter().fold(addk.clone(), |mut acc, (k, vr)| {
      //     if let Some(&va) = acc.get(k) {
      //         match va.compare(vr) {
      //             crate::replica::VClockCompareResult::LESS_THAN => {
      //                 acc.remove(k);
      //             }
      //             _ => ()
      //         }
      //     }
      //     acc
      // });
      let add = removek.iter().fold(addk.clone(), |mut acc, (k, vr)| {
          // Use the entry API to handle both the existence and non-existence of the key
          match acc.entry(k.clone()) {
              std::collections::hash_map::Entry::Occupied(entry) => {
                  match entry.get().compare(vr) {
                      crate::set::vclock::VClockCompareResult::LESS_THAN => {
                          // Remove the entry if it meets the condition
                          entry.remove_entry();
                      }
                      _ => (),
                  }
              }
              std::collections::hash_map::Entry::Vacant(_) => (),
          }
          acc
      });

      // let rem = addk.iter().fold(removek, |mut acc, (k, va)| {
      //     if let Some(&vr) = acc.get(k) {
      //         match va.compare(&vr) {
      //             crate::replica::VClockCompareResult::LESS_THAN => {
      //                 acc.remove(k);
      //             }
      //             _ => ()
      //         }
      //     }
      //     acc
      // });

      let rem = addk.iter().fold(removek, |mut acc, (k, va)| {
          match acc.entry(k.clone()) {
              std::collections::hash_map::Entry::Occupied(entry) => {
                  if let crate::set::vclock::VClockCompareResult::LESS_THAN = va.compare(entry.get())
                  {
                      // Remove the entry if it meets the condition
                      entry.remove();
                  }
              }
              std::collections::hash_map::Entry::Vacant(_) => {}
          }
          acc
      });

      self.adds = add;
      self.removes = rem;
  }
}

impl Runnable for SetsReplica {
  fn run(&mut self) {
      // todo!()
      while self.running.load(Ordering::SeqCst) {
          let r = self.rx.try_recv();
          if let Ok(message) = r {
              match message {
                  Message::SetGetRequest(message) => self.handle_set_get(message),
                  Message::SetRemoveRequest(message) => self.handle_set_remove(message),
                  Message::SetInsertRequest(message) => self.handle_set_insert(message),
                  Message::SetMerge(message) => self.handle_set_merge(message),
                  _ => panic!(),
              }
          }
      }
  }
}
