use chrono::Utc;
use md5;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp() as u64;
        let hash = format!("{:x}", md5::compute(format!("{}{}", index, timestamp)));

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
        }
    }
}
