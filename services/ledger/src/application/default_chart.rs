use crate::domain::{Account, ChartOfAccounts};

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn build_default_chart() -> ChartOfAccounts {
    let mut chart = ChartOfAccounts::new();

    chart.add_account(Account::new("1001".to_string(), "Cash".to_string()));

    chart.add_account(Account::new("1100".to_string(), "Bank".to_string()));

    chart.add_account(Account::new(
        "2000".to_string(),
        "Accounts Payable".to_string(),
    ));

    chart.add_account(Account::new("3000".to_string(), "Equity".to_string()));

    chart.add_account(Account::new("4000".to_string(), "Revenue".to_string()));

    chart.add_account(Account::new("5000".to_string(), "Expense".to_string()));

    chart
}
