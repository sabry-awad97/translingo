use serde::{Deserialize, Serialize};


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
    pub trans: Option<String>,
    pub orig: Option<String>,
}
