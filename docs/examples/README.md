# Perfil local (v1)

## Archivos

| Archivo | En git | Uso |
|---------|--------|-----|
| `profile.example.json` | Sí | Plantilla copiable; valores de demo |
| `profile.json` (raíz del proyecto) | No (`.gitignore`) | Tu perfil real en desarrollo |

Copia la plantilla:

```bash
cp docs/examples/profile.example.json profile.json
# edita age y country_code
```

## Esquema v1 (`profile.schema.v1`)

| Campo | Tipo | Reglas |
|-------|------|--------|
| `age` | entero | `0..=150` — edad en años (autodeclarada) |
| `country_code` | string | ISO 3166-1 alpha-2 en mayúsculas (`MX`, `AR`, `DE`, …) |

Ejemplo válido:

```json
{
  "age": 25,
  "country_code": "MX"
}
```

## Políticas MVP

Las políticas son lo que **otra app pide verificar**. No se guardan en el perfil; se aplican al generar una prueba ZK.

| Política | ID sugerido | Regla | Fase |
|----------|-------------|-------|------|
| Mayor de edad | `age_gte_18` | `age >= 18` | F1 (P0) |
| Residencia LATAM | `country_in_latam` | `country_code ∈ LATAM_ISO` | F5 stretch |

Lista LATAM (v1, fija para circuitos): ver `LATAM_COUNTRY_CODES` en `src/profile.rs`.

## Relación perfil → prueba

1. **Perfil** = datos privados locales (`profile.json`).
2. **Política** = pregunta pública (`¿cumple >= 18?`).
3. **Prueba** = artefacto en `proofs/age_gte_18/` (`receipt.bin` + `manifest.json`).

```bash
cargo run --bin prove_age_gte_18 -- prove --profile profile.json
cargo run --bin prove_age_gte_18 -- verify --proof-dir proofs/age_gte_18
```

Detalle del circuito: [`circuits/age_gte_18/README.md`](../circuits/age_gte_18/README.md).

Etapa 2 añadirá credenciales firmadas por emisores antes del paso ZK; el esquema de perfil puede ganar campos `credentials[]` sin romper `age` / `country_code`.
