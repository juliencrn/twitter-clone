CREATE TABLE users (
    id          UUID PRIMARY KEY        NOT NULL,
    name        VARCHAR(60)             NOT NULL,
    handle      VARCHAR(60) UNIQUE      NOT NULL,
    created     TIMESTAMP DEFAULT now() NOT NULL
);

CREATE TABLE hashtags (
    id          UUID PRIMARY KEY        NOT NULL,
    name        VARCHAR(100) UNIQUE     NOT NULL,
    created     TIMESTAMP DEFAULT now() NOT NULL
);

CREATE TABLE tweets (
    id          UUID PRIMARY KEY        NOT NULL,
    message     VARCHAR(280)            NOT NULL,
    author      UUID                    NOT NULL REFERENCES users(id),
    likes       INT                     NOT NULL,
    retweets    INT                     NOT NULL,
    comments    INT                     NOT NULL,
    created     TIMESTAMP DEFAULT now() NOT NULL,
    asset       TEXT                    NOT NULL
);

CREATE TABLE tweet_hashtags (
    id          UUID PRIMARY KEY        NOT NULL,
    hashtag_id  UUID                    NOT NULL REFERENCES hashtags(id),
    tweet_id    UUID                    NOT NULL REFERENCES tweets(id)
);

CREATE TABLE user_accounts (
    id          UUID PRIMARY KEY        NOT NULL,
    email       VARCHAR(100) UNIQUE     NOT NULL,
    user_id     UUID                    NOT NULL REFERENCES users(id),
    password    VARCHAR(255)            NOT NULL
);
