use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
}

impl Token {
    pub fn new() -> Self {
        Token {
            name: "SUZO".to_string(),
            symbol: "SUZO".to_string(),
            total_supply: 500_000_000, // 500M Supply
        }
    }

    pub fn get_supply(&self) -> u64 {
        self.total_supply
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supply() {
        let suzo = Token::new();
        assert_eq!(suzo.get_supply(), 500_000_000);
    }
}
