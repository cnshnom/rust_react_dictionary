// @generated automatically by Diesel CLI.

diesel::table! {
    word_pairs (id) {
        id -> Text,
        german -> Text,
        chinese -> Text,
    }
}
