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
