CREATE TABLE users (
    id          UUID PRIMARY KEY        NOT NULL,
    name        VARCHAR(100)            NOT NULL,
    handle      VARCHAR(100) UNIQUE     NOT NULL,
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

CREATE TABLE user_accounts (
    id          UUID PRIMARY KEY        NOT NULL,
    email       VARCHAR(255) UNIQUE     NOT NULL,
    user_id     UUID                    NOT NULL REFERENCES users(id),
    password    VARCHAR(255)            NOT NULL
);
