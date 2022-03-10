table! {
    hashtags (id) {
        id -> Uuid,
        name -> Varchar,
        created -> Timestamp,
    }
}

table! {
    tweet_hashtags (id) {
        id -> Uuid,
        hashtag_id -> Uuid,
        tweet_id -> Uuid,
    }
}

table! {
    tweets (id) {
        id -> Uuid,
        message -> Varchar,
        author -> Uuid,
        likes -> Int4,
        retweets -> Int4,
        comments -> Int4,
        created -> Timestamp,
        asset -> Text,
    }
}

table! {
    user_accounts (id) {
        id -> Uuid,
        email -> Varchar,
        user_id -> Uuid,
        password -> Varchar,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        handle -> Varchar,
        created -> Timestamp,
    }
}

joinable!(tweet_hashtags -> hashtags (hashtag_id));
joinable!(tweet_hashtags -> tweets (tweet_id));
joinable!(tweets -> users (author));
joinable!(user_accounts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    hashtags,
    tweet_hashtags,
    tweets,
    user_accounts,
    users,
);
