

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod anomalies;
pub mod claims;
pub mod cli;
pub mod config;
pub mod errors;
pub mod generator;
pub mod population;
pub mod x12;


pub use {
    anomalies::AnomalyInjector,
    claims::Claim,
    config::Config,
    errors::Error,
    generator::Generator,
    population::{Person, Provider},
    x12::{
        envelope::{FunctionalGroup, TransactionSet, X12Interchange},
        segments::{
            BprSegment, ClpSegment, DtmSegment, GeSegment, GsSegment, IeaSegment, IsaSegment,
            N1Segment, SeSegment, StSegment, SvcSegment, TrnSegment, X12Segment,
        },
    },
};


pub type Result<T> = std::result::Result<T, Error>;
