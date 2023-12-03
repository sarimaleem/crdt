use std::cmp::Ordering;

pub struct VPtr {
    pub sequence: Vec<u8>,
    pub id: String,
}

impl VPtr {
    pub fn new(sequence: Vec<u8>, id: String) -> Self {
        Self { sequence, id }
    }
}


impl Clone for VPtr {
    fn clone(&self) -> Self {
        Self {
            sequence: self.sequence.clone(),
            id: self.id.clone(),
        }
    }
}

impl Ord for VPtr {
    fn cmp(&self, other: &Self) -> Ordering {
        let len = std::cmp::min(self.sequence.len(), other.sequence.len());
        for i in 0..len {
            let cmp = self.sequence[i].cmp(&other.sequence[i]);
            if cmp != Ordering::Equal {
                return cmp;
            }
        }

        if self.sequence.len() != other.sequence.len() {
            return self.sequence.len().cmp(&other.sequence.len());
        }

        self.id.cmp(&other.id)
    }
}

impl PartialOrd for VPtr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for VPtr {}

impl PartialEq for VPtr {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
