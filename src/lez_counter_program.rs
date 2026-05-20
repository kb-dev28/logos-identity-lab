//! Host-side `lez_counter` (IDL + tests). Guest: `methods/guest/src/bin/lez_counter.rs`.
use lez_framework::prelude::*;
use lez_framework::error::LezResult;
use lez_framework_core::types::LezOutput;
use nssa_core::account::AccountWithMetadata;
use nssa_core::program::AccountPostState;

#[lez_program]
mod lez_counter {
    #[allow(unused_imports)]
    use super::*;

    #[instruction]
    pub fn initialize(
        #[account(init, pda = literal("counter"))]
        counter: AccountWithMetadata,
        #[account(signer)]
        authority: AccountWithMetadata,
    ) -> LezResult {
        Ok(LezOutput::states_only(vec![
            AccountPostState::new_claimed(counter.account.clone()),
            AccountPostState::new(authority.account.clone()),
        ]))
    }

    #[instruction]
    pub fn increment(
        #[account(mut, pda = literal("counter"))]
        counter: AccountWithMetadata,
        #[account(signer)]
        authority: AccountWithMetadata,
        amount: u64,
    ) -> LezResult {
        let mut counter_post = counter.account.clone();
        counter_post.balance += amount as u128;

        Ok(LezOutput::states_only(vec![
            AccountPostState::new(counter_post),
            AccountPostState::new(authority.account.clone()),
        ]))
    }
}
