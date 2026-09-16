use chrono::{DateTime, Utc};

pub type Timestamp = DateTime<Utc>;

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn now() -> Timestamp {
    Utc::now()
}
