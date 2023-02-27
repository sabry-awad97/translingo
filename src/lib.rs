pub mod config;
pub mod error;
pub mod model;
pub mod translation_cache;
pub mod translator;
pub mod utils;

use std::{
    error::Error,
    fs::File,
    io::{self, Read},
};

use structopt::StructOpt;

use crate::{config::TranslateOptions, translator::translate};

#[derive(StructOpt, Debug)]
#[structopt(name = "translator", about = "A command line translator tool")]
pub struct Cli {
    #[structopt(help = "The input source (file path or \"-\" for stdio)")]
    pub input_source: Option<String>,

    #[structopt(short = "f", long = "from", help = "The source language code")]
    pub from: Option<String>,

    #[structopt(short = "t", long = "to", help = "The target language code")]
    pub to: Option<String>,
}

enum InputSource {
    File(String),
    Stdio,
}

impl Cli {
    fn get_input_source(&self) -> Result<InputSource, std::io::Error> {
        match self.input_source.as_deref() {
            None | Some("-") => Ok(InputSource::Stdio),
            Some(file_path) => Ok(InputSource::File(file_path.to_owned())),
        }
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let input_source = args.get_input_source()?;
    let input = match input_source {
        InputSource::File(file_path) => {
            let mut file = File::open(file_path)?;
            let mut input = String::new();
            file.read_to_string(&mut input)?;
            input
        }
        InputSource::Stdio => {
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;
            input
        }
    };

    let options = TranslateOptions::new(args.from, args.to);
    let response = translate(&input, options).await?;
    println!("{}", serde_json::to_string(&response)?);

    Ok(())
}
