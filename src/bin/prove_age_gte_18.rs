//! Genera y verifica pruebas ZK locales para edad >= min (MVP: min = 18).

use clap::{Parser, Subcommand};
use example_program_deployment_methods::{AGE_GTE_18_ELF, AGE_GTE_18_ID};
use identity_lab::profile::{Policy, Profile};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn format_image_id(id: &[u32; 8]) -> String {
    id.iter()
        .map(|w| format!("{w:08x}"))
        .collect::<Vec<_>>()
        .join("")
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Lee profile.json, genera receipt + manifest en --proof-dir
    Prove {
        #[arg(long, default_value = "profile.json")]
        profile: PathBuf,
        #[arg(long, default_value = "18")]
        min_age: u8,
        #[arg(long, default_value = "proofs/age_gte_18")]
        proof_dir: PathBuf,
        /// Si el perfil no cumple la política, no generar (por defecto se genera con meets=0)
        #[arg(long)]
        require_pass: bool,
    },
    /// Verifica receipt.bin y muestra meets_policy (sin necesitar profile.json)
    Verify {
        #[arg(long, default_value = "proofs/age_gte_18")]
        proof_dir: PathBuf,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct ProofManifest {
    policy: String,
    min_age: u8,
    /// Salida pública del journal (0 o 1)
    meets_policy: u8,
    image_id: String,
    receipt_path: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    match Cli::parse().command {
        Command::Prove {
            profile,
            min_age,
            proof_dir,
            require_pass,
        } => prove(&profile, min_age, &proof_dir, require_pass),
        Command::Verify { proof_dir } => verify(&proof_dir),
    }
}

fn prove(
    profile_path: &PathBuf,
    min_age: u8,
    proof_dir: &PathBuf,
    require_pass: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let profile = Profile::from_json_file(profile_path)?;
    let policy = Policy::AgeGte { min: min_age };
    let expected = profile.satisfies(&policy);

    if require_pass && !expected {
        return Err(format!(
            "profile age {} does not satisfy >= {min_age} (use without --require-pass to build a proof with meets=0)",
            profile.age
        )
        .into());
    }

    let env = ExecutorEnv::builder()
        .write(&profile.age)?
        .write(&min_age)?
        .build()?;

    let prove_info = default_prover().prove(env, AGE_GTE_18_ELF)?;
    let receipt = prove_info.receipt;
    let meets_policy: u8 = receipt.journal.decode()?;

    std::fs::create_dir_all(proof_dir)?;
    let receipt_path = proof_dir.join("receipt.bin");
    let receipt_bytes = bincode::serialize(&receipt)?;
    std::fs::write(&receipt_path, &receipt_bytes)?;

    let manifest = ProofManifest {
        policy: format!("age_gte_{min_age}"),
        min_age,
        meets_policy,
        image_id: format_image_id(&AGE_GTE_18_ID),
        receipt_path: receipt_path.display().to_string(),
    };
    let manifest_path = proof_dir.join("manifest.json");
    std::fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest)?,
    )?;

    println!("proof written to {}", proof_dir.display());
    println!("  meets_policy: {meets_policy} (1 = passes age >= {min_age})");
    println!("  manifest: {}", manifest_path.display());
    println!("  receipt:  {} ({} bytes)", receipt_path.display(), receipt_bytes.len());
    println!("  image_id: {}", manifest.image_id);
    if meets_policy == 0 {
        println!("  note: proof is valid but policy is NOT satisfied (e.g. age < min)");
    }

    Ok(())
}

fn verify(proof_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let receipt_path = proof_dir.join("receipt.bin");
    let manifest_path = proof_dir.join("manifest.json");

    let receipt_bytes = std::fs::read(&receipt_path)?;
    let receipt: Receipt = bincode::deserialize(&receipt_bytes)?;
    receipt.verify(AGE_GTE_18_ID)?;

    let meets_policy: u8 = receipt.journal.decode()?;
    let manifest: ProofManifest = serde_json::from_slice(&std::fs::read(&manifest_path)?)?;

    println!("verify OK");
    println!("  meets_policy (from journal): {meets_policy}");
    println!("  min_age (manifest): {}", manifest.min_age);
    println!("  policy: {}", manifest.policy);
    println!("  image_id: {}", manifest.image_id);

    if meets_policy != manifest.meets_policy {
        return Err("manifest meets_policy does not match receipt journal".into());
    }

    Ok(())
}
