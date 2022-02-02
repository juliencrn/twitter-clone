# Twitter Clone

A Rest API as an "Hello world" with Rust, Actix-web and PostgreSQL following this [tutorial](https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/).

## API Design

```bash
/tweets
    GET: list last 50 tweets
    POST: create a new tweet
/tweets/:id
    GET: find a tweet by its ID
    DELETE: delete a tweet by its ID
/tweets/:id/likes
    GET: list all likes attached to a tweet
    POST: add +1 like to a tweet
    DELETE: add -1 like to a tweet
```

## Install

This project requires:

- Rustc - cargo installed
- PostgreSQL running server
- diesel_cli

### 1. Setup the PostgreSQL database (on macOS)

```bash
# Install Postgres
brew install postgres

# start Postgres database
brew services start postgresql
# and $ brew services stop postgresql # to stop

# Create main user and set role
psql postgres
# Then type this in the interactive term
> CREATE ROLE username WITH LOGIN PASSWORD 'password';
> ALTER ROLE username CREATEDB;

# Then type \q + Enter to quit.
# Install pgAdmin 4

# Create migration (if doesn't exists)
# diesel migration generate create_tweets
# diesel migration generate create_likes

# Exec migration
diesel migration run
diesel migration redo
```

```bash
# Setup database
diesel setup

# Launch dev server
cargo run

# Tests
cargo test
```
