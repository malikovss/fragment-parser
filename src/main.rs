use reqwest;
use scraper::{Html, Selector};
use serde::Deserialize;
use std::time::Instant;

#[derive(Debug, Deserialize)]
struct TonRate {
    usd_price: f64,
}
fn main() {
    const RATES_API_URL: &str = "https://price-api.crypto.com/price/v1/tokens/toncoin";
    let start_time = Instant::now();
    let client = reqwest::blocking::Client::new();
    let ton_rate: TonRate = client
        .get(RATES_API_URL)
        .header("User-Agent", "PostmanRuntime/7.32.2")
        .send()
        .unwrap()
        .json::<TonRate>()
        .unwrap();
    let response =
        reqwest::blocking::get("https://fragment.com/numbers?sort=price_asc&filter=sale")
            .unwrap()
            .text()
            .unwrap();

    println!("1 TON = {:.2?} USD", ton_rate.usd_price);

    let document = Html::parse_document(&response);
    let tr_selector = Selector::parse("table>tbody.tm-high-cells>tr.tm-row-selectable").unwrap();
    let trs = document.select(&tr_selector).take(10);
    for num in trs {
        let number_value: String = num
            .select(&Selector::parse("div.table-cell-value.tm-value").unwrap())
            .next()
            .unwrap()
            .inner_html()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();
        let number_price = num
            .select(&Selector::parse("div.table-cell-value.tm-value.icon-before.icon-ton").unwrap())
            .next()
            .unwrap()
            .inner_html();
        println!(
            "{} | {} TON | {:.2?} USD",
            number_value,
            number_price,
            number_price.parse::<f64>().unwrap() * ton_rate.usd_price
        );
    }

    let elapsed_time = start_time.elapsed();
    println!("Took {:.2?}", elapsed_time);
}
