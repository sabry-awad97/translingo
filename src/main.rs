use config::TranslateOptions;

use crate::translator::translate;

mod config;
mod error;
mod model;
mod translator;
mod utils;

#[tokio::main]
async fn main() {
    let options = TranslateOptions::new(Some("en"), None);
    let response = translate("Hello world", options).await;
    println!("Response: {:?}", response);
}
