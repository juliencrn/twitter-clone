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
    users (id) {
        id -> Uuid,
        name -> Varchar,
        handle -> Varchar,
        created -> Timestamp,
        password -> Varchar,
    }
}

joinable!(tweets -> users (author));

allow_tables_to_appear_in_same_query!(
    tweets,
    users,
);
