

use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::Path;

use csv;
use fake::faker::{
    address::en::{BuildingNumber, SecondaryAddress, StreetName},
    company::en::CompanyName,
    name::en::{FirstName, LastName},
};
use fake::Fake;

#[derive(Debug, Deserialize)]
struct FirstNameRecord {
    gender: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CityRecord {
    city: String,
    state: String,
    zip: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    
    pub id: String,

    
    pub first_name: String,

    
    pub last_name: String,

    
    pub date_of_birth: String,

    
    pub gender: String,

    
    pub address: Address,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    
    pub line1: String,

    
    pub line2: Option<String>,

    
    pub city: String,

    
    pub state: String,

    
    pub zip_code: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    
    pub npi: String,

    
    pub provider_type: String,

    
    pub name: String,

    
    pub address: Address,

    
    pub taxonomy_codes: Vec<String>,
}


pub struct PopulationGenerator {
    rng: ChaCha8Rng,
    first_names: HashMap<String, Vec<String>>,
    last_names: Vec<String>,
    cities: Vec<(String, String, String)>, 
    provider_types: Vec<String>,
    taxonomy_codes: Vec<String>,
}

impl PopulationGenerator {
    
    pub fn new(seed: Option<u64>) -> Self {
        let rng = match seed {
            Some(seed) => ChaCha8Rng::seed_from_u64(seed),
            None => ChaCha8Rng::from_entropy(),
        };

        
        let data_dir = env::var("ZEDI_GEN_DATA_DIR").unwrap_or_else(|_| "data".to_string());
        let data_path = Path::new(&data_dir);

        
        let mut first_names: HashMap<String, Vec<String>> = HashMap::new();
        if let Ok(mut rdr) = csv::Reader::from_path(data_path.join("first_names.csv")) {
            for result in rdr.deserialize() {
                if let Ok(rec) = result {
                    let rec: FirstNameRecord = rec;
                    first_names
                        .entry(rec.gender.clone())
                        .or_default()
                        .push(rec.name.clone());
                }
            }
        }
        if first_names.is_empty() {
            first_names.insert(
                "M".to_string(),
                vec!["John".to_string(), "Robert".to_string()],
            );
            first_names.insert(
                "F".to_string(),
                vec!["Jane".to_string(), "Mary".to_string()],
            );
        }

        
        let mut last_names = Vec::new();
        if let Ok(mut rdr) = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(data_path.join("last_names.csv"))
        {
            for result in rdr.records() {
                if let Ok(record) = result {
                    if let Some(name) = record.get(0) {
                        last_names.push(name.to_string());
                    }
                }
            }
        }
        if last_names.is_empty() {
            last_names = vec![
                "Doe".to_string(),
                "Smith".to_string(),
                "Johnson".to_string(),
            ];
        }

        
        let mut cities = Vec::new();
        if let Ok(mut rdr) = csv::Reader::from_path(data_path.join("cities.csv")) {
            for result in rdr.deserialize() {
                if let Ok(rec) = result {
                    let rec: CityRecord = rec;
                    cities.push((rec.city, rec.state, rec.zip));
                }
            }
        }
        if cities.is_empty() {
            cities.push(("Anytown".to_string(), "CA".to_string(), "12345".to_string()));
        }

        
        let mut provider_types = Vec::new();
        if let Ok(mut rdr) = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(data_path.join("provider_types.csv"))
        {
            for result in rdr.records() {
                if let Ok(record) = result {
                    if let Some(pt) = record.get(0) {
                        provider_types.push(pt.to_string());
                    }
                }
            }
        }
        if provider_types.is_empty() {
            provider_types = vec!["General Practice".to_string()];
        }

        
        let mut taxonomy_codes = Vec::new();
        if let Ok(mut rdr) = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(data_path.join("taxonomy_codes.csv"))
        {
            for result in rdr.records() {
                if let Ok(record) = result {
                    if let Some(code) = record.get(0) {
                        taxonomy_codes.push(code.to_string());
                    }
                }
            }
        }
        if taxonomy_codes.is_empty() {
            taxonomy_codes = vec!["207Q00000X".to_string()];
        }

        Self {
            rng,
            first_names,
            last_names,
            cities,
            provider_types,
            taxonomy_codes,
        }
    }

    
    pub fn generate_person(&mut self) -> Person {
        
        let gender = if self.rng.gen_bool(0.5) { "M" } else { "F" }.to_string();
        
        let first_name = self
            .first_names
            .get(&gender)
            .and_then(|names| names.choose(&mut self.rng).cloned())
            .unwrap_or_else(|| FirstName().fake_with_rng(&mut self.rng));
        let last_name = self
            .last_names
            .choose(&mut self.rng)
            .cloned()
            .unwrap_or_else(|| LastName().fake_with_rng(&mut self.rng));
        
        let today = chrono::Utc::now().date_naive();
        let min_age_days = 18 * 365;
        let max_age_days = 90 * 365;
        let age_days = self.rng.gen_range(min_age_days..=max_age_days) as i64;
        let dob = today - chrono::Duration::days(age_days);
        let date_of_birth = dob.format("%Y-%m-%d").to_string();
        
        let (city, state, zip_code) = self
            .cities
            .choose(&mut self.rng)
            .cloned()
            .unwrap_or_else(|| ("Anytown".to_string(), "CA".to_string(), "12345".to_string()));
        let building_number: String = BuildingNumber().fake_with_rng(&mut self.rng);
        let street_name: String = StreetName().fake_with_rng(&mut self.rng);
        let line1 = format!("{} {}", building_number, street_name);
        let line2 = if self.rng.gen_bool(0.2) {
            Some(SecondaryAddress().fake_with_rng(&mut self.rng))
        } else {
            None
        };
        Person {
            id: uuid::Uuid::new_v4().to_string(),
            first_name,
            last_name,
            date_of_birth,
            gender,
            address: Address {
                line1,
                line2,
                city,
                state,
                zip_code,
            },
        }
    }

    
    pub fn generate_provider(&mut self) -> Provider {
        
        let npi = format!(
            "{:010}",
            self.rng.gen_range(1_000_000_000u64..=9_999_999_999u64)
        );
        let provider_type = self
            .provider_types
            .choose(&mut self.rng)
            .cloned()
            .unwrap_or_else(|| "General Practice".to_string());
        let name: String = CompanyName().fake_with_rng(&mut self.rng);
        
        let (city, state, zip_code) = self
            .cities
            .choose(&mut self.rng)
            .cloned()
            .unwrap_or_else(|| ("Anytown".to_string(), "CA".to_string(), "12345".to_string()));
        let building_number: String = BuildingNumber().fake_with_rng(&mut self.rng);
        let street_name: String = StreetName().fake_with_rng(&mut self.rng);
        let line1 = format!("{} {}", building_number, street_name);
        let line2 = if self.rng.gen_bool(0.3) {
            Some(SecondaryAddress().fake_with_rng(&mut self.rng))
        } else {
            None
        };
        let taxonomy_codes = vec![self
            .taxonomy_codes
            .choose(&mut self.rng)
            .cloned()
            .unwrap_or_else(|| "207Q00000X".to_string())];
        Provider {
            npi,
            provider_type,
            name,
            address: Address {
                line1,
                line2,
                city,
                state,
                zip_code,
            },
            taxonomy_codes,
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
