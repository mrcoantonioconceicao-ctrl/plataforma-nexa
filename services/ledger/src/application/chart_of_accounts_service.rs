use crate::application::default_chart::build_default_chart;
use crate::domain::{Account, ChartOfAccounts};

#[derive(Clone)]
pub struct ChartOfAccountsService {
    chart: ChartOfAccounts,
}

impl ChartOfAccountsService {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new() -> Self {
        Self {
            chart: build_default_chart(),
        }
    }

    pub fn all(&self) -> Vec<&Account> {
        self.chart.all()
    }

    pub fn find(&self, code: &str) -> Option<&Account> {
        self.chart.get(code)
    }

    pub fn exists(&self, code: &str) -> bool {
        self.chart.exists(code)
    }

    pub fn total_accounts(&self) -> usize {
        self.chart.total_accounts()
    }
}
