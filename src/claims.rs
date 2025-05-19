//! X12 835 claim generation for zedi-gen

use crate::population::{Person, Provider};
use chrono::Utc;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::Path;

use csv;

/// Represents an X12 835 claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    /// Claim identifier
    pub claim_id: String,

    /// Patient information
    pub patient: Person,

    /// Billing provider
    pub billing_provider: Provider,

    /// Rendering provider (if different from billing provider)
    pub rendering_provider: Option<Provider>,

    /// Service lines
    pub service_lines: Vec<ServiceLine>,

    /// Total charge amount (in cents)
    pub total_charge: u64,

    /// Total payment amount (in cents)
    pub total_payment: u64,

    /// Total adjustment amount (in cents)
    pub total_adjustment: u64,

    /// Patient responsibility amount (in cents)
    pub patient_responsibility: u64,

    /// Claim status (paid, denied, etc.)
    pub status: ClaimStatus,
}

/// Represents a service line in a claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLine {
    /// Line item control number
    pub line_number: u32,

    /// Procedure code (CPT/HCPCS)
    pub procedure_code: String,

    /// Procedure description
    pub procedure_description: String,

    /// Service date (YYYY-MM-DD)
    pub service_date: String,

    /// Charge amount (in cents)
    pub charge_amount: u64,

    /// Payment amount (in cents)
    pub payment_amount: u64,

    /// Paid amount (in cents)
    pub paid_amount: u64,

    /// Adjustment amount (in cents)
    pub adjustment_amount: u64,

    /// Units of service
    pub units: f64,

    /// Place of service code
    pub place_of_service: String,

    /// Revenue code (for institutional claims)
    pub revenue_code: Option<String>,

    /// Modifiers (if any)
    pub modifiers: Vec<String>,
}

/// Claim status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClaimStatus {
    /// Paid in full
    Paid,

    /// Denied
    Denied,

    /// Partially paid
    Partial,

    /// Pending
    Pending,
}

/// Generates X12 835 claims
pub struct ClaimGenerator {
    rng: rand_chacha::ChaCha8Rng,
    procedure_codes: Vec<ProcedureCode>,
    modifiers: Vec<String>,
    place_of_service_codes: HashMap<String, String>,
}

/// Represents a procedure code with its description
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProcedureCode {
    code: String,
    description: String,
    typical_charge: u64,
    typical_units: f64,
}

impl ClaimGenerator {
    /// Create a new claim generator with an optional seed
    pub fn new(seed: Option<u64>) -> Self {
        use rand_chacha::rand_core::SeedableRng;

        let rng = if let Some(seed) = seed {
            rand_chacha::ChaCha8Rng::seed_from_u64(seed)
        } else {
            rand_chacha::ChaCha8Rng::from_entropy()
        };

        // Load real procedure codes, modifiers, and POS codes from files (fallback to defaults)
        let data_dir = env::var("ZEDI_GEN_DATA_DIR").unwrap_or_else(|_| "data".to_string());
        let data_path = Path::new(&data_dir);

        // Procedure codes
        #[derive(Debug, Deserialize)]
        struct CsvProcedureCode {
            code: String,
            description: String,
            typical_charge: u64,
            typical_units: f64,
        }

        let mut procedure_codes = Vec::new();
        if let Ok(mut rdr) = csv::Reader::from_path(data_path.join("procedure_codes.csv")) {
            for result in rdr.deserialize() {
                if let Ok(rec) = result {
                    let rec: CsvProcedureCode = rec;
                    procedure_codes.push(ProcedureCode {
                        code: rec.code,
                        description: rec.description,
                        typical_charge: rec.typical_charge,
                        typical_units: rec.typical_units,
                    });
                }
            }
        }
        if procedure_codes.is_empty() {
            procedure_codes.push(ProcedureCode {
                code: "99213".to_string(),
                description: "Office or other outpatient visit for the evaluation and management of an established patient".to_string(),
                typical_charge: 15000,
                typical_units: 1.0,
            });
        }

        // Modifiers
        let mut modifiers = Vec::new();
        if let Ok(mut rdr) = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(data_path.join("modifiers.csv"))
        {
            for result in rdr.records() {
                if let Ok(record) = result {
                    if let Some(m) = record.get(0) {
                        modifiers.push(m.to_string());
                    }
                }
            }
        }
        if modifiers.is_empty() {
            modifiers = vec![
                "25".to_string(),
                "59".to_string(),
                "LT".to_string(),
                "RT".to_string(),
            ];
        }

        // Place of Service codes
        #[derive(Debug, Deserialize)]
        struct CsvPosCode {
            pos: String,
            description: String,
        }

        let mut place_of_service_codes = HashMap::new();
        if let Ok(mut rdr) = csv::Reader::from_path(data_path.join("pos_codes.csv")) {
            for result in rdr.deserialize() {
                if let Ok(rec) = result {
                    let rec: CsvPosCode = rec;
                    place_of_service_codes.insert(rec.pos, rec.description);
                }
            }
        }
        if place_of_service_codes.is_empty() {
            place_of_service_codes.insert("11".to_string(), "Office".to_string());
            place_of_service_codes.insert("21".to_string(), "Inpatient Hospital".to_string());
            place_of_service_codes.insert("22".to_string(), "Outpatient Hospital".to_string());
            place_of_service_codes.insert("23".to_string(), "Emergency Room".to_string());
        }

        Self {
            rng,
            procedure_codes,
            modifiers,
            place_of_service_codes,
        }
    }

    /// Generate a synthetic claim
    pub fn generate_claim(
        &mut self,
        patient: Person,
        billing_provider: Provider,
        rendering_provider: Option<Provider>,
    ) -> Claim {
        let claim_id = format!("CLM{:08}", self.rng.gen_range(10000000..=99999999));

        // Generate 1-5 service lines per claim
        let num_service_lines = self.rng.gen_range(1..=5);
        let mut service_lines = Vec::with_capacity(num_service_lines as usize);

        let mut total_charge = 0;
        let mut total_payment = 0;
        let mut total_adjustment = 0;

        for i in 0..num_service_lines {
            let procedure = self.procedure_codes.choose(&mut self.rng).unwrap();

            let charge_amount = procedure.typical_charge;
            let payment_amount = (charge_amount as f64 * self.rng.gen_range(0.5..1.0)) as u64;
            let adjustment_amount = charge_amount - payment_amount;

            total_charge += charge_amount;
            total_payment += payment_amount;
            total_adjustment += adjustment_amount;

            // Randomly add 0-2 modifiers
            let num_modifiers = self.rng.gen_range(0..=2);
            let modifiers = self
                .modifiers
                .choose_multiple(&mut self.rng, num_modifiers as usize)
                .cloned()
                .collect();

            let service_line = ServiceLine {
                line_number: i + 1,
                procedure_code: procedure.code.clone(),
                procedure_description: procedure.description.clone(),
                service_date: Utc::now().format("%Y-%m-%d").to_string(),
                charge_amount,
                payment_amount,
                paid_amount: payment_amount,
                adjustment_amount,
                units: procedure.typical_units,
                place_of_service: "11".to_string(), // Office
                revenue_code: None, // Will be set based on the procedure code if needed
                modifiers,
            };

            service_lines.push(service_line);
        }

        // Determine claim status based on payment
        let status = if total_payment == 0 {
            ClaimStatus::Denied
        } else if total_payment < total_charge {
            ClaimStatus::Partial
        } else {
            ClaimStatus::Paid
        };

        Claim {
            claim_id,
            patient,
            billing_provider,
            rendering_provider,
            service_lines,
            total_charge,
            total_payment,
            total_adjustment,
            patient_responsibility: (total_charge - total_payment) / 2, // 50% patient responsibility
            status,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claim_generation() {
        use crate::population::PopulationGenerator;

        let mut claim_gen = ClaimGenerator::new(Some(42));
        let mut pop_gen = PopulationGenerator::new(Some(42));

        let patient = pop_gen.generate_person();
        let provider = pop_gen.generate_provider();

        let claim = claim_gen.generate_claim(patient, provider, None);

        assert!(!claim.claim_id.is_empty());
        assert!(!claim.service_lines.is_empty());
        assert!(claim.total_charge > 0);
        assert!(claim.patient_responsibility <= claim.total_charge - claim.total_payment);
    }
}
