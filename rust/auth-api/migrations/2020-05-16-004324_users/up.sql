CREATE TABLE users (
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  encrypted_password VARCHAR(122) NOT NULL,
  created_at TIMESTAMP NOT NULL
);
