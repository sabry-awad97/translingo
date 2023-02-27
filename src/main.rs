use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::IpAddr;
use thiserror::Error;

const DEFAULT_FROM: &str = "auto";
const DEFAULT_TO: &str = "fr";
const DEFAULT_HOST: &str = "translate.google.com";

#[derive(Debug, Serialize, Deserialize)]
pub struct TranslationResponse {
    pub sentences: Vec<Sentence>,
    pub src: String,
    pub confidence: f32,
    pub ld_result: LDResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LDResult {
    pub srclangs: Vec<String>,
    pub srclangs_confidences: Vec<f32>,
    pub extended_srclangs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sentence {
    pub trans: String,
    pub orig: String,
}

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
    SerdeError(serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

#[derive(Debug, Clone)]
pub struct TranslateOptions {
    from: String,
    to: String,
    host: String,
}

impl Default for TranslateOptions {
    fn default() -> Self {
        Self {
            from: DEFAULT_FROM.to_string(),
            to: DEFAULT_TO.to_string(),
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

    pub async fn translate(&self) -> Result<String, TranslateError> {
        let body = format!(
            "sl={}&tl={}&q={}",
            self.options.from, self.options.to, self.input_text
        );

        let ip_address = IpAddr::from([208, 126, 87, 161]);
        let res = self
            .client
            .post(&format!(
                "https://{}/translate_a/single?client=at&dt=t&dt=rm&dj=1",
                self.options.host
            ))
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded;charset=utf-8",
            )
            .header("X-Forwarded-For", ip_address.to_string())
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

        println!("{}", text);
        let raw = serde_json::from_str::<Value>(&text).map_err(TranslateError::SerdeError)?;

        if let Some(error) = raw.get("error") {
            return Err(TranslateError::ApiError(
                error
                    .get("message")
                    .map_or_else(|| "".to_string(), |v| v.to_string()),
            ));
        }
        let sentences = serde_json::from_value::<TranslationResponse>(raw)
            .map_err(TranslateError::SerdeError)?;
        let text = sentences
            .sentences
            .into_iter()
            .filter_map(|s| Some(s.trans))
            .collect::<Vec<String>>()
            .join("");
        Ok(text)
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
