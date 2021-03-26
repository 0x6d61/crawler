use reqwest::blocking::ClientBuilder;
use url::Url;
use crawler::LinkEtractor;
use crawler::crawler::Crawler;
use std::time::Duration;

fn main() ->  eyre::Result<()> {
    env_logger::init();
    let url = std::env::args()
            .nth(1)
            .unwrap_or("https://www.rust-lang.org".to_owned());
    let url = Url::parse(&url)?;
    let client = ClientBuilder::new().build()?;

    let extractor = LinkEtractor::from_client(client);
    let crawler = Crawler::new(&extractor,url);
    let wait = Duration::from_millis(100);
    for url in crawler.take(10) {
        println!("{}",url);
        std::thread::sleep(wait.clone());
    }

    Ok(())
}