use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub(crate) name: String,
    pub(crate) hash_id: u64,
    meta: HashMap<String, String>,
}

impl FileInfo {
    pub fn new(name: String, meta: HashMap<String, String>) -> FileInfo {
        let mut res = FileInfo {
            name: name,
            hash_id: 0,
            meta: meta,
        };
        let mut hasher = DefaultHasher::new();
        res.hash(&mut hasher);
        res.hash_id = hasher.finish();
        res
    }
}

// FIX: Bad hash algorithm:
// ("b=o", "b") and ("b", "o=b") as the same key valye pairs.
// Also, A sort and a vec allocation is neccessary. A order independant hash algorithm may fix those issues
impl Hash for FileInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        let mut kv_vec = self
            .meta
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>();
        kv_vec.sort();
        for kv in kv_vec.iter() {
            kv.hash(state);
        }
    }
}

impl PartialEq<FileInfo> for FileInfo {
    fn eq(&self, other: &FileInfo) -> bool {
        self.hash_id == other.hash_id
    }
}
