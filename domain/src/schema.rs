diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        genre -> Varchar,
        published -> Bool,
    }
}