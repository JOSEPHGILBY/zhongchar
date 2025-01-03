use serde::{Deserialize, Serialize};
use leptos::{*};
use crate::WasmResult;
use crate::helpers::prepend_relative_url;

use crate::model::radical_from_csv;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Radical {
    pub number: i32,
    pub radical_forms: Vec<char>,
    pub stroke_count: i32,
    pub meaning: String,
    pub colloquial_term: Option<String>,
    pub pinyin: String,
    pub han_viet: String,
    pub hiragana_romaji: String,
    pub hangul_romaja: String,
    pub frequency: i32,
    pub simplified: Option<String>,
    pub examples: String,
}

impl Radical {
    pub async fn fetch_radicals() -> WasmResult<Vec<Radical>> {
        let url = format!(
            "{}//{}:{}{}",
            window().location().protocol().unwrap(),
            window().location().hostname().unwrap(),
            window().location().port().unwrap_or_default(),
            prepend_relative_url("public/radicals.csv")
        );
        let text = 
            reqwasm::http::Request::get(&url)
                .send()
                .await?
                .text()
                .await?;
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(text.as_bytes());
        let mut radicals: Vec<Radical> = Vec::new();
        let des = reader.deserialize();
        for result in des {
            let radical: radical_from_csv::Radical = result?;
            radicals.push(Radical::from(radical));
        }
        
        Ok(radicals)
    }
}

impl From<radical_from_csv::Radical> for Radical {
    fn from(value: radical_from_csv::Radical) -> Self {
        let radical_forms_vec: Vec<char> = value.radical_forms.chars()
            .filter(|c| c.is_alphabetic() && !c.is_ascii())
            .collect();
        Self {
            radical_forms: radical_forms_vec, 
            number: value.number,
            stroke_count: value.stroke_count,
            meaning: value.meaning,
            colloquial_term: value.colloquial_term,
            pinyin: value.pinyin,
            han_viet: value.han_viet,
            hiragana_romaji: value.hiragana_romaji,
            hangul_romaja: value.hangul_romaja,
            frequency: value.frequency,
            simplified: value.simplified,
            examples: value.examples,
        }
    }
}