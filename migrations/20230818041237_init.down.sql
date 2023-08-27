-- Add down migration script here
DROP TRIGGER IF EXISTS update_at ON "user";
DROP FUNCTION IF EXISTS update_at();
DROP TABLE IF EXISTS "session";
DROP TABLE IF EXISTS "users";
