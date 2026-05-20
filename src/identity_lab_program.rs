//! Host-side `identity_lab` (IDL). Guest: `methods/guest/src/bin/identity_lab.rs`.
use lez_framework::error::{LezError, LezResult};
use lez_framework::prelude::*;
use nssa_core::account::AccountWithMetadata;

/// Stub error code until F2.2 implements receipt verification in-guest.
pub const VERIFY_AGE_PROOF_STUB_CODE: u32 = 1;

#[lez_program]
mod identity_lab {
    #[allow(unused_imports)]
    use super::*;

    /// Verifies an age-gte proof (RISC0 receipt bytes) against `min_age`.
    ///
    /// F2.2: deserialize `proof`, check image ID + journal `meets_policy` for `min_age`.
    #[instruction]
    pub fn verify_age_proof(
        #[account(signer)]
        authority: AccountWithMetadata,
        proof: Vec<u8>,
        min_age: u8,
    ) -> LezResult {
        let _ = (authority, proof, min_age);
        Err(LezError::custom(
            VERIFY_AGE_PROOF_STUB_CODE,
            "verify_age_proof: stub — on-chain verification in F2.2",
        ))
    }
}
