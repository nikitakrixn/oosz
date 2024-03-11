-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "accounts" (
                            account_id uuid NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
                            account_name VARCHAR(100) NOT NULL UNIQUE,
                            account_password VARCHAR(100) NOT NULL,
                            account_photo VARCHAR NOT NULL DEFAULT 'default.png',
                            account_enabled BOOLEAN NOT NULL DEFAULT TRUE,
                            account_created_at TIMESTAMP
                                WITH TIME ZONE DEFAULT NOW(),
                            account_updated_at TIMESTAMP
                                WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX accounts_name_idx ON accounts (account_name);