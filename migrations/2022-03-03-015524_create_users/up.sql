CREATE TABLE users (
    name        VARCHAR(60)             NOT NULL,
    handle      VARCHAR(60) PRIMARY KEY NOT NULL,
    created     TIMESTAMP DEFAULT now() NOT NULL
);