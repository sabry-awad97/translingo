use reqwest::{
    header::{HeaderMap, CONTENT_TYPE, USER_AGENT},
    Client,
};
use serde_json::Value;
use url::Url;

use crate::{
    config::TranslateOptions, error::TranslateError, model::TranslationResponse,
    translation_cache::TranslationCache, utils::extract_too_many_requests_info,
};

pub struct Translator {
    client: Client,
    pub input_text: String,
    options: TranslateOptions,
}

impl Translator {
    pub fn new(input_text: String, options: TranslateOptions) -> Self {
        Self {
            input_text,
            options: options,
            client: Client::new(),
        }
    }

    pub async fn translate(&self) -> Result<TranslationResponse, TranslateError> {
        let url = self.build_url()?;

        let body = format!("q={}", self.input_text);

        let res = self
            .client
            .post(url)
            .headers(self.build_headers())
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
            .map_err(|err| TranslateError::TextDecodeError(err.to_string()))?;

        let raw = serde_json::from_str::<Value>(&text).map_err(TranslateError::SerdeError)?;
        if let Some(error) = raw.get("error") {
            return Err(TranslateError::ApiError(
                error
                    .get("message")
                    .map_or_else(|| "".to_string(), |v| v.to_string()),
            ));
        }

        let result = serde_json::from_value::<TranslationResponse>(raw)
            .map_err(TranslateError::SerdeError)?;

        Ok(result)
    }

    fn build_url(&self) -> Result<Url, TranslateError> {
        let api_endpoint = format!("{}/translate_a/single", self.options.host);
        let url = Url::parse_with_params(&api_endpoint, self.options.to_query_params())?;
        Ok(url)
    }

    fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3".parse().unwrap());
        headers.insert(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded;charset=utf-8"
                .parse()
                .unwrap(),
        );
        headers
    }
}

pub async fn translate(
    input_text: &str,
    options: TranslateOptions,
) -> Result<TranslationResponse, TranslateError> {
    let mut translator = TranslationCache::new(input_text.to_owned(), options);
    let translation_result = match translator.translate().await {
        Ok(result) => result,
        Err(err) => return Err(TranslateError::ApiError(err.to_string())),
    };
    Ok(translation_result)
}
