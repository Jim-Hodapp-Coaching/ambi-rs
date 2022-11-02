use ambi_rs::rocket_builder;
use diesel::prelude::*;

#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    let _ = rocket_builder().launch().await?;
    Ok(())
}
