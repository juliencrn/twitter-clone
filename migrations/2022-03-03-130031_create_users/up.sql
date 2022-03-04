CREATE TABLE users (
    id          UUID PRIMARY KEY         NOT NULL,
    name        VARCHAR(100)             NOT NULL,
    handle      VARCHAR(100) UNIQUE      NOT NULL,
    created     TIMESTAMP DEFAULT now()  NOT NULL,
    password    VARCHAR(255)             NOT NULL
);