use leptos::{*};

use crate::model::error::WasmResult;
use crate::model::radical::Radical;



#[component]
pub(crate) fn RadicalTable<F>(
    radicals: F
) -> impl IntoView 
where
    F: SignalWith<Value = Option<WasmResult<Vec<Radical>>>> + Clone + 'static,
{
    view! {
        <div>
            { move || radicals.with(move |radicals| match radicals {
                None => view! { "Loading..." }.into_view(),
                Some(Err(err)) => {

                    view! { {format!{"{}", err}} }.into_view()
                },
                Some(Ok(radicals)) => view! {
                    <table>
                        <thead>
                            <tr>
                                <th>"Number"</th>
                                <th>"Radical Forms"</th>
                                <th>"Stroke Count"</th>
                                <th>"Meaning"</th>
                                <th>"Colloquial Term"</th>
                                <th>"Pinyin"</th>
                                <th>"Han Viet"</th>
                                <th>"Hiragana Romaji"</th>
                                <th>"Hangul Romaja"</th>
                                <th>"Frequency"</th>
                                <th>"Simplified"</th>
                                <th>"Examples"</th>
                            </tr>
                        </thead>
                        <tbody> 
                        {radicals.clone().into_iter()
                            .map(|radical| {
                                let joined_string: String = radical.radical_forms.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(", ");

                                view! {
                                    <tr>
                                        <td>{ radical.number.to_string() }</td>
                                        <td>{ joined_string }</td>
                                        <td>{ radical.stroke_count.to_string() }</td>
                                        <td>{ radical.meaning }</td>
                                        <td>{ radical.colloquial_term.unwrap_or_else(|| "".to_string()) }</td>
                                        <td>{ radical.pinyin }</td>
                                        <td>{ radical.han_viet }</td>
                                        <td>{ radical.hiragana_romaji }</td>
                                        <td>{ radical.hangul_romaja }</td>
                                        <td>{ radical.frequency.to_string() }</td>
                                        <td>{ radical.simplified.unwrap_or_else(|| "".to_string()) }</td>
                                        <td>{ radical.examples }</td>
                                    </tr>
                                }
                            })
                            .collect_view()
                        }
                        </tbody>
                    </table>
                }.into_view(),
            })}
        </div>
    }
}