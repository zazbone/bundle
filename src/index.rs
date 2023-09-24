use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BundleIndex {
    name: String,
    num_entry: usize,
    free_entries: HashSet<usize>,
    entries: Vec<serde_json::Value>,
}

impl BundleIndex {
    pub fn new(name: String) -> Self {
        Self {
            name,
            num_entry: 0,
            free_entries: HashSet::new(),
            entries: Vec::new(),
        }
    }

    pub(crate) fn new_entry<T: Serialize>(&mut self, entry: &T) -> usize {
        let oindex = self.free_entries.iter().next().map(|i| *i);
        if let Some(index) = oindex {
            self.free_entries.remove(&index);
            self.entries[index] = serde_json::to_value(entry).unwrap();
            index
        } else {
            self.entries.push(serde_json::to_value(entry).unwrap());
            self.num_entry += 1;
            self.num_entry - 1
        }
    }

    pub(crate) fn rm_entry(&mut self, index: usize) {
        self.free_entries.insert(index);
        self.entries[index] = serde_json::Value::Null;
    }

    pub(crate) fn find_entry_index<T>(&self, target: &T) -> Option<usize>
    where
        T: PartialEq<serde_json::Value>,
    {
        for (i, entry) in self.entries.iter().enumerate() {
            if target == entry {
                return Some(i);
            }
        }
        None
    }
}
