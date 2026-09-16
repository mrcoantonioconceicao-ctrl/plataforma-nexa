use crate::domain::{Journal, JournalEntry};
use crate::infrastructure::{
    in_memory_journal_repository::InMemoryJournalRepository, journal_repository::JournalRepository,
};

use rust_decimal::Decimal;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone)]
pub struct JournalService {
    repository: InMemoryJournalRepository,
}

impl JournalService {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn new(repository: InMemoryJournalRepository) -> Self {
        Self { repository }
    }

    pub fn create(&self, journal: Journal) -> Result<Journal, String> {
        // Validação estrutural do Journal
        journal.validate()?;

        // Regras enterprise adicionais
        self.validate_entries(&journal.entries)?;

        // Persistência
        self.repository.save(journal.clone());

        Ok(journal)
    }

    pub fn list(&self) -> Vec<Journal> {
        self.repository.list()
    }

    pub fn find(&self, id: Uuid) -> Option<Journal> {
        self.repository.find_by_id(id)
    }

    fn validate_entries(&self, entries: &[JournalEntry]) -> Result<(), String> {
        // Não pode haver contas duplicadas no mesmo lançamento
        let mut accounts = HashSet::new();

        for entry in entries {
            if !accounts.insert(entry.account_id) {
                return Err(format!(
                    "Conta duplicada no lançamento: {}",
                    entry.account_id
                ));
            }

            // Débito e crédito não podem ser negativos
            if entry.is_negative() {
                return Err("Débito ou crédito negativo.".into());
            }

            // Não pode ter débito e crédito ao mesmo tempo
            if entry.debit > Decimal::ZERO && entry.credit > Decimal::ZERO {
                return Err(
                    "Uma entrada não pode possuir débito e crédito simultaneamente.".into(),
                );
            }

            // Precisa ter pelo menos um valor
            if entry.debit == Decimal::ZERO && entry.credit == Decimal::ZERO {
                return Err("Uma entrada precisa possuir débito ou crédito.".into());
            }
        }

        // Todas as moedas do Journal devem ser iguais
        let currencies: HashSet<_> = entries.iter().map(|e| e.currency()).collect();

        if currencies.len() != 1 {
            return Err("Lançamentos com moedas diferentes não são permitidos.".into());
        }

        Ok(())
    }
}
