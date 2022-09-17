use std::error::Error;
use tokio;

mod read_web;
mod persistence;
mod models;


async fn scrap_then_save() -> Result<(), Box<dyn Error>> {
    let data_point = read_web::collect_from_website().await.unwrap();
    persistence::save_to_db(&data_point).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    scrap_then_save().await?;
    Ok(())
}
