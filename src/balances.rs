use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};



#[derive(Debug)]
pub struct Pallet<AccountId, Balance>
{
	balances: BTreeMap<AccountId, Balance>,
}

impl<AccountId, Balance> Pallet<AccountId, Balance>
where
	AccountId: Ord + Clone,
	Balance: Zero + CheckedSub + CheckedAdd + Copy + Ord,
{
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn get_balance(&self, who: &AccountId) -> Balance {
		*self.balances.get(who).unwrap_or(&Balance::zero())
	}

	pub fn set_balance(&mut self, who: &AccountId, amount: Balance) {
		self.balances.insert(who.clone(), amount);
	}

	pub fn transfer(&mut self, who: &AccountId, to: &AccountId, amount: Balance) -> Result<(), &'static str> {
		let who_balance = self.get_balance(who);
		let to_balance = self.get_balance(to);

		if amount > who_balance {
			return Err("Not enough funds.");
		}

		let new_who_balance = who_balance.checked_sub(&amount).ok_or("Error: Overflow in subtracting balance")?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or("Error: Overflow in adding balance")?;

		// Subtract Balance in sender
		self.set_balance(who, new_who_balance);
		// Add balance to receiver
		self.set_balance(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*; // Importa todo desde el ámbito superior

	#[test]
	fn init_balances() {
		/* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
		let mut balances = Pallet::<&'static str, u128>::new();

		let alice = "alice";
		let bob = "bob";

		assert_eq!(balances.get_balance(&alice), 0);


		balances.set_balance(&alice, 100);

		assert_eq!(balances.get_balance(&alice), 100);
		assert_eq!(balances.get_balance(&bob), 0);
	}

    #[test]
	fn transfer_balance() {
        let mut balances = Pallet::<&'static str, u128>::new();
		let alice = "alice";
		let bob = "bob";

        balances.set_balance(&alice, 100);
        balances.set_balance(&bob, 0);

        // Test that Alice cannot transfer more funds than she has
        assert!(balances.transfer(&alice, &bob, 150).is_err());
        assert_eq!(balances.get_balance(&alice), 100);
        assert_eq!(balances.get_balance(&bob), 0);

        // Test that Alice can successfully transfer funds to Bob
        assert!(balances.transfer(&alice, &bob, 50).is_ok());
        assert_eq!(balances.get_balance(&alice), 50);
        assert_eq!(balances.get_balance(&bob), 50);

        // Further check that the balances are correctly updated after a transfer
        assert!(balances.transfer(&alice, &bob, 50).is_ok());
        assert_eq!(balances.get_balance(&alice), 0);
        assert_eq!(balances.get_balance(&bob), 100);

	} 
}
