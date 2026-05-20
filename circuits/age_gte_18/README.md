# Circuito `age_gte_18`

Prueba local (RISC Zero) de la política `Policy::AgeGte { min: 18 }` sin revelar la edad en el **journal** público.

| Pieza | Ruta |
|-------|------|
| Guest (zkVM) | `methods/guest/src/bin/age_gte_18.rs` |
| ELF embebido | `example_program_deployment_methods::AGE_GTE_18_ELF` |
| CLI prove/verify | `cargo run --bin prove_age_gte_18 -- …` |

## Uso

```bash
cp docs/examples/profile.example.json profile.json
# edita age

cargo run --bin prove_age_gte_18 -- prove --profile profile.json
cargo run --bin prove_age_gte_18 -- verify --proof-dir proofs/age_gte_18

# Simular app "solo +18" (falla si meets_policy == 0)
cargo run --bin prove_age_gte_18 -- verify --proof-dir proofs/age_gte_18 --require-pass
```

Salidas en `proofs/age_gte_18/`:

- `receipt.bin` — receipt RISC0 (bincode)
- `manifest.json` — metadatos legibles (`meets_policy`, `min_age`, `image_id`, …)

## Qué contiene la prueba

- **Journal (público):** solo `meets_policy` (0 o 1), no la edad.
- **Receipt:** datos criptográficos para verificar; el verificador no necesita `profile.json`.
- **No incluye:** `country_code`, nombre, ni la edad en claro en el manifest (solo `meets_policy`).
