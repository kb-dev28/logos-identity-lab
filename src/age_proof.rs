//! Verificación host-side de receipts `age_gte_18` (RISC Zero).
//! Usado por `prove_age_gte_18` y por el runner `run_identity_lab`.

use example_program_deployment_methods::AGE_GTE_18_ID;
use lez_framework::error::LezError;
use risc0_zkvm::Receipt;

pub const ERR_PROOF_DECODE: u32 = 10;
pub const ERR_PROOF_CRYPTO: u32 = 11;
pub const ERR_JOURNAL_DECODE: u32 = 12;
pub const ERR_POLICY_NOT_MET: u32 = 13;
pub const ERR_MIN_AGE_MISMATCH: u32 = 14;

/// Image ID del circuito `age_gte_18` (debe coincidir con `methods` embed).
pub fn age_gte_18_image_id() -> &'static [u32; 8] {
    &AGE_GTE_18_ID
}

/// Valida `proof` (receipt bincode) y devuelve `meets_policy` del journal (0 o 1).
///
/// `min_age` es el umbral que la app verificadora exige (p. ej. 18). El receipt
/// solo compromete `meets` para el `min_age` usado al generar la prueba; en MVP
/// exigimos que coincida con el usado en `prove`.
pub fn verify_age_receipt_bytes(proof: &[u8], min_age: u8) -> Result<u8, LezError> {
    if proof.is_empty() {
        return Err(LezError::custom(ERR_PROOF_DECODE, "empty proof"));
    }

    let receipt: Receipt = bincode::deserialize(proof).map_err(|e| {
        LezError::custom(ERR_PROOF_DECODE, format!("invalid receipt encoding: {e}"))
    })?;

    receipt.verify(AGE_GTE_18_ID).map_err(|e| {
        LezError::custom(ERR_PROOF_CRYPTO, format!("receipt verification failed: {e}"))
    })?;

    let (proved_min_age, meets): (u8, u8) = receipt.journal.decode().map_err(|e| {
        LezError::custom(ERR_JOURNAL_DECODE, format!("journal decode failed: {e}"))
    })?;

    if proved_min_age != min_age {
        return Err(LezError::custom(
            ERR_MIN_AGE_MISMATCH,
            format!("proof min_age {proved_min_age} != requested {min_age}"),
        ));
    }

    Ok(meets)
}

/// Igual que [`verify_age_receipt_bytes`] pero exige `meets == 1`.
pub fn verify_age_receipt_passes(proof: &[u8], min_age: u8) -> Result<(), LezError> {
    let meets = verify_age_receipt_bytes(proof, min_age)?;
    if meets != 1 {
        return Err(LezError::custom(
            ERR_POLICY_NOT_MET,
            format!("age policy not satisfied (meets_policy={meets}, need 1)"),
        ));
    }
    Ok(())
}
