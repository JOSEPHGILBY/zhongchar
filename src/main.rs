use leptos::{*, html::audio, error::Result};
use leptos_use::docs::BooleanDisplay;
use leptos_use::{use_service_worker_with_options, UseServiceWorkerReturn, UseServiceWorkerOptions};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum RadicalError {
    #[error("there was a radical error")]
    ThereIsAnError,
}

#[component]
fn ServiceWorkerInfo() -> impl IntoView {
    let UseServiceWorkerReturn {
        registration,
        installing,
        waiting,
        active,
        skip_waiting,
        ..
    } = use_service_worker_with_options(
        UseServiceWorkerOptions::default()
        .script_url("service_worker.js".to_owned())
    );

    view! {

        <br/>

        <p>"registration: " {move || format!("{:#?}", registration())}</p>
        <p>"installing: " <BooleanDisplay value=installing/></p>
        <p>"waiting: " <BooleanDisplay value=waiting/></p>
        <p>"active: " <BooleanDisplay value=active/></p>

        <br/>

        <button on:click=move |_| { skip_waiting() }>"Send skip_waiting event"</button>
    }
}

#[component]
fn AnswerAttemptComponent() -> impl IntoView {
    let (answer_attempt, set_answer_attempt) = create_signal("".to_string());
    let (res, set_res) = create_signal("".to_string());
    view! {
        <button on:click=move |_| {
            let audio = audio();
            audio.set_src("/public/tmp03bqpcha.wav");
            set_res(audio.play().err().and_then(|v| v.as_string()).unwrap_or("ddd333".to_string()));
        }>play sound{res}</button>
        <br/>
        <input type="text"
            on:input=move |ev| {
                set_answer_attempt(event_target_value(&ev));
            }
            prop:value=answer_attempt
        />
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <AnswerAttemptComponent/>
        <RadicalTable/>
        <ServiceWorkerInfo/>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Radical {
    #[serde(rename = "#")]
    pub number: i32,
    #[serde(rename = "Radical forms")]
    radical_forms: String,
    #[serde(rename = "Stroke count")]
    stroke_count: i32,
    #[serde(rename = "Meaning")]
    meaning: String,
    #[serde(rename = "Colloquial Term")]
    colloquial_term: Option<String>,
    #[serde(rename = "Pīnyīn")]
    pinyin: String,
    #[serde(rename = "Hán-Việt")]
    han_viet: String,
    #[serde(rename = "Hiragana-Romaji")]
    hiragana_romaji: String,
    #[serde(rename = "Hangul-Romaja")]
    hangul_romaja: String,
    #[serde(rename = "Frequency")]
    frequency: i32,
    #[serde(rename = "Simplified")]
    simplified: Option<String>,
    #[serde(rename = "Examples")]
    examples: String,
}

#[component]
fn RadicalTable() -> impl IntoView {
    
    let radicals = create_local_resource(move || (), |_| fetch_radicals());
    view! {
        <div>
            <div>
                {   move ||
                    match radicals.get() {
                        Some(Ok(radicals)) => radicals[0].number.to_string(),
                        Some(Err(err)) => format!("Error: {}", err),
                        None => "Loading...".to_owned(),
                    }
                }
            </div>
            <button on:click=move |_| radicals.refetch()>
                    "Refetch"
            </button>
        </div>
    }
}
    
async fn fetch_radicals() -> Result<Vec<Radical>> {
    let text = 
        reqwasm::http::Request::get(&format!(
        "http://127.0.0.1:8080/public/radicals.csv"
        ))
            .send()
            .await?
            .text()
            .await?;

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(text.as_bytes());
    let mut radicals = Vec::new();
    for result in reader.deserialize() {
        let radical: Radical = result?;
        radicals.push(radical);
    }
    
    Ok(radicals)
}

fn main() {
    mount_to_body(|| view! { <App/> })
}

