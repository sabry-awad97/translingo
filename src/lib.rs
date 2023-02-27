pub mod config;
pub mod error;
pub mod model;
pub mod translation_cache;
pub mod translator;
pub mod utils;

use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

use crate::{config::TranslateOptions, translator::translate};

pub async fn run() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();
    let reader = args.get_input_source()?;
    let options = TranslateOptions::new(args.from, args.to);
    for line_result in reader.lines() {
        let line = line_result?;
        let response = translate(&line, options.clone()).await?;
        println!("{}", serde_json::to_string(&response)?);
    }
    Ok(())
}

#[derive(StructOpt, Debug)]
#[structopt(name = "translator", about = "A command line translator tool")]
pub struct Cli {
    #[structopt(parse(from_os_str), help = "The input file path, or - for stdin")]
    pub input_source: Option<PathBuf>,

    #[structopt(short = "f", long = "from", help = "The source language code")]
    pub from: Option<String>,

    #[structopt(short = "t", long = "to", help = "The target language code")]
    pub to: Option<String>,
}

impl Cli {
    fn get_input_source(&self) -> io::Result<Box<dyn BufRead>> {
        match self.input_source.as_deref() {
            Some(file) => {
                let file = File::open(file)?;
                let reader = BufReader::new(file);
                Ok(Box::new(reader))
            }

            None => {
                let stdin = io::stdin();
                let reader = BufReader::new(stdin);
                Ok(Box::new(reader))
            }
        }
    }
}
