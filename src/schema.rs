// @generated automatically by Diesel CLI.

diesel::table! {
    brands (id) {
        id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    paints (id) {
        id -> Int4,
        name -> Text,
        brand -> Int4,
        #[max_length = 9]
        color -> Nullable<Varchar>,
    }
}

diesel::table! {
    paints_2_storage_boxes (id) {
        id -> Int4,
        number -> Int4,
        paint -> Int4,
        storage_box -> Int4,
    }
}

diesel::table! {
    storage_boxes (id) {
        id -> Int4,
        name -> Text,
        flags -> Nullable<Text>,
    }
}

diesel::joinable!(paints -> brands (brand));
diesel::joinable!(paints_2_storage_boxes -> paints (paint));
diesel::joinable!(paints_2_storage_boxes -> storage_boxes (storage_box));

diesel::allow_tables_to_appear_in_same_query!(
    brands,
    paints,
    paints_2_storage_boxes,
    storage_boxes,
);
