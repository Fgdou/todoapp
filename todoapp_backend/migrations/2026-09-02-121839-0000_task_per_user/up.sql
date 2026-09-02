-- Your SQL goes here

INSERT INTO users (id, username, password)
SELECT 0, 'test', '37268335dd6931045bdcdf92623ff819a64244b53d0e746d438797349d4da578'
WHERE NOT EXISTS (
    SELECT 1 FROM users WHERE id = 0
);

CREATE TABLE tasks_new (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    description TEXT NOT NULL DEFAULT "",
    done BOOLEAN NOT NULL DEFAULT False,
    user_id INTEGER NOT NULL REFERENCES users(id)
);

INSERT INTO tasks_new (id, title, done, user_id)
SELECT id, title, done, 0 FROM tasks;

DROP TABLE tasks;
ALTER TABLE tasks_new RENAME TO tasks;