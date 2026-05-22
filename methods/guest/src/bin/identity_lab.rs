//! Guest `identity_lab` — verifica receipts del circuito `age_gte_18`.
#![no_main]

use example_program_deployment_programs::age_gte_18_id::AGE_GTE_18_IMAGE_ID;
use lez_framework::error::{LezError, LezResult};
use lez_framework::prelude::*;
use lez_framework_core::types::LezOutput;
use nssa_core::program::AccountPostState;
use risc0_zkvm::Receipt;

#[cfg(not(test))]
risc0_zkvm::guest::entry!(main);

const ERR_PROOF_DECODE: u32 = 10;
const ERR_PROOF_CRYPTO: u32 = 11;
const ERR_JOURNAL_DECODE: u32 = 12;
const ERR_POLICY_NOT_MET: u32 = 13;
const ERR_MIN_AGE_MISMATCH: u32 = 14;

fn verify_age_receipt(proof: &[u8], min_age: u8) -> Result<(), LezError> {
    if proof.is_empty() {
        return Err(LezError::custom(ERR_PROOF_DECODE, "empty proof"));
    }

    let receipt: Receipt = bincode::deserialize(proof).map_err(|e| {
        LezError::custom(ERR_PROOF_DECODE, format!("invalid receipt: {e}"))
    })?;

    receipt.verify(AGE_GTE_18_IMAGE_ID).map_err(|e| {
        LezError::custom(ERR_PROOF_CRYPTO, format!("receipt verify failed: {e}"))
    })?;

    let (proved_min_age, meets): (u8, u8) = receipt.journal.decode().map_err(|e| {
        LezError::custom(ERR_JOURNAL_DECODE, format!("journal decode: {e}"))
    })?;

    if proved_min_age != min_age {
        return Err(LezError::custom(
            ERR_MIN_AGE_MISMATCH,
            format!("proof min_age {proved_min_age} != requested {min_age}"),
        ));
    }

    if meets != 1 {
        return Err(LezError::custom(
            ERR_POLICY_NOT_MET,
            "age policy not satisfied (meets_policy=0)",
        ));
    }

    Ok(())
}

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
        verify_age_receipt(&proof, min_age)?;

        Ok(LezOutput::states_only(vec![AccountPostState::new(
            authority.account.clone(),
        )]))
    }
}
