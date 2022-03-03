table! {
    tweets (id) {
        id -> Uuid,
        message -> Varchar,
        likes -> Int4,
        retweets -> Int4,
        comments -> Int4,
        created -> Timestamp,
        asset -> Text,
    }
}

table! {
    users (handle) {
        name -> Varchar,
        handle -> Varchar,
        created -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    tweets,
    users,
);
