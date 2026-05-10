CREATE TABLE words
(
    id       INTEGER PRIMARY KEY,
    word     TEXT                               NOT NULL,
    source   TEXT                               NOT NULL,
    added_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    status   TEXT                               NOT NULL CHECK (status IN ('known', 'learning', 'new')),
    notes    TEXT
);

CREATE UNIQUE INDEX idx_words_word ON words (word);

CREATE TABLE reviews
(
    id          INTEGER PRIMARY KEY,
    word_id     INTEGER NOT NULL REFERENCES words (id) ON DELETE CASCADE,
    result      TEXT    NOT NULL CHECK (result IN ('easy', 'hard', 'forgot')),
    reviewed_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_reviews_word_id ON reviews (word_id);


CREATE TABLE tokens
(
    id         INTEGER PRIMARY KEY,
    name       TEXT                               NOT NULL,
    token_hash TEXT                               NOT NULL UNIQUE,
    is_revoked INTEGER  DEFAULT 0                 NOT NULL CHECK (is_revoked IN (0, 1)),
    is_admin   INTEGER  DEFAULT 0                 NOT NULL CHECK (is_admin IN (0, 1)),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    expires_at DATETIME                           NOT NULL
);

CREATE INDEX idx_tokens_hash_revoked ON tokens (token_hash, is_revoked);
CREATE INDEX idx_tokens_expires_at ON tokens (expires_at);