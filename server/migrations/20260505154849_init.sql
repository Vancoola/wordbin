CREATE TABLE words
(
    id       INTEGER PRIMARY KEY,
    word     TEXT NOT NULL,
    context  TEXT,
    source   TEXT,                   -- "netflix", "browser", "manual"
    status   TEXT     DEFAULT 'new', -- new | learning | known
    added_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    notes    TEXT
);