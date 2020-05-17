table! {
    brands (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    cars (id) {
        id -> Int4,
        engine_id -> Nullable<Int4>,
        brand_id -> Nullable<Int4>,
        price -> Float8,
        release_year -> Int4,
    }
}

table! {
    engines (id) {
        id -> Int4,
        name -> Varchar,
    }
}

joinable!(cars -> brands (brand_id));
joinable!(cars -> engines (engine_id));

allow_tables_to_appear_in_same_query!(
    brands,
    cars,
    engines,
);
