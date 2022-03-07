CREATE TABLE users (
    id          UUID PRIMARY KEY         NOT NULL,
    name        VARCHAR(100)             NOT NULL,
    handle      VARCHAR(100) UNIQUE      NOT NULL,
    created     TIMESTAMP DEFAULT now()  NOT NULL,
    password    VARCHAR(255)             NOT NULL
);

CREATE TABLE tweets (
    id         UUID PRIMARY KEY        NOT NULL,
    message    VARCHAR(280)            NOT NULL,
    author     UUID                    NOT NULL REFERENCES users(id),
    likes      INT                     NOT NULL,
    retweets   INT                     NOT NULL,
    comments   INT                     NOT NULL,
    created    TIMESTAMP DEFAULT now() NOT NULL,
    asset      TEXT                    NOT NULL
);
