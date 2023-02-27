pub mod config;
pub mod error;
pub mod model;
pub mod translation_cache;
pub mod translator;
pub mod utils;

use std::error::Error;

use structopt::StructOpt;

use crate::{config::TranslateOptions, translator::translate};

#[derive(StructOpt, Debug)]
#[structopt(name = "translator", about = "A command line translator tool")]
pub struct Cli {
    #[structopt(help = "The input text to translate")]
    pub input: String,

    #[structopt(short = "f", long = "from", help = "The source language code")]
    pub from: Option<String>,

    #[structopt(short = "t", long = "to", help = "The target language code")]
    pub to: Option<String>,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let options = TranslateOptions::new(args.from, args.to);
    let response = translate(&args.input, options).await?;
    println!("{}", serde_json::to_string(&response)?);

    Ok(())
}
