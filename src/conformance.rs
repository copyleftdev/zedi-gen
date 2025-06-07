

use crate::errors::Result;
use std::collections::HashSet;
use std::fs;
use std::path::Path;


pub struct ConformanceResult {
    
    pub present: HashSet<String>,
    
    pub found: usize,
    
    pub total: usize,
}


const REQUIRED_SEGMENTS: &[&str] = &[
    "ISA", "GS", "ST", "BPR", "TRN", "DTM", "N1", "CLP", "SVC", "SE", "GE", "IEA",
];


pub fn compute_conformance(content: &str) -> ConformanceResult {
    
    let mut present = HashSet::new();
    for seg in content.split('~') {
        let seg = seg.trim();
        if seg.is_empty() {
            continue;
        }
        if let Some(id) = seg.split('*').next() {
            present.insert(id.to_string());
        }
    }
    let total = REQUIRED_SEGMENTS.len();
    let found = REQUIRED_SEGMENTS
        .iter()
        .filter(|id| present.contains(&id.to_string()))
        .count();
    ConformanceResult {
        present,
        found,
        total,
    }
}


pub fn run(input_path: &Path) -> Result<()> {
    let content = fs::read_to_string(input_path)?;
    let result = compute_conformance(&content);
    
    println!("Segment presence:");
    for id in REQUIRED_SEGMENTS {
        let status = if result.present.contains(&id.to_string()) {
            "OK"
        } else {
            "MISSING"
        };
        println!("  {:<3} - {}", id, status);
    }
    println!();
    let pct = (result.found as f64) / (result.total as f64) * 100.0;
    println!(
        "Score: {}/{} segments present ({:.1}%)",
        result.found, result.total, pct
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "ISA*00*          *00*          *ZZ*AAA          *ZZ*BBB          *200101*1253*^*00501*000000905*0*P*~\
GS*HP*AAA*BBB*200101*1253*1*X*005010X221A1~\
ST*835*0001~BPR*C*100.00*C*ACH***CTX*01*999999999*DA*999999999*200101~TRN*1*12345*98765~DTM*405*200101~\
N1*PR*PAYER~N1*PE*PAYEE~CLP*1234*1*200*100**11*PAYER123~SVC*HC:200*150*100*0450*1~SE*12*0001~GE*1*1~IEA*1*000000905~";

    #[test]
    fn test_compute_conformance_full() {
        let res = compute_conformance(SAMPLE);
        assert_eq!(res.total, 12);
        assert_eq!(res.found, 12);
    }

    #[test]
    fn test_compute_conformance_partial() {
        
        let sample = SAMPLE.replace("GE*1*1~IEA*1*000000905~", "");
        let res = compute_conformance(&sample);
        assert_eq!(res.total, 12);
        assert_eq!(res.found, 10);
    }
}
