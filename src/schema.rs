// @generated automatically by Diesel CLI.

table! {
    readings (id) {
        id -> Int4,
        temperature -> Float8,
        humidity -> Float8,
        dust_concentration -> Float8,
        pressure -> Int4,
        air_purity -> Varchar,
    }
}
