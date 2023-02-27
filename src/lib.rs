pub mod config;
pub mod error;
pub mod model;
pub mod translation_cache;
pub mod translator;
pub mod utils;
pub mod language;

use std::error::Error;
use std::io::BufRead;

use config::TranslationConfig;

use crate::{config::TranslateOptions, translator::translate};

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = TranslationConfig::from_args();
    let reader = args.get_input_source()?;
    let options = TranslateOptions::new(args.from, args.to);
    for line_result in reader.lines() {
        let line = line_result?;
        let response = translate(&line, options.clone()).await?;
        println!("{}", serde_json::to_string(&response)?);
    }
    Ok(())
}
