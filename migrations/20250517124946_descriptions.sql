-- Add migration script here
ALTER TABLE job ADD COLUMN title TEXT;
ALTER TABLE job ADD COLUMN preface TEXT;
ALTER TABLE job ADD COLUMN description TEXT;
