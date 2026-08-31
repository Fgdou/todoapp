-- Your SQL goes here
CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username VARCHAR NOT NULL,
    CONSTRAINT UC_user UNIQUE (username)
);

CREATE TABLE user_token (
    token VARCHAR NOT NULL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    CONSTRAINT UC_token UNIQUE (token)
);