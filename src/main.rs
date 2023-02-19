use clap::Parser;
use reqwest::get;
use reqwest::Url;
use scraper::Html;
use scraper::Selector;
use selector_grabber::download_file;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Parser)]
#[command(
    author = "lordxan",
    version = "0.1.0",
    about = "Download file from item attribute on the page using selector matching"
)]
struct Cli {
    /// URL to webpage
    url: Url,
    /// CSS selector to element
    selector: String,
    /// Attribute to extract url from
    #[arg(short, long, default_value_t = ("src").to_string())]
    attribute: String,
}

#[tokio::main]
async fn main() {
    let Cli {
        url,
        selector,
        attribute,
    } = Cli::parse();
    let selector = Selector::parse(&selector).expect("Selector is invalid");
    let response = get(url).await.expect("Url is invalid");
    let page = response.text().await.expect("Not a html page");
    let document = Html::parse_document(&page);
    let elements = document.select(&selector).collect::<Vec<_>>();
    if let 0 = elements.len() {
        println!("No items found")
    }
    for element in elements {
        match download_file(element, attribute.to_owned()).await {
            Ok((bytes, filename)) => {
                let mut file = File::create(filename).expect("Error creating file");
                file.write_all(&bytes).unwrap();
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
    }
}
