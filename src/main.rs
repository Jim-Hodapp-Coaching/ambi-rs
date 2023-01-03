use ambi_rs::rocket_builder;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    env_logger::init();
    let _ = rocket_builder().launch().await?;
    Ok(())
}
