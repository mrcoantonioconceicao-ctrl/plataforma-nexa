CREATE TABLE IF NOT EXISTS ledger_entries (
    id UUID PRIMARY KEY,
    journal_id UUID NOT NULL,
    account_id UUID NOT NULL,
    currency VARCHAR(3) NOT NULL,
    debit DECIMAL(18, 4) NOT NULL DEFAULT 0.0000,
    credit DECIMAL(18, 4) NOT NULL DEFAULT 0.0000,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_ledger_entries_account_id ON ledger_entries(account_id);
CREATE INDEX IF NOT EXISTS idx_ledger_entries_journal_id ON ledger_entries(journal_id);
CREATE INDEX IF NOT EXISTS idx_ledger_entries_created_at ON ledger_entries(created_at);

