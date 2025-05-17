-- Add migration script here
CREATE TABLE job (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  uri STRING UNIQUE NOT NULL
)
  
