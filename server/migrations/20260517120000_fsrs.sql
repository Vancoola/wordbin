CREATE TABLE cards
(
    word_id     INTEGER PRIMARY KEY REFERENCES words (id) ON DELETE CASCADE,
    stability   REAL,
    difficulty  REAL,
    lapses      INTEGER  NOT NULL DEFAULT 0,
    last_review DATETIME,
    next_review DATETIME
);

CREATE INDEX idx_cards_next_review ON cards (next_review);

CREATE TABLE reviews
(
    id          INTEGER PRIMARY KEY,
    word_id     INTEGER  NOT NULL REFERENCES words (id) ON DELETE CASCADE,
    rating      INTEGER  NOT NULL CHECK (rating BETWEEN 1 AND 4),
    delta_t     INTEGER  NOT NULL,
    reviewed_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_reviews_word_at ON reviews (word_id, reviewed_at);
