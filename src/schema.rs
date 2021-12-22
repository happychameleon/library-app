table! {
    authors (id) {
        id -> Integer,
        olid -> Text,
        name -> Text,
    }
}

table! {
    books (id) {
        id -> Integer,
        olid -> Text,
        uid -> Text,
        isbn -> Nullable<Text>,
        title -> Text,
        author -> Nullable<Text>,
        work -> Nullable<Text>,
        covers -> Nullable<Text>,
    }
}

table! {
    works (id) {
        id -> Integer,
        olid -> Text,
        title -> Text,
        author -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(authors, books, works,);
