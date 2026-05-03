diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        handle -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        banner_url -> Nullable<Text>,
        is_verified -> Bool,
        is_private -> Bool,
        follower_count -> Int4,
        following_count -> Int4,
        tweet_count -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}
