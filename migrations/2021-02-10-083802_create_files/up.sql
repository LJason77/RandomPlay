CREATE TABLE IF NOT EXISTS files
(
    id        INTEGER     NOT NULL PRIMARY KEY,
    path      VARCHAR     NOT NULL UNIQUE
);
