-- Your SQL goes here
CREATE TABLE ingredients
(
    id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    description TEXT,
    category_id INTEGER NOT NULL,
    FOREIGN KEY (category_id) REFERENCES categories (id)
);