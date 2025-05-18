//! Main generator module for zedi-gen

use crate::anomalies::{Anomaly, AnomalyConfig, AnomalyInjector, AnomalyInjectionResult};
use crate::claims::{Claim, ClaimGenerator};
use crate::config::{Config, OutputFormat};
use crate::population::PopulationGenerator;

// Import x12 types using standard module paths
use crate::x12::envelope::{X12Interchange, FunctionalGroup, TransactionSet};
use crate::x12::segments::{BprSegment, ClpSegment, DtmSegment, N1Segment, SvcSegment, TrnSegment};
use chrono::Utc;
use rand::Rng;
use serde::Serialize;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

/// Main generator for creating synthetic X12 835 claims
pub struct Generator {
    config: Config,
    pop_generator: PopulationGenerator,
    claim_generator: ClaimGenerator,
    anomaly_injector: AnomalyInjector,
}

impl Generator {
    /// Create a new Generator with the given configuration
    pub fn new(config: Config) -> Self {
        let seed = config.seed;
        
        let pop_generator = PopulationGenerator::new(seed);
        let claim_generator = ClaimGenerator::new(seed);
        
        // Configure anomaly injection
        let anomaly_config = AnomalyConfig {
            base_rate: config.anomaly_rate,
            ..Default::default()
        };
        
        let anomaly_injector = AnomalyInjector::new(anomaly_config, seed);
        
        Self {
            config,
            pop_generator,
            claim_generator,
            anomaly_injector,
        }
    }
    
    /// Generate claims and write them to the configured output
    pub fn generate(&mut self) -> io::Result<()> {
        // Create a local reference to config to avoid borrowing issues
        let output_path = self.config.output_path.clone();
        match output_path {
            Some(path) => self.generate_to_file(path),
            None => self.generate_to_stdout(),
        }
    }
    
    /// Generate claims and write them to a file
    pub fn generate_to_file<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        let file = File::create(path)?;
        self.generate_and_serialize::<File>(Box::new(file))
    }
    
    /// Generate claims and write them to stdout
    pub fn generate_to_stdout(&mut self) -> io::Result<()> {
        let stdout = io::stdout();
        let handle = stdout.lock();
        self.generate_and_serialize::<io::StdoutLock<'_>>(Box::new(handle))
    }
    
    /// Generate claims and serialize them to the given writer
    fn generate_and_serialize<W: Write + 'static>(
        &mut self,
        writer: Box<dyn Write>,
    ) -> io::Result<()> {
        match self.config.output_format {
            OutputFormat::X12 => self.generate_x12::<W>(writer),
            OutputFormat::Json => self.generate_json::<W>(writer, false),
            OutputFormat::JsonPretty => self.generate_json::<W>(writer, true),
        }
    }
    
    /// Generate claims in X12 EDI format (835 transaction set)
    fn generate_x12<W: Write + 'static>(&mut self, mut writer: Box<dyn Write>) -> io::Result<()> {
        // Create a new X12 interchange
        let control_number = Self::generate_control_number();
        let mut interchange = X12Interchange::new(
            "SENDER001",
            "RECEIVER01",
            &control_number,
        );

        // Create a functional group
        let mut group = FunctionalGroup::new(
            "SENDER001",
            "RECEIVER01",
            &control_number,
            &Utc::now().format("%Y%m%d").to_string(),
            &Utc::now().format("%H%M").to_string(),
        );

        // Create a transaction set for each claim
        for _ in 0..self.config.claim_count {
            let claim_result = self.generate_single_claim();
            let transaction = self.create_835_transaction(&claim_result.claim);
            group.add_transaction_set(transaction);
        }

        // Add the group to the interchange
        interchange.add_functional_group(group);

        // Write the X12 data to the writer
        write!(&mut *writer, "{}", interchange)?;
        Ok(())
    }
    
    /// Create an X12 835 transaction from a claim
    fn create_835_transaction(&self, claim: &Claim) -> TransactionSet {
        let control_number = Self::generate_control_number();
        let mut transaction = TransactionSet::new(&control_number);
        
        // Add BPR (Beginning of Payment Order/Remittance Advice)
        let bpr = BprSegment {
            bpr02_payment_amount: (claim.total_payment as f64) / 100.0,
            bpr03_credit_debit: 'C',
            bpr04_payment_method: "ACH".to_string(),
            bpr16_payment_date: Utc::now().format("%Y%m%d").to_string(),
        };
        transaction.add_segment(bpr);
        
        // Add TRN (Trace Number)
        let trn = TrnSegment {
            trn02_reference_id: claim.claim_id.clone(),
            trn03_orig_company_id: "PAYER123".to_string(),
        };
        transaction.add_segment(trn);
        
        // Add DTM (Date/Time Reference) - Production Date
        let dtm = DtmSegment {
            dtm01_qualifier: "405".to_string(),
            dtm02_date: Utc::now().format("%Y%m%d").to_string(),
        };
        transaction.add_segment(dtm);
        
        // Add N1 (Name) - Payer
        let n1_payer = N1Segment {
            n101_entity_id: "PR".to_string(),
            n102_name: "PAYER NAME".to_string(),
            n103_id_qual: "PI".to_string(),
            n104_id: "1234567890".to_string(),
        };
        transaction.add_segment(n1_payer);
        
        // Add N1 (Name) - Payee
        let n1_payee = N1Segment {
            n101_entity_id: "PE".to_string(),
            n102_name: claim.billing_provider.name.clone(),
            n103_id_qual: "XX".to_string(),
            n104_id: claim.billing_provider.npi.clone(),
        };
        transaction.add_segment(n1_payee);
        
        // Add CLP (Claim Payment Information)
        let clp = ClpSegment {
            clp01_claim_id: claim.claim_id.clone(),
            clp02_claim_status: "1".to_string(), // Paid
            clp03_charge_amount: (claim.total_charge as f64) / 100.0,
            clp04_paid_amount: (claim.total_payment as f64) / 100.0,
            clp05_patient_responsibility: (claim.patient_responsibility as f64) / 100.0,
            clp06_claim_type: "11".to_string(), // Non-FFS
            clp07_payer_claim_number: format!("CLM{}", claim.claim_id),
        };
        transaction.add_segment(clp);
        
        // Add service lines
        for (i, service_line) in claim.service_lines.iter().enumerate() {
            let svc = SvcSegment {
                svc01_procedure_code: service_line.procedure_code.clone(),
                svc02_charge_amount: (service_line.charge_amount as f64) / 100.0,
                svc03_paid_amount: (service_line.paid_amount as f64) / 100.0,
                svc04_revenue_code: service_line.revenue_code.clone(),
                svc05_units: service_line.units as f64,
            };
            transaction.add_segment(svc);
            
            // TODO: Add any service line adjustments here
        }
        
        transaction
    }
    
    /// Generate a unique control number
    fn generate_control_number() -> String {
        let start = SystemTime::now();
        let since_epoch = start
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let random: u32 = rand::thread_rng().gen();
        format!("{:08}{:08}", since_epoch % 100000000, random % 10000)
    }
    
    /// Generate claims in JSON format
    fn generate_json<W: Write + 'static>(
        &mut self,
        mut writer: Box<dyn Write>,
        pretty: bool,
    ) -> io::Result<()> {
        let mut claims = Vec::with_capacity(self.config.claim_count);
        
        for _ in 0..self.config.claim_count {
            let result = self.generate_single_claim();
            claims.push(JsonOutput {
                claim: result.claim,
                anomalies: result.anomalies,
            });
        }
        
        let writer_ref = &mut *writer;
        if pretty {
            serde_json::to_writer_pretty(writer_ref, &claims)?;
        } else {
            serde_json::to_writer(writer_ref, &claims)?;
        }
        
        Ok(())
    }
    
    /// Generate a single claim with anomalies
    fn generate_single_claim(&mut self) -> AnomalyInjectionResult {
        // Generate synthetic data
        let patient = self.pop_generator.generate_person();
        let billing_provider = self.pop_generator.generate_provider();
        let rendering_provider = if rand::random() {
            Some(self.pop_generator.generate_provider())
        } else {
            None
        };
        
        // Generate claim
        let claim = self.claim_generator.generate_claim(
            patient,
            billing_provider,
            rendering_provider,
        );
        
        // Inject anomalies
        self.anomaly_injector.inject_anomalies(claim)
    }
}

/// Structure for JSON output
#[derive(Debug, Serialize)]
struct JsonOutput {
    claim: Claim,
    anomalies: Vec<Anomaly>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_generator_creation() {
        let config = Config::default();
        let generator = Generator::new(config);
        
        assert!(generator.config.claim_count > 0);
        assert!(generator.config.anomaly_rate >= 0.0);
    }
    
    #[test]
    fn test_generate_to_file() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_owned();
        
        let config = Config {
            claim_count: 5,
            output_path: Some(path.clone()),
            output_format: crate::config::OutputFormat::Json,
            ..Default::default()
        };
        
        let mut generator = Generator::new(config);
        generator.generate()?;
        
        // Verify the file was created and has content
        let metadata = std::fs::metadata(path)?;
        assert!(metadata.len() > 0);
        
        Ok(())
    }
    
    #[test]
    fn test_generate_to_stdout() -> io::Result<()> {
        use tempfile::NamedTempFile;
        
        let config = Config {
            claim_count: 2,
            output_path: None, // This should make it use stdout
            output_format: crate::config::OutputFormat::Json,
            ..Default::default()
        };
        
        let mut generator = Generator::new(config);
        
        // Create a temporary file for testing
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_owned();
        
        // Generate to file
        generator.generate_to_file(&path)?;
        
        // Read the file back
        let content = std::fs::read_to_string(path)?;
        assert!(!content.is_empty());
        
        // The output should be valid JSON
        let json: serde_json::Value = serde_json::from_str(&content)
            .expect("Output should be valid JSON");
            
        // Should be an array with 2 items (claim_count = 2)
        if let serde_json::Value::Array(items) = json {
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected JSON array");
        }
        
        Ok(())
    }
}
