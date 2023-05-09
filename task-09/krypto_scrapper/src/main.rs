extern crate reqwest;
extern crate scraper;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let url = "https://crypto.com/price";
    let response = reqwest::blocking::get(url).unwrap();
    let body = response.text().unwrap();
    let document = Html::parse_document(&body);

    let mut csv_data: Vec<Vec<String>> = Vec::new();

    // selector for prices
    let selector_prices = Selector::parse(".chakra-text.css-13hqrwd").unwrap(); 
    let mut prices: Vec<String> = Vec::new();
    for element in document.select(&selector_prices) {
        prices.push(element.text().collect::<String>());
    }
    csv_data.push(prices);

    // selector for names
    let selector_names = Selector::parse(".chakra-text.css-rkws3").unwrap(); 
    let mut names: Vec<String> = Vec::new();
    for element in document.select(&selector_names) {
        names.push(element.text().collect::<String>());
    }
    csv_data.push(names);

    // selector for 24H change
    let selector_changes = Selector::parse(".chakra-text.css-yyku61").unwrap(); 
    let mut changes: Vec<String> = Vec::new();
    for element in document.select(&selector_changes) {
        changes.push(element.text().collect::<String>());
    }
    csv_data.push(changes);

    // selector for 24H volume
    let selector_volumes = Selector::parse(".css-1nh9lk8:nth-child(3n+1)").unwrap(); 
    let mut volumes: Vec<String> = Vec::new();
    for element in document.select(&selector_volumes) {
        volumes.push(element.text().collect::<String>());
    }
    csv_data.push(volumes);

    // selector for market cap
    let selector_caps = Selector::parse(".css-1nh9lk8:nth-child(3n+2)").unwrap(); 
    let mut caps: Vec<String> = Vec::new();
    for element in document.select(&selector_caps) {
        caps.push(element.text().collect::<String>());
    }
    csv_data.push(caps);

    // write to CSV file
    let mut file = File::create("crypto_data.csv").unwrap();
    for row in csv_data {
        let csv_row = row.join(",");
        file.write_all(csv_row.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();
    }

    println!("Data written to crypto_data.csv");
}
