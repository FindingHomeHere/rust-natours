// @generated automatically by Diesel CLI.

diesel::table! {
    tours (id) {
        id -> Int4,
        name -> Text,
        ratings_avg -> Nullable<Float8>,
        ratings_qty -> Nullable<Int4>,
        images -> Nullable<Array<Nullable<Text>>>,
        image_cover -> Nullable<Text>,
        start_dates -> Nullable<Array<Nullable<Date>>>,
        duration -> Nullable<Int4>,
        max_group_size -> Nullable<Int4>,
        difficulty -> Nullable<Text>,
        price -> Nullable<Int4>,
        summary -> Nullable<Text>,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}
