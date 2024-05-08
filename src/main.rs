mod balances;
mod system;
mod support;

use crate::support::Dispatch;

mod types {
    use crate::{support, RuntimeCall};

	pub type AccountId = String;
	pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = support::Extrinsic<AccountId, RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    BalancesTransfer { to: types::AccountId, amount: types::Balance },
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}

    // Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.system.inc_block_number();

        if block.header.block_number != self.system.block_number() {
			return Err(&"block number does not match what is expected")
		}

        for (i, el) in block.extrinsics.into_iter().enumerate() {
            let caller = el.caller;
            let call = el.call;

            self.system.inc_nonce(&caller);

            let _res = self.dispatch(caller, call).map_err(|e| eprintln!(
                "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                block.header.block_number, i, e
            ));
        }
        
		Ok(())
	}
}

impl system::Config for Runtime {
	type AccountId = types::AccountId;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
        match call {
            RuntimeCall::BalancesTransfer { to, amount } => {
                self.balances.transfer(&caller, &to, amount)?;
            }
        }
        Ok(())
    }
}

fn main() {
	/* TODO: Create a mutable variable `runtime`, which is a new instance of `Runtime`. */
	/* TODO: Set the balance of `alice` to 100, allowing us to execute other transactions. */
    let mut runtime = Runtime::new();
    runtime.balances.set_balance(&"alice".to_string(), 100);

	// start emulating a block
	/* TODO: Increment the block number in system. */
	/* TODO: Assert the block number is what we expect. */
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	/* TODO: Increment the nonce of `alice`. */
	/* TODO: Execute a transfer from `alice` to `bob` for 30 tokens.
		- The transfer _could_ return an error. We should use `map_err` to print
		  the error if there is one.
		- We should capture the result of the transfer in an unused variable like `_res`.
	*/
    runtime.system.inc_nonce(&"alice".to_string());
    let _res = runtime.balances.transfer(&"alice".to_string(), &"bob".to_string(), 30).map_err(|e| eprintln!("{}", e));
    
	// second transaction
	/* TODO: Increment the nonce of `alice` again. */
	/* TODO: Execute another balance transfer, this time from `alice` to `charlie` for 20. */
    runtime.system.inc_nonce(&"alice".to_string());
    let _res = runtime.balances.transfer(&"alice".to_string(), &"charlie".to_string(), 20).map_err(|e| eprintln!("{}", e));


    println!("{:#?}", runtime);
}