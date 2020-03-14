extern crate pocket;
extern crate hyper;

use pocket::Pocket;
use hyper::client::IntoUrl;

fn main() {
    let pocket = Pocket::new(
        &std::env::var("POCKET_CONSUMER_KEY").unwrap(),
        &std::env::var("POCKET_ACCESS_TOKEN").unwrap(),
    );

    let item = pocket.add(
        "https://example.com".into_url().unwrap(),
        Some("Example title"),
        Some("example-tag"),
        Some("example_tweet_id"),
    ).unwrap();
    println!("item: {:?}", item);
}