-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS "users" (
  id UUID DEFAULT uuid_generate_v4() NOT NULL,
  name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  is_verified BOOLEAN NOT NULL DEFAULT FALSE,
  verification_token TEXT,
  reset_token TEXT,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);

CREATE INDEX IF NOT EXISTS "user_email_idx" ON "users"(email);

CREATE TABLE IF NOT EXISTS session (
  token UUID DEFAULT uuid_generate_v4() NOT NULL,
  user_id UUID NOT NULL,
  expiration TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (token),
  FOREIGN KEY (user_id) REFERENCES "users"(id)
);

CREATE OR REPLACE FUNCTION update_at() RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_at BEFORE INSERT OR UPDATE ON "users" FOR EACH ROW EXECUTE PROCEDURE update_at();
