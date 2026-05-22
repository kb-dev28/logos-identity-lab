//! Runner CLI: prueba local de `verify_age_receipt` (F2.2) antes de enviar tx LEZ.

use clap::Parser;
use identity_lab::age_proof::{verify_age_receipt_bytes, verify_age_receipt_passes};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long, default_value = "proofs/age_gte_18")]
    proof_dir: PathBuf,
    #[arg(long, default_value = "18")]
    min_age: u8,
    #[arg(long)]
    require_pass: bool,
}

fn main() {
    let cli = Cli::parse();
    let receipt_path = cli.proof_dir.join("receipt.bin");
    let proof = std::fs::read(&receipt_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", receipt_path.display()));

    let result = if cli.require_pass {
        verify_age_receipt_passes(&proof, cli.min_age).map(|_| 1u8)
    } else {
        verify_age_receipt_bytes(&proof, cli.min_age)
    };

    match result {
        Ok(meets) => {
            println!("verify_age_receipt OK");
            println!("  meets_policy: {meets}");
            if meets == 1 {
                println!("  access: GRANTED");
            } else {
                println!("  access: DENIED");
            }
        }
        Err(e) => {
            eprintln!("verify_age_receipt FAILED: {e}");
            std::process::exit(1);
        }
    }
}
