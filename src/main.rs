use std::error::Error;

use config::TranslateOptions;

use crate::translator::translate;
use structopt::StructOpt;

mod config;
mod error;
mod model;
mod translator;
mod utils;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let options = TranslateOptions::new(args.from, args.to);
    let response = translate(&args.input, options).await?;
    println!("{}", serde_json::to_string(&response)?);

    Ok(())
}
