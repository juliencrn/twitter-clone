# Models

```rust
/// User information
pub struct User {
    name: String,   // Mary
    handle: String, // @logiconly9
    icon: String,   // user1
}

/// Login methods, points to an unique user
pub struct Account {
    user: String,  // User
    email: String, // user@test.com
}

/// Tweet
pub struct Tweet {
    message: String,  // Hello world!
    author: String,   // User
    hashtags: String, // Hashtags
    likes: u32,       // 86
    retweets: u32,    // 6
    comments: u32,    // 17
    created: String,  // 2020-05-14T12:35:59.209273Z
    original: String, // Tweet
    asset: String,    /* json like:
                      {
                          "id": "fdjnfdsj",
                          "url": "https://res...",
                          "type": "image",
                          "cloudName": "ffdgsgbr"
                      }
                      */
}

pub struct Hashtags {
    name: String, // #hashtags
}

/// Save interaction statistics between an user and a tweet.
pub struct TweetStats {
    user: String,  // User
    tweet: String, // Tweet
    like: bool,    // User
    retweet: bool,
    comment: bool, // User
}

pub struct Comment {
    message: String, // Awesome!
    author: String,  // User
    tweet: String,   // Tweet
}

/// Save information about how much users interact with each other
pub struct FollowerStats {
    author: String,     // User
    follower: String,   // User
    post_likes: u32,    // 13
    post_retweets: u32, // 7
}
```
