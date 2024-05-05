use std::collections::BTreeMap;
use crate::balances::AccountId;

type BlockNumber = u32;
type Nonce = u32;

#[derive(Debug)]
pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<AccountId, Nonce>
}

impl Pallet {
    pub fn new() -> Self {
        Self{
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        *self.nonce.entry(who.to_string()).or_insert(0) += 1;
    }
}


#[cfg(test)]
mod test {
    use super::Pallet;

    #[test]
    fn initial_block_number() {
        let systems = Pallet::new();
        assert_eq!(systems.block_number(), 0);
    }

    #[test]
    fn increment_block_number() {
        let mut systems = Pallet::new();
        systems.inc_block_number();
        assert_eq!(systems.block_number(), 1);
    }

    #[test]
    fn increment_nonce() {
        let mut systems = Pallet::new();
        systems.inc_nonce(&"alice".to_string());
        let nonce = systems.nonce.get("alice").unwrap();
        assert_eq!(*nonce, 1);
    }
}