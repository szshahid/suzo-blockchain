use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Wallet {
    pub address: String,
    pub balance: u64,
}

impl Wallet {
    pub fn new(address: String) -> Self {
        Wallet { address, balance: 0 }
    }

    pub fn credit(&mut self, amount: u64) {
        self.balance += amount;
    }

    pub fn debit(&mut self, amount: u64) -> bool {
        if self.balance >= amount {
            self.balance -= amount;
            return true;
        }
        false
    }
}
