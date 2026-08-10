CREATE TABLE IF NOT EXISTS ledger_entries (
    id UUID PRIMARY KEY,

    journal_id UUID NOT NULL,

    account_id UUID NOT NULL,

    currency VARCHAR(16) NOT NULL,

    debit NUMERIC(20,8) NOT NULL DEFAULT 0,

    credit NUMERIC(20,8) NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL,

    CONSTRAINT chk_positive_values
        CHECK (debit >= 0 AND credit >= 0),

    CONSTRAINT chk_only_one_side
        CHECK (
            (debit > 0 AND credit = 0)
            OR
            (credit > 0 AND debit = 0)
        )
);

CREATE INDEX idx_ledger_journal
ON ledger_entries(journal_id);

CREATE INDEX idx_ledger_account
ON ledger_entries(account_id);

CREATE INDEX idx_ledger_created_at
ON ledger_entries(created_at);
