# Twitter Clone

I'm learning Rust. The first version of this project was just a basic CRUD application to create a REST API connected to a SQL database following this [tutorial](https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/).

Now, we will create a better version more close than a real-world application with user auth, an API closest than the real Twitter API and a bigger focus on the database structure and tables relations. I want to build this by myself, without any step-by-step tutorial, but I will use the data-structure model from this [post](https://docs.fauna.com/fauna/current/learn/sample_apps/fwitter),  that seems to be a good schema.

The final model for the application looks like this:

(but I still will use "tweet" instead of "fweet" :) )

![model](.github/model.svg)

## API Design


all routes are prefixed by `/api/v1`.

```bash
/tweets
    GET /: list last 50 tweets
    GET /{id}: get tweet by id
    POST /: create a new tweet (require auth)
    DELETE /{id}: delete a tweet by id (require auth)
/users
    GET /: list all users
    GET /{id}: get user by id
    UPDATE /{id}: update user by id (require auth)
    DELETE /{id}: delete a tweet by id (require auth)
/profile
    GET /: get logged-in user (require auth)
/auth
    POST /register: Create an user
    POST /login: Return the json web token
```

## Installation

### Requirements

- Set `POSTGRES_USER`, `POSTGRES_PASSWORD` and `POSTGRES_DB` in an `.env` file.
- Docker
- Rust

```sh
# Install the tools (the first time only)
cargo install cargo-watch
cargo install diesel_cli --no-default-features --features postgres

# Create the docker volume (the first time only)
docker volume create --name=twitter-db

# Run the db container
docker-compose up -d

# Setup the db (the first time only)
diesel setup
diesel migration run

# Run the app in watch mode
cargo watch -x run

# Stop the db
docker-compose down
```
