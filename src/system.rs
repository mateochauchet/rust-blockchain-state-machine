use std::{collections::BTreeMap, ops::AddAssign};
use num::{ One, Zero };

#[derive(Debug)]
pub struct Pallet<BlockNumber, Nonce, AccountId>
{
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>
}

impl<BlockNumber, Nonce, AccountId> Pallet<BlockNumber, Nonce, AccountId> 
where
    BlockNumber: Zero + One + AddAssign + Copy,
    Nonce: Zero + One + Copy,
    AccountId: Ord + Clone,
{
    pub fn new() -> Self {
        Self{
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
		let new_nonce = nonce + Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
    }
}


#[cfg(test)]
mod test {
    use super::Pallet;

    type TestAccountId = String;
    type TestBlockNumber = u32;
    type TestNonce = u32;

    #[test]
    fn initial_block_number() {
        let systems = Pallet::<TestBlockNumber, TestNonce, TestAccountId>::new();
        assert_eq!(systems.block_number(), 0);
    }

    #[test]
    fn increment_block_number() {
        let mut systems = Pallet::<TestBlockNumber, TestNonce, TestAccountId>::new();
        systems.inc_block_number();
        assert_eq!(systems.block_number(), 1);
    }

    #[test]
    fn increment_nonce() {
        let mut systems = Pallet::<TestBlockNumber, TestNonce, TestAccountId>::new();
        systems.inc_nonce(&"alice".to_string());
        let nonce = systems.nonce.get("alice").unwrap();
        assert_eq!(*nonce, 1);
    }
}