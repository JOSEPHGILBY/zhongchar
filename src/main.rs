mod helpers;
mod components;
mod model;
mod pages;

use std::sync::Arc;

use leptos::{*, html::audio};
use leptos_use::docs::BooleanDisplay;
use leptos_use::{use_service_worker_with_options, UseServiceWorkerReturn, UseServiceWorkerOptions};
use leptos_router::*;

use helpers::prepend_relative_url;

use crate::components::radical_table_idb_test::RadicalTableIDBTest;
use crate::model::error::WasmResult;
use crate::model::indexed_db_storage::IndexedDBStorage;
use crate::pages::home::Home;

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
            audio.set_src(&prepend_relative_url("public/tmp03bqpcha.wav"));
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
    let idb: Resource<(), WasmResult<Arc<IndexedDBStorage>>> = create_local_resource(move || (), |_| async {
        let db = IndexedDBStorage::new().await?;
        Ok(Arc::new(db))
    });
    provide_context(idb);
    view! {
        <Router>
            <nav class="bg-white border-gray-200 dark:bg-gray-900">
                <div class="max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4">
                    <a href="https://flowbite.com/" class="flex items-center space-x-3 rtl:space-x-reverse">
                        <img src="https://flowbite.com/docs/images/logo.svg" class="h-8" alt="Flowbite Logo" />
                        <span class="self-center text-2xl font-semibold whitespace-nowrap dark:text-white">Flowbite</span>
                    </a>
                    <button data-collapse-toggle="navbar-default" type="button" class="inline-flex items-center p-2 w-10 h-10 justify-center text-sm text-gray-500 rounded-lg md:hidden hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600" aria-controls="navbar-default" aria-expanded="false">
                        <span class="sr-only">Open main menu</span>
                        <svg class="w-5 h-5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 17 14">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 1h15M1 7h15M1 13h15"/>
                        </svg>
                    </button>
                    <div class="hidden w-full md:block md:w-auto" id="navbar-default">
                        <ul class="font-medium flex flex-col p-4 md:p-0 mt-4 border border-gray-100 rounded-lg bg-gray-50 md:flex-row md:space-x-8 rtl:space-x-reverse md:mt-0 md:border-0 md:bg-white dark:bg-gray-800 md:dark:bg-gray-900 dark:border-gray-700">
                        <li>
                            <a href="/" class="block py-2 px-3 text-white bg-blue-700 rounded md:bg-transparent md:text-blue-700 md:p-0 dark:text-white md:dark:text-blue-500" aria-current="page">Home</a>
                        </li>
                        <li>
                            <a href="/quiz" class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">Quiz</a>
                        </li>
                        <li>
                            <a href="/data" class="block py-2 px-3 text-gray-900 rounded hover:bg-gray-100 md:hover:bg-transparent md:border-0 md:hover:text-blue-700 md:p-0 dark:text-white md:dark:hover:text-blue-500 dark:hover:bg-gray-700 dark:hover:text-white md:dark:hover:bg-transparent">Data</a>
                        </li>
                        </ul>
                    </div>
                </div>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/quiz" view=AnswerAttemptComponent/>
                </Routes>
            </main>
        </Router>
    }
}




fn main() {
    mount_to_body(|| view! { <App/> })
}

