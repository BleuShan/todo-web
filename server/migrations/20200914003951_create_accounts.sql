CREATE TABLE IF NOT EXISTS accounts (
    id UUID DEFAULT uuid_generate_v4(),
    handle TEXT UNIQUE NOT NULL,
    first_name TEXT,
    last_name TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS accounts_emails (
    id UUID DEFAULT uuid_generate_v4(),
    account_id UUID,
    email TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    PRIMARY KEY (id),
    CONSTRAINT fk_accounts_emails_account_id FOREIGN KEY (account_id) REFERENCES accounts ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS account_passwords (
    id UUID DEFAULT uuid_generate_v4(),
    account_id UUID,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    PRIMARY KEY (id),
    CONSTRAINT fk_account_passwords_account_id FOREIGN KEY (account_id) REFERENCES Accounts ON DELETE CASCADE
);