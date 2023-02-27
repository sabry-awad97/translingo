use std::num::NonZeroUsize;

use lru::LruCache;

use crate::{
    config::{TranslateOptions, CACHE_CAPACITY},
    error::TranslateError,
    model::TranslationResponse,
    translator::Translator,
};

pub struct TranslationCache {
    translator: Translator,
    cache: LruCache<String, TranslationResponse>,
}

impl TranslationCache {
    pub fn new(input_text: String, options: TranslateOptions) -> TranslationCache {
        TranslationCache {
            translator: Translator::new(input_text, options),
            cache: LruCache::new(NonZeroUsize::new(CACHE_CAPACITY).unwrap()),
        }
    }

    pub async fn translate(&mut self) -> Result<TranslationResponse, TranslateError> {
        // check if the result is already in the cache
        if let Some(result) = self.cache.get(&self.translator.input_text) {
            return Ok(result.clone());
        }

        // make the API call
        let result = self.translator.translate().await?;

        // add the result to the cache
        self.cache
            .put(self.translator.input_text.clone(), result.clone());

        Ok(result)
    }
}
