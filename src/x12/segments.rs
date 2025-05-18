//! X12 835 segment definitions

use std::fmt;

/// X12 Segment trait that defines how to format a segment
pub trait X12Segment: fmt::Display {}

/// ISA - Interchange Control Header
#[derive(Debug, Clone)]
pub struct IsaSegment {
    pub isa06_sender_id: String,      // Sender ID
    pub isa08_receiver_id: String,    // Receiver ID
    pub isa12_control_number: String, // Interchange Control Number
    pub isa13_usage_indicator: char,  // I=Information, P=Production, T=Test
}

impl fmt::Display for IsaSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ISA*00*          *00*          *ZZ*{:15}*ZZ*{:15}*{}*{}*^*00501*{}*0*{}*~",
            self.isa06_sender_id,
            self.isa08_receiver_id,
            chrono::Local::now().format("%y%m%d"),
            chrono::Local::now().format("%H%M"),
            self.isa12_control_number,
            self.isa13_usage_indicator
        )
    }
}

impl X12Segment for IsaSegment {}

/// GS - Functional Group Header
#[derive(Debug, Clone)]
pub struct GsSegment {
    pub gs02_sender_id: String,       // Application Sender's Code
    pub gs03_receiver_id: String,     // Application Receiver's Code
    pub gs06_group_control_number: String, // Group Control Number
}

impl fmt::Display for GsSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GS*HP*{}*{}*{}*{}*{}*X*005010X221A1~",
            self.gs02_sender_id,
            self.gs03_receiver_id,
            chrono::Local::now().format("%Y%m%d"),
            chrono::Local::now().format("%H%M"),
            self.gs06_group_control_number
        )
    }
}

impl X12Segment for GsSegment {}

/// ST - Transaction Set Header
#[derive(Debug, Clone)]
pub struct StSegment {
    pub st02_control_number: String, // Transaction Set Control Number
}

impl fmt::Display for StSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ST*835*0001*005010X221A1~")
    }
}

impl X12Segment for StSegment {}

/// BPR - Beginning of Payment Order/Remittance Advice
#[derive(Debug, Clone)]
pub struct BprSegment {
    pub bpr02_payment_amount: f64,
    pub bpr03_credit_debit: char, // C=Credit, D=Debit
    pub bpr04_payment_method: String, // ACH, CHK, FWT, etc.
    pub bpr16_payment_date: String, // YYYYMMDD format
}

impl fmt::Display for BprSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BPR*C*{:.2}*C*ACH*CC*01*999999999*DA*999999999*{}*{}*01*999999999*DA*999999999*{}*{}~",
            self.bpr02_payment_amount,
            self.bpr04_payment_method,
            "9999999999",
            self.bpr16_payment_date,
            chrono::Local::now().format("%Y%m%d")
        )
    }
}

impl X12Segment for BprSegment {}

/// TRN - Trace Number
#[derive(Debug, Clone)]
pub struct TrnSegment {
    pub trn02_reference_id: String,
    pub trn03_orig_company_id: String,
}

impl fmt::Display for TrnSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TRN*1*{}*{}~",
            self.trn02_reference_id, self.trn03_orig_company_id
        )
    }
}

impl X12Segment for TrnSegment {}

/// DTM - Date/Time Reference
#[derive(Debug, Clone)]
pub struct DtmSegment {
    pub dtm01_qualifier: String, // 405 = Production Date
    pub dtm02_date: String,     // YYYYMMDD format
}

impl fmt::Display for DtmSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DTM*{}*{}~", self.dtm01_qualifier, self.dtm02_date)
    }
}

impl X12Segment for DtmSegment {}

/// N1 - Name
#[derive(Debug, Clone)]
pub struct N1Segment {
    pub n101_entity_id: String,  // PR = Payer, PE = Payee
    pub n102_name: String,
    pub n103_id_qual: String,   // XX = EIN, FI = Federal Tax ID, etc.
    pub n104_id: String,
}

impl fmt::Display for N1Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "N1*{}*{}*{}*{}~",
            self.n101_entity_id, self.n102_name, self.n103_id_qual, self.n104_id
        )
    }
}

impl X12Segment for N1Segment {}

/// CLP - Claim Payment Information
#[derive(Debug, Clone)]
pub struct ClpSegment {
    pub clp01_claim_id: String,
    pub clp02_claim_status: String, // 1 = Processed as Primary, 2 = Processed as Secondary, etc.
    pub clp03_charge_amount: f64,
    pub clp04_paid_amount: f64,
    pub clp05_patient_responsibility: f64,
    pub clp06_claim_type: String,   // 11 = Non-FFS, 12 = PPV, etc.
    pub clp07_payer_claim_number: String,
}

impl fmt::Display for ClpSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CLP*{}*{}*{:.2}*{:.2}**{}*{}*11*1~",
            self.clp01_claim_id,
            self.clp02_claim_status,
            self.clp03_charge_amount,
            self.clp04_paid_amount,
            self.clp06_claim_type,
            self.clp07_payer_claim_number
        )
    }
}

impl X12Segment for ClpSegment {}

/// SVC - Service Payment Information
#[derive(Debug, Clone)]
pub struct SvcSegment {
    pub svc01_procedure_code: String,
    pub svc02_charge_amount: f64,
    pub svc03_paid_amount: f64,
    pub svc04_revenue_code: Option<String>,
    pub svc05_units: f64,
}

impl fmt::Display for SvcSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SVC*{}:{:.2}*{:.2}*{:.2}*{}*{}~",
            self.svc01_procedure_code,
            self.svc02_charge_amount,
            self.svc03_paid_amount,
            self.svc03_paid_amount, // Paid amount again for the second amount
            self.svc04_revenue_code.as_deref().unwrap_or(""),
            self.svc05_units
        )
    }
}

impl X12Segment for SvcSegment {}

/// SE - Transaction Set Trailer
#[derive(Debug, Clone)]
pub struct SeSegment {
    pub se01_segment_count: u32,
    pub se02_transaction_control_number: String,
}

impl fmt::Display for SeSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SE*{}*{}~",
            self.se01_segment_count, self.se02_transaction_control_number
        )
    }
}

impl X12Segment for SeSegment {}

/// GE - Functional Group Trailer
#[derive(Debug, Clone)]
pub struct GeSegment {
    pub ge01_number_of_transaction_sets: u32,
    pub ge02_group_control_number: String,
}

impl fmt::Display for GeSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GE*{}*{}~",
            self.ge01_number_of_transaction_sets, self.ge02_group_control_number
        )
    }
}

impl X12Segment for GeSegment {}

/// IEA - Interchange Control Trailer
#[derive(Debug, Clone)]
pub struct IeaSegment {
    pub iea01_number_of_included_functional_groups: u32,
    pub iea02_interchange_control_number: String,
}

impl fmt::Display for IeaSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IEA*{}*{}~",
            self.iea01_number_of_included_functional_groups, self.iea02_interchange_control_number
        )
    }
}

impl X12Segment for IeaSegment {}
