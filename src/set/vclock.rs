use std::collections::HashMap;
use std::cmp;

pub enum VClockCompareResult {
  EQUAL,
  LESS_THAN,
  GREATER_THAN,
  CONCURRENT,
}

#[derive(Clone, Debug)]
pub struct VClock {
  pub clock: HashMap<String, i32>,
}

impl VClock {
  pub fn new(total_replicas: usize) -> Self {
      let mut tmp = Self {
          clock: HashMap::new(),
      };
      for i in 0..total_replicas {
          tmp.clock.insert(format!("replica_{}", i), 0);
      }
      tmp
  }

  pub fn new_with_clock(m: HashMap<String, i32>) -> Self {
      Self { clock: m }
  }

  pub fn compare(&self, rhs: &VClock) -> VClockCompareResult {
      let mut less = false;
      let mut more = false;
      for (replica_id, clock_value) in &self.clock {
          let lhs_stamp = clock_value;
          let rhs_stamp = rhs.clock.get(replica_id).unwrap();
          if lhs_stamp < rhs_stamp {
              less = true
          } else if lhs_stamp > rhs_stamp {
              more = true
          }
      }

      if less & more {
          return crate::set::vclock::VClockCompareResult::CONCURRENT;
      }

      if less {
          return crate::set::vclock::VClockCompareResult::LESS_THAN;
      }

      if more {
          return crate::set::vclock::VClockCompareResult::GREATER_THAN;
      }

      crate::set::vclock::VClockCompareResult::EQUAL
  }

  pub fn merge(clk1: &VClock, clk2: &VClock) -> VClock {
      let mut new_clock: HashMap<String, i32> = HashMap::new();
      for key in clk1.clock.keys() {
          new_clock.insert(
              key.clone(),
              cmp::max(
                  clk1.clock.get(key).unwrap().clone(),
                  clk2.clock.get(key).unwrap().clone(),
              ),
          );
      }
      VClock::new_with_clock(new_clock)
  }

  pub fn increment(clk: &VClock, id: &String) -> VClock {
      // let temp = VClock::new_with_clock(self.clock.clone());
      let temp_map = clk.clock.clone();
      let mut temp = VClock::new_with_clock(temp_map);
      
      temp.clock
          .insert(id.clone(), temp.clock.get(id).unwrap() + 1);
      temp
  }
}