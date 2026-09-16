use crate::application::errors::LedgerError;
use crate::application::journal_validator::JournalValidator;
use crate::domain::Journal;

pub struct PostingEngine;

impl PostingEngine {
    // [SecOps Guard] Checked Signer & Authority Validation
    pub fn post(journal: &Journal) -> Result<(), LedgerError> {
        JournalValidator::validate(journal).map_err(LedgerError::RepositoryError)?;

        // Próximas etapas:
        // 1. Validar Chart of Accounts
        // 2. Validar moedas
        // 3. Abrir transação
        // 4. Persistir Journal
        // 5. Persistir Ledger Entries
        // 6. Commit/Rollback

        Ok(())
    }
}
