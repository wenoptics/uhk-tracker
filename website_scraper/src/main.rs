use clokwerk::{AsyncScheduler, TimeUnits};
use std::error::Error;
use std::time::Duration;
use tokio;

mod models;
mod persistence;
mod read_web;

async fn scrap_then_save() -> Result<(), Box<dyn Error>> {
    let data_point = read_web::collect_from_website().await.unwrap();
    persistence::save_to_db(&data_point).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Run once at startup
    scrap_then_save().await?;

    // Schedule to run daily
    let mut scheduler = AsyncScheduler::new();
    scheduler
        .every(1.day())
        .run(|| async { scrap_then_save().await.unwrap_or(()) });
    println!("Task is scheduled.");

    // Manually run the scheduler forever
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_secs(60 * 60)).await;
    }
}
