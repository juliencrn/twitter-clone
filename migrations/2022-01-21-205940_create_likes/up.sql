CREATE TABLE IF NOT EXISTS likes (
    id         UUID PRIMARY KEY        NOT NULL,
    created_at TIMESTAMP DEFAULT now() NOT NULL,
    tweet_id   UUID                    NOT NULL REFERENCES tweets (id)
);