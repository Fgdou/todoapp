-- diesel:run:no_transaction

PRAGMA foreign_keys = OFF;

CREATE TABLE users_dg_tmp (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    username VARCHAR NOT NULL,
    password VARCHAR,
    CONSTRAINT UC_user UNIQUE (username)
);

INSERT INTO users_dg_tmp (id, username, password)
SELECT id, username, password FROM users;

DROP TABLE users;

ALTER TABLE users_dg_tmp RENAME TO users;

PRAGMA foreign_keys = ON;