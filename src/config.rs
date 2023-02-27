use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use structopt::StructOpt;

const DEFAULT_FROM: &str = "auto";
const DEFAULT_TO: &str = "ar";
const DEFAULT_HOST: &str = "https://translate.googleapis.com";
pub const CACHE_CAPACITY: usize = 1024;

#[derive(Debug, Clone)]
pub struct TranslateOptions {
    pub source_lang: String,
    pub target_lang: String,
    pub host: String,
}

impl TranslateOptions {
    pub fn to_query_params(&self) -> Vec<(&'static str, String)> {
        vec![
            ("client", String::from("gtx")),
            ("sl", String::from(&self.source_lang)),
            ("tl", String::from(&self.target_lang)),
            ("dt", String::from("t")),
            ("dt", String::from("rm")),
            ("dj", String::from("1")),
            ("ie", String::from("UTF-8")),
            ("oe", String::from("UTF-8")),
        ]
    }
}

impl TranslateOptions {
    pub fn new(source_language: Option<String>, target_language: Option<String>) -> Self {
        Self {
            source_lang: source_language.unwrap_or(DEFAULT_FROM.to_string()),
            target_lang: target_language.unwrap_or(DEFAULT_TO.to_string()),
            host: DEFAULT_HOST.to_string(),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "translator", about = "A command line translator tool")]
pub struct TranslationConfig {
    #[structopt(parse(from_os_str), help = "The input file path, or - for stdin")]
    pub input_source: Option<PathBuf>,

    #[structopt(short = "f", long = "from", help = "The source language code")]
    pub from: Option<String>,

    #[structopt(short = "t", long = "to", help = "The target language code")]
    pub to: Option<String>,
}

impl TranslationConfig {
    pub fn get_input_source(&self) -> io::Result<Box<dyn BufRead>> {
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

impl TranslationConfig {
    pub fn from_args() -> Self {
        Self::from_args_safe().unwrap_or_else(|e| e.exit())
    }
}
