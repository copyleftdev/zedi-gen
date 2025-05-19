//! Configuration management for zedi-gen

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Default directory for CSV data files
const DEFAULT_DATA_DIR: &str = "data";

use crate::errors::Error;

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct Config {
    /// Random seed for reproducibility
    pub seed: Option<u64>,

    /// Number of claims to generate
    pub claim_count: usize,

    /// Anomaly injection rate (0.0 to 1.0)
    pub anomaly_rate: f64,

    /// Output file path (None for stdout)
    pub output_path: Option<PathBuf>,

    /// Output format
    pub output_format: OutputFormat,
    /// Data directory path for CSV files
    pub data_dir: PathBuf,
}

/// Supported output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    /// X12 EDI format
    X12,

    /// JSON format
    Json,

    /// Pretty-printed JSON (for debugging)
    JsonPretty,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            seed: None,
            claim_count: 1000,
            anomaly_rate: 0.01, // 1% anomalies by default
            output_path: None,
            output_format: OutputFormat::X12,
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
        }
    }
}

impl Config {
    /// Create a new configuration from generate subcommand arguments
    pub fn from_cli(args: &crate::cli::GenerateArgs) -> Self {
        let output_format = match args.format {
            crate::cli::OutputFormat::X12 => OutputFormat::X12,
            crate::cli::OutputFormat::Json => OutputFormat::Json,
            crate::cli::OutputFormat::JsonPretty => OutputFormat::JsonPretty,
        };

        Self {
            seed: args.seed,
            claim_count: args.count,
            anomaly_rate: args.anomaly_rate,
            output_path: args.output.clone(),
            output_format,
            data_dir: args.data_dir.clone(),
        }
    }

    /// Load configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let content = fs::read_to_string(path).map_err(Error::Io)?;
        toml::from_str(&content).map_err(Error::TomlDe)
    }

    /// Save configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let content = toml::to_string_pretty(self).map_err(Error::Toml)?;
        fs::write(path, content).map_err(Error::Io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_serialization() {
        let config = Config {
            seed: Some(42),
            claim_count: 500,
            anomaly_rate: 0.05,
            output_path: Some("output.json".into()),
            output_format: OutputFormat::Json,
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
        };

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // Test serialization
        config.save_to_file(path).unwrap();

        // Test deserialization
        let loaded = Config::from_file(path).unwrap();

        assert_eq!(config.seed, loaded.seed);
        assert_eq!(config.claim_count, loaded.claim_count);
        assert_eq!(config.anomaly_rate, loaded.anomaly_rate);
        assert_eq!(
            config.output_path.unwrap().to_str(),
            loaded.output_path.unwrap().to_str()
        );
        assert_eq!(config.output_format, loaded.output_format);
        assert_eq!(config.data_dir, loaded.data_dir);
    }
}
