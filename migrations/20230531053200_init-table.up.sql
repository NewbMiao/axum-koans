-- Add up migration script here
CREATE TABLE IF NOT EXISTS profiles (
    id SERIAL PRIMARY KEY,
    account_id VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    picture VARCHAR(255),
    refresh_token VARCHAR(255) NOT NULL,
    created_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_date TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT idx_profiles_account UNIQUE (account_id)
);
