use core::fmt::Debug;
use std::collections::BTreeMap;
use crate::support::{self, Dispatch};

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
    claims: BTreeMap<T::Content, T::AccountId>
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
        Self { claims: BTreeMap::new() }
	}

    pub fn get_claim(&self, content: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(content)
    }

    pub fn create_claim(&mut self, caller: T::AccountId, content: T::Content) -> support::DispatchResult {
        if self.claims.contains_key(&content) {
            return Err("Property already claim");
        }

        self.claims.insert(content, caller);

        Ok(())
    }
    

    pub fn revoke_claim(&mut self, caller: &T::AccountId, content: &T::Content) -> support::DispatchResult {
        let owner = self.get_claim(content).ok_or("Content does not exist")?;

        if owner != caller {
            return Err("You are not the owner");
        }

        self.claims.remove(content);

        Ok(())
    }
}

pub enum Call<T: Config> {
    CreateClaim {
        content: T::Content,
    },
    RevokeClaim {
        content: T::Content,
    }
}

impl<T: Config> Dispatch for Pallet<T> {
    type Call = Call<T>;
    type Caller = T::AccountId;
    
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
        match call {
            Call::CreateClaim { content } => self.create_claim(caller, content)?,
            Call::RevokeClaim { content } => self.revoke_claim(&caller, &content)?,
        };
        Ok(())
    }
}
