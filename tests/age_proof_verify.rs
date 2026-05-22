//! Tests F2.2: verificación de receipts válidos / inválidos.

use identity_lab::age_proof::{verify_age_receipt_bytes, verify_age_receipt_passes};

#[test]
fn verify_valid_and_invalid_proofs() {
    let valid = std::fs::read("proofs/age25/receipt.bin")
        .or_else(|_| std::fs::read("proofs/age_gte_18/receipt.bin"))
        .expect("run prove first to create proofs/age25 or proofs/age_gte_18");

    let meets = verify_age_receipt_bytes(&valid, 18).expect("valid proof");
    assert_eq!(meets, 1);
    verify_age_receipt_passes(&valid, 18).expect("passes");

    if let Ok(invalid) = std::fs::read("proofs/age16/receipt.bin") {
        let meets = verify_age_receipt_bytes(&invalid, 18).expect("crypto ok");
        assert_eq!(meets, 0);
        assert!(verify_age_receipt_passes(&invalid, 18).is_err());
    }
}
