use chrono;
use regex::{CaptureMatches, Regex};
use scraper::{Html, Selector};

use crate::models::ShipmentUpdate;

pub async fn collect_from_website() -> Result<ShipmentUpdate, Box<dyn std::error::Error>> {
    let html = reqwest::get("https://ultimatehackingkeyboard.com/delivery-status")
        .await?
        .text()
        .await?;
    // println!("{:#?}", html);

    let document = Html::parse_document(&html);
    // Parse the main post section
    let sel = Selector::parse("html>body main div.post-content").unwrap();
    let main_el = document.select(&sel).next().unwrap();

    let text_arr = main_el.text().collect::<Vec<_>>();
    let text = text_arr
        .iter()
        .map(|p| p.trim())
        .collect::<Vec<_>>()
        .join("\n");

    // println!("{}", text);

    let mut ret = ShipmentUpdate {
        id: None,
        updated: chrono::Utc::now(),
        raw_post: text.clone(),
        order_range_l: None,
        order_range_r: None,
    };

    // Parse the order number range
    let re_order_num = Regex::new(r"#(\d+)").unwrap();
    let mut iter = re_order_num.captures_iter(&text);

    fn match_or_none(it: &mut CaptureMatches) -> Option<u32> {
        match it.next() {
            None => None,
            Some(cap) => match cap.get(1).unwrap().as_str().parse::<u32>() {
                Ok(num) => Some(num),
                Err(_) => None,
            },
        }
    }

    ret.order_range_l = match_or_none(&mut iter);
    ret.order_range_r = match_or_none(&mut iter);

    println!(
        "Parsed order begin: {:?} order end: {:?}",
        ret.order_range_l, ret.order_range_r
    );

    Ok(ret)
}
