//! Host-side `identity_lab` (IDL). Guest: `methods/guest/src/bin/identity_lab.rs`.
use super::age_proof::verify_age_receipt_passes;
use lez_framework::error::LezResult;
use lez_framework::prelude::*;
use lez_framework_core::types::LezOutput;
use nssa_core::account::AccountWithMetadata;
use nssa_core::program::AccountPostState;

#[lez_program]
mod identity_lab {
    #[allow(unused_imports)]
    use super::*;

    /// Verifies an `age_gte_18` RISC0 receipt (`proof` bytes) for the given `min_age`.
    #[instruction]
    pub fn verify_age_proof(
        #[account(signer)]
        authority: AccountWithMetadata,
        proof: Vec<u8>,
        min_age: u8,
    ) -> LezResult {
        verify_age_receipt_passes(&proof, min_age)?;

        Ok(LezOutput::states_only(vec![AccountPostState::new(
            authority.account.clone(),
        )]))
    }
}
