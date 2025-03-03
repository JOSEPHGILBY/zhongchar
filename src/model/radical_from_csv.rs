use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Radical {
    #[serde(rename = "#")]
    pub number: i32,
    #[serde(rename = "Radical forms")]
    pub radical_forms: String,
    #[serde(rename = "Stroke count")]
    pub stroke_count: i32,
    #[serde(rename = "Meaning")]
    pub meaning: String,
    #[serde(rename = "Colloquial Term")]
    pub colloquial_term: Option<String>,
    #[serde(rename = "Pīnyīn")]
    pub pinyin: String,
    #[serde(rename = "Hán-Việt")]
    pub han_viet: String,
    #[serde(rename = "Hiragana-Romaji")]
    pub hiragana_romaji: String,
    #[serde(rename = "Hangul-Romaja")]
    pub hangul_romaja: String,
    #[serde(rename = "Frequency")]
    pub frequency: i32,
    #[serde(rename = "Simplified")]
    pub simplified: Option<String>,
    #[serde(rename = "Examples")]
    pub examples: String,
}