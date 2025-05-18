//! Synthetic population generation for zedi-gen

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a synthetic person
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Unique identifier
    pub id: String,
    
    /// First name
    pub first_name: String,
    
    /// Last name
    pub last_name: String,
    
    /// Date of birth (YYYY-MM-DD)
    pub date_of_birth: String,
    
    /// Gender (M/F/Other)
    pub gender: String,
    
    /// Address information
    pub address: Address,
}

/// Represents an address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    /// Street address line 1
    pub line1: String,
    
    /// Street address line 2 (optional)
    pub line2: Option<String>,
    
    /// City
    pub city: String,
    
    /// State (2-letter code)
    pub state: String,
    
    /// ZIP code
    pub zip_code: String,
}

/// Represents a healthcare provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    /// National Provider Identifier (NPI)
    pub npi: String,
    
    /// Provider type (taxonomy code)
    pub provider_type: String,
    
    /// Provider name (organization or individual)
    pub name: String,
    
    /// Address information
    pub address: Address,
    
    /// Taxonomy codes
    pub taxonomy_codes: Vec<String>,
}

/// Generates synthetic population data
pub struct PopulationGenerator {
    rng: ChaCha8Rng,
    first_names: HashMap<String, Vec<String>>,
    last_names: Vec<String>,
    cities: Vec<(String, String, String)>, // (city, state, zip)
    provider_types: Vec<String>,
    taxonomy_codes: Vec<String>,
}

impl PopulationGenerator {
    /// Create a new population generator with an optional seed
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };
        
        // TODO: Load real data from files
        let first_names = HashMap::new();
        let last_names = Vec::new();
        let cities = Vec::new();
        let provider_types = Vec::new();
        let taxonomy_codes = Vec::new();
        
        Self {
            rng,
            first_names,
            last_names,
            cities,
            provider_types,
            taxonomy_codes,
        }
    }
    
    /// Generate a synthetic person
    pub fn generate_person(&mut self) -> Person {
        // TODO: Implement realistic person generation
        Person {
            id: uuid::Uuid::new_v4().to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            date_of_birth: "1970-01-01".to_string(),
            gender: "M".to_string(),
            address: Address {
                line1: "123 Main St".to_string(),
                line2: None,
                city: "Anytown".to_string(),
                state: "CA".to_string(),
                zip_code: "12345".to_string(),
            },
        }
    }
    
    /// Generate a synthetic provider
    pub fn generate_provider(&mut self) -> Provider {
        // TODO: Implement realistic provider generation
        Provider {
            npi: format!("{:010}", self.rng.gen_range(1000000000u64..=9999999999)),
            provider_type: "General Practice".to_string(),
            name: "Acme Medical Group".to_string(),
            address: Address {
                line1: "456 Healthcare Dr".to_string(),
                line2: Some("Suite 100".to_string()),
                city: "Meditown".to_string(),
                state: "CA".to_string(),
                zip_code: "12345".to_string(),
            },
            taxonomy_codes: vec!["207Q00000X".to_string()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_person_generation() {
        let mut generator = PopulationGenerator::new(Some(42));
        let person = generator.generate_person();
        
        assert!(!person.id.is_empty());
        assert!(!person.first_name.is_empty());
        assert!(!person.last_name.is_empty());
        assert!(!person.date_of_birth.is_empty());
        assert!(!person.gender.is_empty());
        assert!(!person.address.line1.is_empty());
        assert!(!person.address.city.is_empty());
        assert!(!person.address.state.is_empty());
        assert!(!person.address.zip_code.is_empty());
    }
    
    #[test]
    fn test_provider_generation() {
        let mut generator = PopulationGenerator::new(Some(42));
        let provider = generator.generate_provider();
        
        assert_eq!(provider.npi.len(), 10);
        assert!(!provider.provider_type.is_empty());
        assert!(!provider.name.is_empty());
        assert!(!provider.address.line1.is_empty());
        assert!(!provider.address.city.is_empty());
        assert!(!provider.address.state.is_empty());
        assert!(!provider.address.zip_code.is_empty());
        assert!(!provider.taxonomy_codes.is_empty());
    }
}
