table! {
    customer_license (id) {
        id -> Int4,
        customer_name -> Varchar,
        address -> Nullable<Varchar>,
        license_id -> Int4,
        license_code -> Varchar,
        start_date -> Timestamp,
        end_date -> Timestamp,
        number_of_nodes -> Int4,
    }
}

table! {
    license (id) {
        id -> Int4,
        name -> Varchar,
        duration -> Int4,
        number_of_nodes -> Int4,
    }
}

joinable!(customer_license -> license (license_id));

allow_tables_to_appear_in_same_query!(
    customer_license,
    license,
);
