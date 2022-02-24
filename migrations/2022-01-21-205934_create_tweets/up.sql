CREATE TABLE tweets (
    id         UUID PRIMARY KEY        NOT NULL,
    message    VARCHAR(280)            NOT NULL,
    likes      INT                     NOT NULL,
    retweets   INT                     NOT NULL,
    comments   INT                     NOT NULL,
    created    TIMESTAMP DEFAULT now() NOT NULL,
    asset      TEXT                    NOT NULL
);