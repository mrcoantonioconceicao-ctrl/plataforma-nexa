use crate::domain::journal::Journal;
use crate::domain::ledger_entry::LedgerEntry;
use crate::infrastructure::ledger_repository::LedgerRepository;

use rust_decimal::Decimal;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone)]
pub struct LedgerService<R>
where
    R: LedgerRepository + Clone,
{
    repository: R,
}

impl<R> LedgerService<R>
where
    R: LedgerRepository + Clone,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    /// Publica um Journal no Livro Razão.
    pub fn post_journal(&self, journal: &Journal) -> Result<(), String> {
        journal.validate()?;

        self.validate_accounts(journal)?;

        for entry in &journal.entries {
            let ledger_entry = LedgerEntry::new(
                journal.id,
                entry.account_id,
                entry.currency.clone(),
                entry.debit,
                entry.credit,
            );

            self.repository.save(ledger_entry);
        }

        Ok(())
    }

    /// Retorna todos os lançamentos.
    pub fn list_all(&self) -> Vec<LedgerEntry> {
        self.repository.find_all()
    }

    /// Busca um lançamento pelo ID.
    pub fn find_by_id(&self, id: Uuid) -> Option<LedgerEntry> {
        self.repository.find_by_id(id)
    }

    /// Lista lançamentos de uma conta.
    pub fn find_by_account(&self, account: &str) -> Vec<LedgerEntry> {
        self.repository.find_by_account(account)
    }

    /// Soma dos débitos da conta.
    pub fn total_debit(&self, account: &str) -> Decimal {
        self.find_by_account(account)
            .iter()
            .fold(Decimal::ZERO, |sum, entry| sum + entry.debit)
    }

    /// Soma dos créditos da conta.
    pub fn total_credit(&self, account: &str) -> Decimal {
        self.find_by_account(account)
            .iter()
            .fold(Decimal::ZERO, |sum, entry| sum + entry.credit)
    }

    /// Saldo líquido da conta.
    pub fn balance(&self, account: &str) -> Decimal {
        self.total_debit(account) - self.total_credit(account)
    }

    fn validate_accounts(&self, journal: &Journal) -> Result<(), String> {
        let mut accounts = HashSet::<Uuid>::new();

        for entry in &journal.entries {
            if !accounts.insert(entry.account_id) {
                return Err(format!(
                    "Conta duplicada encontrada no Journal: {}",
                    entry.account_id
                ));
            }
        }

        Ok(())
    }
}
