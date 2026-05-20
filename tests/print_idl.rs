//! Emits IDL JSON for `lgs build idl` (`cargo test __lssa_idl_print`).

#[test]
fn __lssa_idl_print() {
    println!("--- LSSA IDL BEGIN lez_counter ---");
    println!("{}", identity_lab::lez_counter_program::PROGRAM_IDL_JSON);
    println!("--- LSSA IDL END lez_counter ---");
    println!("--- LSSA IDL BEGIN identity_lab ---");
    println!("{}", identity_lab::identity_lab_program::PROGRAM_IDL_JSON);
    println!("--- LSSA IDL END identity_lab ---");
}
