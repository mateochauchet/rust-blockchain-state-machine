use std::{collections::BTreeMap, ops::AddAssign};
use num::{ One, Zero };

/*
	TODO: Combine all generic types and their trait bounds into a single `pub trait Config`.
	When you are done, your `Pallet` can simply be defined with `Pallet<T: Config>`.
*/

pub trait Config {
	type AccountId: Ord + Clone;
	type BlockNumber: Zero + One + AddAssign + Copy;
	type Nonce: Zero + One + Copy;
}


#[derive(Debug)]
pub struct Pallet<T: Config>
{
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>
}

impl<T: Config> Pallet<T> 
{
    pub fn new() -> Self {
        Self{
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
		let new_nonce = nonce + T::Nonce::one();
		self.nonce.insert(who.clone(), new_nonce);
    }
}


#[cfg(test)]
mod test {
    use super::{Config, Pallet};

    type TestAccountId = String;
    type TestBlockNumber = u32;
    type TestNonce = u32;

    struct TestConfig;


    impl Config for TestConfig {
        type AccountId = String;
	    type BlockNumber = u32;
	    type Nonce = u32;
    }

    #[test]
    fn initial_block_number() {
        let systems = Pallet::<TestConfig>::new();
        assert_eq!(systems.block_number(), 0);
    }

    #[test]
    fn increment_block_number() {
        let mut systems = Pallet::<TestConfig>::new();
        systems.inc_block_number();
        assert_eq!(systems.block_number(), 1);
    }

    #[test]
    fn increment_nonce() {
        let mut systems = Pallet::<TestConfig>::new();
        systems.inc_nonce(&"alice".to_string());
        let nonce = systems.nonce.get("alice").unwrap();
        assert_eq!(*nonce, 1);
    }
}