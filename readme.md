# Twitter Clone

A Rest API as an "Hello world" with Rust, Actix-web and PostgreSQL following this [tutorial](https://hub.qovery.com/guides/tutorial/create-a-blazingly-fast-api-in-rust-part-1/).

## API Design

```
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
