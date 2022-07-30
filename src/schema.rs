table! {
    readings (id) {
        id -> Int4,
        temperature -> Nullable<Float8>,
        humidity -> Nullable<Float8>,
        dust_concentration -> Nullable<Float8>,
        pressure -> Nullable<Int4>,
        air_purity -> Nullable<Varchar>,
    }
}
