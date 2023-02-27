use regex::Regex;
use reqwest::Client;
use serde_json::Value;
use thiserror::Error;
use url::Url;

const DEFAULT_FROM: &str = "auto";
const DEFAULT_TO: &str = "ar";
const DEFAULT_HOST: &str = "https://translate.googleapis.com";

#[derive(Debug, Error)]
pub enum TranslateError {
    #[error("HTTP error: {status} - {message}")]
    HttpError {
        status: u16,
        message: String,
        ip_address: String,
        time: String,
        url: String,
    },
    #[error("Google Translate API error: {0}")]
    ApiError(String),
    #[error("Reqwest error: {0}")]
    ReqwestError(reqwest::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Failed to build URL: {0}")]
    UrlBuildError(#[from] url::ParseError),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone)]
pub struct TranslateOptions {
    source_lang: String,
    target_lang: String,
    host: String,
}

impl TranslateOptions {
    fn to_query_params(&self) -> Vec<(&'static str, String)> {
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

impl Default for TranslateOptions {
    fn default() -> Self {
        Self {
            source_lang: DEFAULT_FROM.to_string(),
            target_lang: DEFAULT_TO.to_string(),
            host: DEFAULT_HOST.to_string(),
        }
    }
}

pub struct Translator {
    client: Client,
    input_text: String,
    options: TranslateOptions,
}

impl Translator {
    pub fn new(input_text: String, options: Option<TranslateOptions>) -> Self {
        Self {
            input_text,
            options: options.unwrap_or_default(),
            client: Client::new(),
        }
    }

    pub async fn translate(&self) -> Result<Value, TranslateError> {
        let body = format!("q={}", self.input_text);

        let api_endpoint = format!("{}/translate_a/single", self.options.host);
        let url = Url::parse_with_params(&api_endpoint, self.options.to_query_params())?;
        let res = self
            .client
            .post(url)
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=utf-8",
            )
            .body(body)
            .send()
            .await
            .map_err(TranslateError::ReqwestError)?;
        if !res.status().is_success() {
            let status = res.status().as_u16();
            let message = res.status().canonical_reason().unwrap_or("").to_string();
            let text = res.text().await.unwrap();
            let (ip_address, time, url) = extract_too_many_requests_info(&text);
            return Err(TranslateError::HttpError {
                status,
                message,
                ip_address,
                time,
                url,
            });
        }
        let text = res
            .text()
            .await
            .map_err(|err| TranslateError::Other(err.to_string()))?;
        let raw = serde_json::from_str::<Value>(&text).map_err(TranslateError::SerdeError)?;
        if let Some(error) = raw.get("error") {
            return Err(TranslateError::ApiError(
                error
                    .get("message")
                    .map_or_else(|| "".to_string(), |v| v.to_string()),
            ));
        }
        Ok(raw)
    }
}

#[tokio::main]
async fn main() {
    let translator = Translator::new("Hello world".to_string(), Some(TranslateOptions::default()));
    let response = translator.translate().await;
    println!("Response: {:?}", response);
}

pub fn extract_too_many_requests_info(html: &str) -> (String, String, String) {
    let ip_regex = Regex::new(r"IP address: (.+?)<br>").unwrap();
    let ip = ip_regex
        .captures(html)
        .map_or(String::new(), |cap| cap[1].to_string());

    let time_regex = Regex::new(r"Time: (.+?)<br>").unwrap();
    let time = time_regex
        .captures(html)
        .map_or(String::new(), |cap| cap[1].to_string());

    let url_regex = Regex::new(r"URL: (.+?)<br>").unwrap();
    let url = url_regex
        .captures(html)
        .map_or(String::new(), |cap| cap[1].to_string())
        .replace("&amp;", "&");

    (ip, time, url)
}
