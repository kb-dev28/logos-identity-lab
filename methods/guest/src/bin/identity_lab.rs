//! Guest `identity_lab` — verifies age policy proofs (stub until F2.2).
#![no_main]

use lez_framework::error::{LezError, LezResult};
use lez_framework::prelude::*;

#[cfg(not(test))]
risc0_zkvm::guest::entry!(main);

#[lez_program]
mod identity_lab {
    #[allow(unused_imports)]
    use super::*;

    #[instruction]
    pub fn verify_age_proof(
        #[account(signer)]
        authority: AccountWithMetadata,
        proof: Vec<u8>,
        min_age: u8,
    ) -> LezResult {
        let _ = (authority, proof, min_age);
        Err(LezError::custom(
            1,
            "verify_age_proof: stub — on-chain verification in F2.2",
        ))
    }
}
