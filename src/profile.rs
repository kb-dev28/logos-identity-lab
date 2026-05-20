//! Perfil autodeclarado (v1) y políticas de verificación para identity-lab.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Perfil local v1 — el usuario lo edita a mano o desde la UI (futuro).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Profile {
    pub age: u8,
    pub country_code: String,
}

/// Política que una app verificadora puede solicitar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Policy {
    /// Edad mínima en años (MVP: `min: 18`).
    AgeGte { min: u8 },
    /// País pertenece a la lista LATAM fija (stretch F5).
    CountryInLatam,
}

impl Profile {
    pub fn from_json_file(path: impl AsRef<Path>) -> Result<Self, Box<dyn std::error::Error>> {
        let bytes = std::fs::read(path.as_ref())?;
        let profile: Self = serde_json::from_slice(&bytes)?;
        profile.validate()?;
        Ok(profile)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.age > 150 {
            return Err(format!("age must be 0..=150, got {}", self.age));
        }
        if self.country_code.len() != 2 {
            return Err(format!(
                "country_code must be ISO 3166-1 alpha-2, got {:?}",
                self.country_code
            ));
        }
        if !self
            .country_code
            .chars()
            .all(|c| c.is_ascii_alphabetic())
        {
            return Err(format!(
                "country_code must be ASCII letters, got {:?}",
                self.country_code
            ));
        }
        Ok(())
    }

    /// Evalúa si el perfil cumple la política (lógica clara; la ZK la replicará en F1).
    pub fn satisfies(&self, policy: &Policy) -> bool {
        match policy {
            Policy::AgeGte { min } => self.age >= *min,
            Policy::CountryInLatam => LATAM_COUNTRY_CODES.contains(&self.country_code.as_str()),
        }
    }
}

impl Policy {
    pub fn age_gte_18() -> Self {
        Self::AgeGte { min: 18 }
    }
}

/// ISO 3166-1 alpha-2 — lista cerrada para MVP LATAM (F5).
pub const LATAM_COUNTRY_CODES: &[&str] = &[
    "AR", "BO", "BR", "CL", "CO", "CR", "CU", "DO", "EC", "SV", "GT", "HN", "MX", "NI", "PA",
    "PY", "PE", "PR", "UY", "VE",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_profile_satisfies_age_gte_18() {
        let p = Profile {
            age: 25,
            country_code: "MX".into(),
        };
        assert!(p.satisfies(&Policy::age_gte_18()));
    }

    #[test]
    fn minor_fails_age_gte_18() {
        let p = Profile {
            age: 16,
            country_code: "MX".into(),
        };
        assert!(!p.satisfies(&Policy::age_gte_18()));
    }

    #[test]
    fn latam_policy() {
        let p = Profile {
            age: 30,
            country_code: "MX".into(),
        };
        assert!(p.satisfies(&Policy::CountryInLatam));
        let de = Profile {
            age: 30,
            country_code: "DE".into(),
        };
        assert!(!de.satisfies(&Policy::CountryInLatam));
    }
}
