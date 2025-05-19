//! Anomaly injection for zedi-gen

use crate::claims::Claim;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Types of anomalies that can be injected
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnomalyType {
    /// Missing required field
    MissingField,

    /// Invalid field value
    InvalidValue,

    /// Invalid date format
    InvalidDateFormat,

    /// Duplicate claim
    DuplicateClaim,

    /// Invalid procedure code
    InvalidProcedureCode,

    /// Invalid modifier combination
    InvalidModifierCombination,

    /// Age/gender mismatch for procedure
    AgeGenderMismatch,

    /// Invalid provider
    InvalidProvider,

    /// Invalid patient information
    InvalidPatientInfo,

    /// Missing documentation
    MissingDocumentation,
}

/// Anomaly injection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyConfig {
    /// Base anomaly rate (0.0 to 1.0)
    pub base_rate: f64,

    /// Per-anomaly-type rates
    pub type_rates: std::collections::HashMap<AnomalyType, f64>,

    /// Whether to log injected anomalies
    pub log_anomalies: bool,
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        let mut type_rates = std::collections::HashMap::new();

        // Set default rates for each anomaly type
        type_rates.insert(AnomalyType::MissingField, 0.3);
        type_rates.insert(AnomalyType::InvalidValue, 0.2);
        type_rates.insert(AnomalyType::InvalidDateFormat, 0.1);
        type_rates.insert(AnomalyType::DuplicateClaim, 0.05);
        type_rates.insert(AnomalyType::InvalidProcedureCode, 0.15);
        type_rates.insert(AnomalyType::InvalidModifierCombination, 0.1);
        type_rates.insert(AnomalyType::AgeGenderMismatch, 0.05);
        type_rates.insert(AnomalyType::InvalidProvider, 0.1);
        type_rates.insert(AnomalyType::InvalidPatientInfo, 0.2);
        type_rates.insert(AnomalyType::MissingDocumentation, 0.1);

        Self {
            base_rate: 0.01, // 1% base rate
            type_rates,
            log_anomalies: true,
        }
    }
}

/// Result of anomaly injection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyInjectionResult {
    /// The claim with anomalies injected
    pub claim: Claim,

    /// List of anomalies that were injected
    pub anomalies: Vec<Anomaly>,
}

/// Represents an injected anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    /// Type of anomaly
    pub anomaly_type: AnomalyType,

    /// Description of the anomaly
    pub description: String,

    /// Field affected by the anomaly (if applicable)
    pub field: Option<String>,

    /// Original value (if applicable)
    pub original_value: Option<String>,

    /// New value (if applicable)
    pub new_value: Option<String>,
}

/// Injects anomalies into claims
pub struct AnomalyInjector {
    rng: rand_chacha::ChaCha8Rng,
    config: AnomalyConfig,
    seen_claims: HashSet<String>,
}

impl AnomalyInjector {
    /// Create a new AnomalyInjector with the given configuration and optional seed
    pub fn new(config: AnomalyConfig, seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => rand_chacha::ChaCha8Rng::seed_from_u64(seed),
            None => rand_chacha::ChaCha8Rng::from_entropy(),
        };

        Self {
            rng,
            config,
            seen_claims: HashSet::new(),
        }
    }

    /// Inject anomalies into a claim
    pub fn inject_anomalies(&mut self, mut claim: Claim) -> AnomalyInjectionResult {
        let mut anomalies = Vec::new();

        // Determine if we should inject anomalies at all based on base_rate
        if self.rng.gen::<f64>() >= self.config.base_rate {
            return AnomalyInjectionResult { claim, anomalies };
        }

        // Clone the type_rates to avoid borrowing issues
        let type_rates = self.config.type_rates.clone();

        // Inject each anomaly type based on its configured rate
        for (&anomaly_type, &rate) in &type_rates {
            if self.rng.gen::<f64>() < rate {
                if let Some(anomaly) = self.inject_anomaly(anomaly_type, &mut claim) {
                    anomalies.push(anomaly);
                }
            }
        }

        let result = AnomalyInjectionResult { claim, anomalies };
        if self.config.log_anomalies {
            for anomaly in &result.anomalies {
                eprintln!("Injected anomaly: {:?}", anomaly);
            }
        }
        result
    }

    /// Inject a specific type of anomaly into a claim
    fn inject_anomaly(&mut self, anomaly_type: AnomalyType, claim: &mut Claim) -> Option<Anomaly> {
        match anomaly_type {
            AnomalyType::MissingField => self.inject_missing_field(claim),
            AnomalyType::InvalidValue => self.inject_invalid_value(claim),
            AnomalyType::InvalidDateFormat => self.inject_invalid_date_format(claim),
            AnomalyType::DuplicateClaim => self.inject_duplicate_claim(claim),
            AnomalyType::InvalidProcedureCode => self.inject_invalid_procedure_code(claim),
            AnomalyType::InvalidModifierCombination => {
                self.inject_invalid_modifier_combination(claim)
            }
            AnomalyType::AgeGenderMismatch => self.inject_age_gender_mismatch(claim),
            AnomalyType::InvalidProvider => self.inject_invalid_provider(claim),
            AnomalyType::InvalidPatientInfo => self.inject_invalid_patient_info(claim),
            AnomalyType::MissingDocumentation => self.inject_missing_documentation(claim),
        }
    }

    // Implementation of each anomaly injection method...

    fn inject_missing_field(&self, claim: &mut Claim) -> Option<Anomaly> {
        let original = claim.claim_id.clone();
        claim.claim_id.clear();
        Some(Anomaly {
            anomaly_type: AnomalyType::MissingField,
            description: "Missing required claim_id".to_string(),
            field: Some("claim_id".to_string()),
            original_value: Some(original.clone()),
            new_value: Some(claim.claim_id.clone()),
        })
    }

    fn inject_invalid_value(&self, claim: &mut Claim) -> Option<Anomaly> {
        let original = claim.total_payment;
        claim.total_payment = claim.total_charge.saturating_add(1000);
        Some(Anomaly {
            anomaly_type: AnomalyType::InvalidValue,
            description: "Payment exceeds charge".to_string(),
            field: Some("total_payment".to_string()),
            original_value: Some(original.to_string()),
            new_value: Some(claim.total_payment.to_string()),
        })
    }

    fn inject_invalid_date_format(&self, claim: &mut Claim) -> Option<Anomaly> {
        if let Some(line) = claim.service_lines.get_mut(0) {
            let original = line.service_date.clone();
            line.service_date = "01/01/1970".to_string();
            return Some(Anomaly {
                anomaly_type: AnomalyType::InvalidDateFormat,
                description: "Invalid service date format".to_string(),
                field: Some("service_date".to_string()),
                original_value: Some(original),
                new_value: Some(line.service_date.clone()),
            });
        }
        None
    }

    fn inject_duplicate_claim(&mut self, claim: &mut Claim) -> Option<Anomaly> {
        if self.seen_claims.contains(&claim.claim_id) {
            // This is already a duplicate, don't inject another duplicate anomaly
            return None;
        }

        self.seen_claims.insert(claim.claim_id.clone());

        Some(Anomaly {
            anomaly_type: AnomalyType::DuplicateClaim,
            description: "Duplicate claim ID".to_string(),
            field: Some("claim_id".to_string()),
            original_value: Some(claim.claim_id.clone()),
            new_value: None,
        })
    }

    fn inject_invalid_procedure_code(&self, claim: &mut Claim) -> Option<Anomaly> {
        if let Some(line) = claim.service_lines.get_mut(0) {
            let original = line.procedure_code.clone();
            line.procedure_code = "INVALID_CODE".to_string();
            return Some(Anomaly {
                anomaly_type: AnomalyType::InvalidProcedureCode,
                description: "Invalid procedure code".to_string(),
                field: Some("procedure_code".to_string()),
                original_value: Some(original),
                new_value: Some(line.procedure_code.clone()),
            });
        }
        None
    }

    fn inject_invalid_modifier_combination(&self, claim: &mut Claim) -> Option<Anomaly> {
        if let Some(line) = claim.service_lines.get_mut(0) {
            let original = line.modifiers.clone();
            line.modifiers = vec!["AAA".to_string(), "AAA".to_string()];
            return Some(Anomaly {
                anomaly_type: AnomalyType::InvalidModifierCombination,
                description: "Invalid modifier combination".to_string(),
                field: Some("modifiers".to_string()),
                original_value: Some(format!("{:?}", original)),
                new_value: Some(format!("{:?}", line.modifiers)),
            });
        }
        None
    }

    fn inject_age_gender_mismatch(&self, claim: &mut Claim) -> Option<Anomaly> {
        let original = claim.patient.gender.clone();
        claim.patient.gender = match original.to_uppercase().as_str() {
            "M" => "F".to_string(),
            _ => "M".to_string(),
        };
        Some(Anomaly {
            anomaly_type: AnomalyType::AgeGenderMismatch,
            description: "Age/gender mismatch for procedure".to_string(),
            field: Some("patient.gender".to_string()),
            original_value: Some(original),
            new_value: Some(claim.patient.gender.clone()),
        })
    }

    fn inject_invalid_provider(&self, claim: &mut Claim) -> Option<Anomaly> {
        let original = claim.billing_provider.npi.clone();
        claim.billing_provider.npi = "000".to_string();
        Some(Anomaly {
            anomaly_type: AnomalyType::InvalidProvider,
            description: "Invalid provider NPI".to_string(),
            field: Some("billing_provider.npi".to_string()),
            original_value: Some(original),
            new_value: Some(claim.billing_provider.npi.clone()),
        })
    }

    fn inject_invalid_patient_info(&self, claim: &mut Claim) -> Option<Anomaly> {
        let original = claim.patient.date_of_birth.clone();
        claim.patient.date_of_birth = "2010-13-40".to_string();
        Some(Anomaly {
            anomaly_type: AnomalyType::InvalidPatientInfo,
            description: "Invalid patient date of birth".to_string(),
            field: Some("patient.date_of_birth".to_string()),
            original_value: Some(original),
            new_value: Some(claim.patient.date_of_birth.clone()),
        })
    }

    fn inject_missing_documentation(&self, _claim: &mut Claim) -> Option<Anomaly> {
        Some(Anomaly {
            anomaly_type: AnomalyType::MissingDocumentation,
            description: "Missing documentation for claim".to_string(),
            field: None,
            original_value: None,
            new_value: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::population::PopulationGenerator;

    #[test]
    fn test_anomaly_injection() {
        let config = AnomalyConfig {
            base_rate: 1.0, // 100% chance to inject anomalies
            type_rates: vec![(AnomalyType::DuplicateClaim, 1.0)]
                .into_iter()
                .collect(),
            log_anomalies: true,
        };

        let mut injector = AnomalyInjector::new(config, Some(42));

        // Generate a test claim
        let mut pop_gen = PopulationGenerator::new(Some(42));
        let patient = pop_gen.generate_person();
        let provider = pop_gen.generate_provider();
        let mut claim_gen = super::super::claims::ClaimGenerator::new(Some(42));
        let claim = claim_gen.generate_claim(patient, provider, None);

        // Inject anomalies
        let result = injector.inject_anomalies(claim);

        // Should have at least one anomaly
        assert!(!result.anomalies.is_empty());

        // Check if the anomaly is logged
        if let Some(anomaly) = result.anomalies.first() {
            assert_eq!(anomaly.anomaly_type, AnomalyType::DuplicateClaim);
            assert_eq!(anomaly.description, "Duplicate claim ID");
            assert_eq!(anomaly.field, Some("claim_id".to_string()));
        } else {
            panic!("Expected at least one anomaly");
        }
    }
}
