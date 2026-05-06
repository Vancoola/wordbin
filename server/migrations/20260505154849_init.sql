CREATE TABLE words
(
    id       INTEGER PRIMARY KEY,
    word     TEXT NOT NULL,
    source   TEXT,                   -- "netflix", "browser", "manual"
    added_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    notes    TEXT
);

CREATE UNIQUE INDEX idx_words_word ON words(word);