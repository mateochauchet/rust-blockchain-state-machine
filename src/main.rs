mod balances;
mod system;
mod support;
mod proof_of_existence;

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
    pub type content = &'static str;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
    BalancesTransfer { to: types::AccountId, amount: types::Balance },
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existance: proof_of_existence::Pallet<Self>,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new(), proof_of_existance: proof_of_existence::Pallet::new() }
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

impl proof_of_existence::Config for Runtime {
    type Content = types::content;
}

impl support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
        match call {
            RuntimeCall::BalancesTransfer { to, amount } => {
                self.balances.transfer(&caller, &to, amount)?;
            },
            RuntimeCall::Balances(call) => {
                self.balances.dispatch(caller, call)?;
            },
            RuntimeCall::ProofOfExistence(call) => {
                self.proof_of_existance.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

fn main() {
	// Create a new instance of the Runtime.
	// It will instantiate with it all the modules it uses.
	let mut runtime = Runtime::new();

	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

    // Initialize the system with some initial balance.
	runtime.balances.set_balance(&alice, 100);


    let first_transfer = support::Extrinsic {
        caller: alice.clone(),
        call: RuntimeCall::Balances(balances::Call::Transfer { to: bob, amount: 10 }),
    };

    let claim_item = support::Extrinsic {
        caller: alice.clone(),
        call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim { content: "messi" }),
    };

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            first_transfer,
            claim_item
        ],
    };


    let _ = runtime.execute_block(block_1).expect("invalid block");

	println!("{:#?}", runtime);
}