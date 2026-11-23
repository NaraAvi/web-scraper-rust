use anyhow::{Result};
use reqwest::blocking::Client;
use std::{
    collections::HashMap, fs, path::Path, time::{SystemTime, UNIX_EPOCH},
};
use scraper::{Html, Selector};

fn main() -> Result<()> {
    let client = Client::new();
    let url = "https://books.toscrape.com/";

    let document = Html::parse_document(&get_web_page(&client, url)?);

    let books_scraped: HashMap<String, String> = get_book_prices(document);

    for (k, v) in books_scraped.iter() {
        println!("{k} : {v}");
    }
    //write_file(res_body)?;
    
    Ok(())
}

fn get_book_prices(document: Html) -> HashMap<String, String> {
    let mut books_scraped: HashMap<String, String> = HashMap::new();

    let price_selector = Selector::parse("p.price_color").unwrap();

    let article_selector = Selector::parse("article.product_pod").unwrap();
    let h3_selector = Selector::parse("h3").unwrap();
    let anchor_selector = Selector::parse("a").unwrap();
    
    let mut titles: Vec<String> = vec![];
    let mut prices: Vec<String> = vec![];

    for article in document.select(&article_selector) {
        let heading = article.select(&h3_selector).next().unwrap(); //heading element
        let anchor = heading.select(&anchor_selector).next().unwrap();
        let title = anchor.value().attr("title").unwrap();
        titles.push(title.to_string());
    };
    for element in document.select(&price_selector) {
        let price = element.text().next().unwrap();
        prices.push(price.to_string());
    }
    for title in titles {
        books_scraped.insert(title, prices.iter_mut().next().unwrap().to_owned());
    };

    books_scraped
}

#[allow(dead_code)]
fn write_file(res_body: String) -> Result<(), anyhow::Error> {
    let path_dir = Path::new("./downloaded");
    fs::create_dir_all(path_dir)?;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = path_dir.join(format!("books-{timestamp}.html"));
    fs::write(path, &res_body)?;
    Ok(())
}

fn get_web_page(client: &Client, url: &str) -> Result<String> {
    let res = client
        .get(url)
        .send()?
        .error_for_status()?
        .text()?;
    Ok(res)
}
