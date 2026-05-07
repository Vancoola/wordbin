CREATE TABLE words
(
    id       INTEGER PRIMARY KEY,
    word     TEXT NOT NULL,
    source   TEXT NOT NULL ,                   
    added_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    status   TEXT NOT NULL CHECK (status IN ('known', 'learning', 'new')),
    notes    TEXT
);

CREATE UNIQUE INDEX idx_words_word ON words(word);

CREATE TABLE reviews
(
    id          INTEGER PRIMARY KEY,
    word_id     INTEGER NOT NULL REFERENCES words (id) ON DELETE CASCADE,
    result      TEXT NOT NULL CHECK (result IN ('easy', 'hard', 'forgot')),
    reviewed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_reviews_word_id ON reviews (word_id);