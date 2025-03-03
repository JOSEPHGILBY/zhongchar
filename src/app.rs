use leptos::either::either;
use leptos::html::{self, Div, HtmlElement, Input};
use leptos::logging::log;
use leptos::{logging, prelude::*};
use leptos_meta::*;
use leptos_router::{components::*, path};
use leptos_use::core::IntoElementMaybeSignal;
use leptos_use::{sync_signal_with_options, use_color_mode_with_options, use_event_listener, use_event_listener_with_options, use_preferred_dark, use_window_focus, ColorMode, SyncSignalOptions, UseColorModeOptions, UseColorModeReturn};
use crate::model::error::ArcZhongCharError;
use crate::model::radical::Radical;
use leptos_router::hooks::use_location;
use leptos::ev::{blur, focus, MouseEvent};
use web_sys::PointerEvent;
use leptos::wasm_bindgen::JsCast;

#[component]
fn Nav() -> impl IntoView {
    let pathname = use_location().pathname;
    let UseColorModeReturn {
        mode,
        set_mode,
        ..
    } = use_color_mode_with_options(
        UseColorModeOptions::default()
            .emit_auto(true)
    );

    let color_mode_to_daisyui_theme = |color_mode: ColorMode| match color_mode {
        ColorMode::Auto => "default".to_string(),
        ColorMode::Light => "cupcake".to_string(),
        ColorMode::Dark => "night".to_string(),
        ColorMode::Custom(_) => "default".to_string()
    };

    let daisyui_theme_to_color_mode = |daisyui_theme: String| match daisyui_theme.as_str() {
        "cupcake" => ColorMode::Light,
        "night" => ColorMode::Dark,
        "default" | _ => ColorMode::Auto,
    };
    
    let selected_theme = RwSignal::new("default".to_string());
    let _ = sync_signal_with_options(
        (mode, set_mode), 
        selected_theme, 
        SyncSignalOptions::with_transforms(
            move |color_mode: &ColorMode| color_mode_to_daisyui_theme(color_mode.clone()), 
            move |daisyui_theme_name| daisyui_theme_to_color_mode(daisyui_theme_name.clone()),
        ),
    );
    let prefers_dark = use_preferred_dark();

    let theme_svg = move || {
        let binding = selected_theme.get();
        let current_theme_name = binding.as_str();
        let prefers_dark_got = prefers_dark.get();
        
        either!(current_theme_name,
            "default" => view! {
                {either!(prefers_dark_got,
                    true => view! {
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 0 1-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0 1 15 18.257V17.25m6-12V15a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 15V5.25m18 0A2.25 2.25 0 0 0 18.75 3H5.25A2.25 2.25 0 0 0 3 5.25m18 0V12a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 12V5.25" transform="scale(0.62) translate(-2,-2)"/>

                            <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" transform="scale(0.6) translate(17,18)"/>

                            <line x1="3" y1="21" x2="21" y2="3" style="stroke-width:1" stroke-linecap="round"/>
                        </svg>
                    },
                    false => view! { 
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 0 1-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0 1 15 18.257V17.25m6-12V15a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 15V5.25m18 0A2.25 2.25 0 0 0 18.75 3H5.25A2.25 2.25 0 0 0 3 5.25m18 0V12a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 12V5.25" transform="scale(0.62) translate(-2,-2)"/>

                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" transform="scale(0.6) translate(17,18)"/>

                            <line x1="3" y1="21" x2="21" y2="3" style="stroke-width:1" stroke-linecap="round"/>
                        </svg>
                    },
                )}
            },
            "cupcake" => view! { 
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" />
                </svg>
            },
            "night" => view! {
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" />
                </svg>
            },
            _ => view! {
                <span></span>
            },
        )
    };

    let default_radio_el = NodeRef::<Input>::new();
    let is_default_focused = use_element_focus(default_radio_el);
    let light_radio_el = NodeRef::<Input>::new();
    let is_light_focused = use_element_focus(light_radio_el);
    let dark_radio_el = NodeRef::<Input>::new();
    let is_dark_focused = use_element_focus(dark_radio_el);

    let clicked_theme_option_to_close_dropdown_on = RwSignal::new(None);
    let record_mouse_pointer_click = move |ev: MouseEvent| {
        let Ok(pointer) = ev.clone().dyn_into::<PointerEvent>() else { return };
        let Ok(clicked_label) = ev.target().unwrap().dyn_into::<web_sys::HtmlLabelElement>() else { return };
        if pointer.pointer_type() != "mouse" && pointer.pointer_type() != "touch" { return };
        let Some(theme_option) = clicked_label.dataset().get("themeOption") else {return};
        clicked_theme_option_to_close_dropdown_on.set(Some(theme_option));
    };

    let click_has_reached_radio = RwSignal::new(false);

    Effect::new(move |_| {
        if !click_has_reached_radio.get() { return };
        match clicked_theme_option_to_close_dropdown_on.get().as_deref() {
            Some("default") => {
                let _ = default_radio_el.get().expect("bleh").blur();
                clicked_theme_option_to_close_dropdown_on.set(None);
            }
            Some("light") => {
                let _ = light_radio_el.get().expect("bleh").blur();
                clicked_theme_option_to_close_dropdown_on.set(None);
            },
            Some("dark") => {
                let _ = dark_radio_el.get().expect("bleh").blur();
                clicked_theme_option_to_close_dropdown_on.set(None);
            },
            _ => {},
        };
        click_has_reached_radio.set(false);
    });

    view! {
        <div class="navbar bg-base-100 shadow-sm sticky top-0 z-50">
            <div class="navbar-start">
                <a href="/"
                    class="btn btn-ghost text-xl hover:bg-transparent"
                >
                    中 Char
                </a>
            </div>
            <div class="navbar-center">
                <a href="/radicals"
                    class="btn btn-ghost hover:bg-transparent"
                    class=("underline", move || pathname.get() == "/radicals".to_string())
                >
                    Radicals
                </a>
            </div>
            <div class="navbar-end">
                <div class="dropdown dropdown-end">
                    <div tabindex="0" role="button" class="btn btn-ghost hover:bg-transparent m-1">
                        {theme_svg}
                    </div>
                    <ul tabindex="0" class="dropdown-content bg-base-300 rounded-box z-1 p-2 shadow-2xl">
                        <li>
                            <label 
                                data-theme-option="default"
                                class="btn btn-sm btn-block btn-ghost justify-start"
                                class=(["border-2", "border-base-content"], move || selected_theme.get() == "default")
                                class=(["outline-2"], move || is_default_focused.get())
                                on:click=record_mouse_pointer_click
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 pointer-events-none">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 17.25v1.007a3 3 0 0 1-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0 1 15 18.257V17.25m6-12V15a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 15V5.25m18 0A2.25 2.25 0 0 0 18.75 3H5.25A2.25 2.25 0 0 0 3 5.25m18 0V12a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 12V5.25" />
                                </svg>
                                <input
                                    node_ref=default_radio_el
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller absolute opacity-0 w-0 h-0"
                                    aria-label="System"
                                    value="default"
                                    bind:radiogroup=selected_theme
                                    prop:checked=move || selected_theme.get() == "default"
                                    on:click=move |_| click_has_reached_radio.set(true)/>
                                System
                            </label>
                        </li>
                        <li>
                            <label
                                data-theme-option="light"
                                class="btn btn-sm btn-block btn-ghost justify-start"
                                class=(["border-2", "border-base-content"], move || selected_theme.get() == "cupcake")
                                class=(["outline-2"], move || is_light_focused.get())
                                on:click=record_mouse_pointer_click
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 pointer-events-none" >
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 3v2.25m6.364.386-1.591 1.591M21 12h-2.25m-.386 6.364-1.591-1.591M12 18.75V21m-4.773-4.227-1.591 1.591M5.25 12H3m4.227-4.773L5.636 5.636M15.75 12a3.75 3.75 0 1 1-7.5 0 3.75 3.75 0 0 1 7.5 0Z" />
                                </svg>
                                <input
                                    node_ref=light_radio_el
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller absolute opacity-0 w-0 h-0"
                                    aria-label="Light"
                                    value="cupcake"
                                    bind:radiogroup=selected_theme
                                    prop:checked=move || selected_theme.get() == "cupcake"
                                    on:click=move |_| click_has_reached_radio.set(true)/>
                                Light
                            </label>
                        </li>
                        <li>
                            <label 
                                data-theme-option="dark"
                                class="btn btn-sm btn-block btn-ghost justify-start"
                                class=(["border-2", "border-base-content"], move || selected_theme.get() == "night")
                                class=(["outline-2"], move || is_dark_focused.get())
                                on:click=record_mouse_pointer_click
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6 pointer-events-none">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z" />
                                </svg>
                                <input
                                    node_ref=dark_radio_el
                                    type="radio"
                                    name="theme-dropdown"
                                    class="theme-controller absolute opacity-0 w-0 h-0"
                                    aria-label="Dark"
                                    value="night"
                                    bind:radiogroup=selected_theme
                                    prop:checked=move || selected_theme.get() == "night"
                                    on:click=move |_| click_has_reached_radio.set(true)/>
                                Dark
                            </label> 
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}



#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let base_url = option_env!("BASE_URL").unwrap_or("");

    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router base=base_url>
            <Routes fallback=|| "Page not found.">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/radicals") view=Radicals/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <main>
            <Title text="中 Char"/>
            <Nav/>
            <div class="w-full h-dvh absolute top-0 flex flex-col">
                <div class="w-full h-16 flex-none"></div>
                hello world
            </div>
        </main>
    }
}

#[component]
fn Radicals() -> impl IntoView {
    let radicals_fetched = LocalResource::new(move || async move {
        Radical::fetch_radicals()
            .await
            .map_err(|e| ArcZhongCharError::from(e))
    });
    view! {
        <main>
            <Title text="中 Char"/>
            <Nav/>
            <div class="w-full h-dvh absolute top-0 flex flex-col">
                <div class="w-full h-16 flex-none"></div>
                <div class="w-full flex-grow overflow-x-auto overflow-y-auto">
                <table class="table table-zebra table-pin-rows w-full">
                    <thead>
                        <tr>
                            <th>"Number"</th>
                            <th>"Radical Forms"</th>
                            <th>"Stroke Count"</th>
                            <th>"Meaning"</th>
                            <th>"Colloquial Term"</th>
                            <th>"Pinyin"</th>
                            <th>"Han Viet"</th>
                            <th>"Hiragana/Romaji"</th>
                            <th>"Hangul/Romaja"</th>
                            <th>"Frequency"</th>
                            <th>"Simplified"</th>
                            <th>"Examples"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <Suspense fallback=move || {
                            let vec_20: Vec<i32> = (0..20).collect();
                            vec_20.into_iter().map(|n| {
                                if n % 2 == 0 {
                                    view!{<tr><td colspan="12">"\u{00A0}"</td></tr>}
                                } else {
                                    view!{<tr><td colspan="12" class="skeleton">"\u{00A0}"</td></tr>}
                                }
                            })
                            .collect::<Vec<_>>()
                        }>
                            <ErrorBoundary fallback=|errors| view! {
                                <div class="error">
                                    <p>"Not a number! Errors: "</p>
                                    <ul>
                                        {move || errors.get()
                                            .into_iter()
                                            .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                                            .collect::<Vec<_>>()
                                        }
                                    </ul>
                                </div>
                                }
                            >
                                { move || Suspend::new ( async move {
                                    radicals_fetched.await.map(|radicals| { // Changed this line
                                        view! {
                                            <> // Added Fragment here
                                                {
                                                    radicals.into_iter().map(|radical| {
                                                        view! {
                                                            <tr>
                                                                <td>{radical.number}</td>
                                                                <td>{radical.radical_forms}</td>
                                                                <td>{radical.stroke_count}</td>
                                                                <td>{radical.meaning}</td>
                                                                <td>{radical.colloquial_term}</td>
                                                                <td>{radical.pinyin}</td>
                                                                <td>{radical.han_viet}</td>
                                                                <td>{radical.hiragana_romaji}</td>
                                                                <td>{radical.hangul_romaja}</td>
                                                                <td>{radical.frequency}</td>
                                                                <td>{radical.simplified}</td>
                                                                <td>{radical.examples}</td>
                                                            </tr>
                                                        }
                                                    }).collect::<Vec<_>>()
                                                }
                                            </> // Close Fragment here
                                        }
                                    })
                                })}
                            </ErrorBoundary>
                        </Suspense>
                    </tbody>
                </table>
                </div>
            </div>
        </main>
    }

}


pub fn use_element_focus<El, M>(
    el: El,
) -> Signal<bool>
where
    El: IntoElementMaybeSignal<web_sys::EventTarget, M>
{
    let (is_focused, set_focused) = signal(false);
    let el = el.into_element_maybe_signal();
    let _ = use_event_listener(el, blur, move |_| set_focused(false));
    let _ = use_event_listener(el, focus, move |_| set_focused(true));

    is_focused.into()
}

