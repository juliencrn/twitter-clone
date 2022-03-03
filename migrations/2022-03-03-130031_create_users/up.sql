CREATE TABLE users (
    id          UUID PRIMARY KEY        NOT NULL,
    name        VARCHAR(60)             NOT NULL,
    handle      VARCHAR(60) UNIQUE      NOT NULL,
    created     TIMESTAMP DEFAULT now() NOT NULL
);