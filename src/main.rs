use clap::Parser;
use reqwest::get;
use reqwest::Url;
use scraper::Html;
use scraper::Selector;
use selector_grabber::download_file;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    url: Url,
    selector: String,
}

#[tokio::main]
async fn main() {
    let Cli { url, selector } = Cli::parse();
    let selector = Selector::parse(&selector).expect("Selector is invalid");
    let response = get(url).await.expect("Url is invalid");
    let page = response.text().await.expect("Not a html page");
    let document = Html::parse_document(&page);
    let elements = document.select(&selector);
    for element in elements {
        match download_file(element).await {
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
