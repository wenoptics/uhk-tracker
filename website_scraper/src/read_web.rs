use scraper::{Html, Selector};


use crate::models::ShipmentUpdate;


pub async fn collect_from_website() -> Result<(), Box<dyn std::error::Error>> {

    let html = reqwest::get("https://ultimatehackingkeyboard.com/delivery-status")
        .await?
        .text()
        .await?;
    // println!("{:#?}", html);

    let document = Html::parse_document(&html);
    let sel = Selector::parse("html>body main div.post-content").unwrap();
    let main_el = document.select(&sel).next().unwrap();

    let text = main_el.text().collect::<Vec<_>>();
    println!("{}", text.iter()
        .map(|p| p.trim())
        .collect::<Vec<_>>()
        .join("\n")
    );

    Ok(())
}
