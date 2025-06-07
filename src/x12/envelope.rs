

use super::segments::*;
use std::fmt;


pub struct X12Interchange {
    pub isa: IsaSegment,
    pub functional_groups: Vec<FunctionalGroup>,
    pub iea: IeaSegment,
}

impl X12Interchange {
    
    pub fn new(sender_id: &str, receiver_id: &str, control_number: &str) -> Self {
        let isa = IsaSegment {
            isa06_sender_id: sender_id.to_string(),
            isa08_receiver_id: receiver_id.to_string(),
            isa12_control_number: control_number.to_string(),
            isa13_usage_indicator: 'P', 
        };

        let iea = IeaSegment {
            iea01_number_of_included_functional_groups: 1, 
            iea02_interchange_control_number: control_number.to_string(),
        };

        Self {
            isa,
            functional_groups: Vec::new(),
            iea,
        }
    }

    
    pub fn add_functional_group(&mut self, group: FunctionalGroup) {
        self.functional_groups.push(group);
        self.iea.iea01_number_of_included_functional_groups = self.functional_groups.len() as u32;
    }
}

impl fmt::Display for X12Interchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        writeln!(f, "{}", self.isa)?;

        
        for group in &self.functional_groups {
            writeln!(f, "{}", group)?;
        }

        
        write!(f, "{}", self.iea)
    }
}


pub struct FunctionalGroup {
    pub gs: GsSegment,
    pub transaction_sets: Vec<TransactionSet>,
    pub ge: GeSegment,
}

impl FunctionalGroup {
    
    pub fn new(
        sender_id: &str,
        receiver_id: &str,
        control_number: &str,
        _date: &str,
        _time: &str,
    ) -> Self {
        let gs = GsSegment {
            gs02_sender_id: sender_id.to_string(),
            gs03_receiver_id: receiver_id.to_string(),
            gs06_group_control_number: control_number.to_string(),
        };

        let ge = GeSegment {
            ge01_number_of_transaction_sets: 0, 
            ge02_group_control_number: control_number.to_string(),
        };

        Self {
            gs,
            transaction_sets: Vec::new(),
            ge,
        }
    }

    
    pub fn add_transaction_set(&mut self, transaction: TransactionSet) {
        self.transaction_sets.push(transaction);
        self.ge.ge01_number_of_transaction_sets = self.transaction_sets.len() as u32;
    }
}

impl fmt::Display for FunctionalGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        writeln!(f, "{}", self.gs)?;

        
        for transaction in &self.transaction_sets {
            writeln!(f, "{}", transaction)?;
        }

        
        write!(f, "{}", self.ge)
    }
}


pub struct TransactionSet {
    pub st: StSegment,
    pub segments: Vec<Box<dyn X12Segment>>,
    pub se: SeSegment,
}

impl TransactionSet {
    
    pub fn new(control_number: &str) -> Self {
        let st = StSegment {
            st02_control_number: control_number.to_string(),
        };

        let se = SeSegment {
            se01_segment_count: 0, 
            se02_transaction_control_number: control_number.to_string(),
        };

        Self {
            st,
            segments: Vec::new(),
            se,
        }
    }

    
    pub fn add_segment<S: X12Segment + 'static>(&mut self, segment: S) {
        self.segments.push(Box::new(segment));
        
        self.se.se01_segment_count = (self.segments.len() + 2) as u32;
    }
}

impl fmt::Display for TransactionSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        writeln!(f, "{}", self.st)?;

        
        for segment in &self.segments {
            writeln!(f, "{}", segment)?;
        }

        
        write!(f, "{}", self.se)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::x12::segments::*;

    #[test]
    fn test_x12_interchange() {
        let mut interchange = X12Interchange::new("SENDER123", "RECEIVER456", "000000001");

        let mut group =
            FunctionalGroup::new("SENDER123", "RECEIVER456", "000000001", "20230518", "1200");

        let mut transaction = TransactionSet::new("0001");

        
        let bpr = BprSegment {
            bpr02_payment_amount: 1000.50,
            bpr03_credit_debit: 'C',
            bpr04_payment_method: "ACH".to_string(),
            bpr16_payment_date: "20230518".to_string(),
        };
        transaction.add_segment(bpr);

        
        let trn = TrnSegment {
            trn02_reference_id: "1234567890".to_string(),
            trn03_orig_company_id: "COMPANY123".to_string(),
        };
        transaction.add_segment(trn);

        group.add_transaction_set(transaction);
        interchange.add_functional_group(group);

        let x12_output = format!("{}", interchange);
        assert!(x12_output.contains("ISA*00*"));
        assert!(x12_output.contains("GS*HP*"));
        assert!(x12_output.contains("ST*835*"));
        assert!(x12_output.contains("BPR*C*1000.50"));
        assert!(x12_output.contains("TRN*1*1234567890*COMPANY123"));
        assert!(x12_output.contains("SE*"));
        assert!(x12_output.contains("GE*"));
        assert!(x12_output.contains("IEA*"));
    }
}
