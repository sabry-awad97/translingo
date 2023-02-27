const DEFAULT_FROM: &str = "auto";
const DEFAULT_TO: &str = "ar";
const DEFAULT_HOST: &str = "https://translate.googleapis.com";

#[derive(Debug)]
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
    pub fn new(source_language: Option<&str>, target_language: Option<&str>) -> Self {
        Self {
            source_lang: source_language.unwrap_or(DEFAULT_FROM).to_string(),
            target_lang: target_language.unwrap_or(DEFAULT_TO).to_string(),
            host: DEFAULT_HOST.to_string(),
        }
    }
}
