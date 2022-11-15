use ambi_rs::rocket_builder;

#[rocket::main]
async fn main() -> Result<(), rocket::Error>{
    let _ = rocket_builder().launch().await?;
    Ok(())
}
