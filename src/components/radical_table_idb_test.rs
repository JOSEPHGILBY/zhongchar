use std::sync::Arc;

use leptos::{*};

use crate::components::radical_table::RadicalTable;
use crate::model::{indexed_db_storage::IndexedDBStorage, error::{WasmError, WasmResult}, radical::Radical};

#[derive(Debug, Clone)]
pub struct IntermediateStruct {
    idb: Option<Arc<IndexedDBStorage>>,
    action_result: Option<bool>,
}

impl IntermediateStruct {
    fn new(idb: &Option<Result<Arc<IndexedDBStorage>, WasmError>>, action_result: &Option<Result<(), WasmError>>) -> Self {
        IntermediateStruct {
            idb: idb.as_ref().and_then(|res| res.as_ref().ok()).cloned(),
            action_result: action_result.as_ref().map(|res| res.is_ok()),
        }
    }
}

impl PartialEq for IntermediateStruct {
    fn eq(&self, other: &Self) -> bool {
        self.idb == other.idb && self.action_result == other.action_result
    }
}

#[component]
pub(crate) fn RadicalTableIDBTest() -> impl IntoView {
    let idb = expect_context::<Resource<(), WasmResult<Arc<IndexedDBStorage>>>>();
    let radicals = create_local_resource(move || (), |_| Radical::fetch_radicals());
    let store_radicals_action = create_action(|(idb, radicals): &(Arc<IndexedDBStorage>, Vec<Radical>)| {
        let radicals_clone = radicals.to_owned();
        let idb_ref_inc = idb.clone();
        async move { idb_ref_inc.store_radicals(radicals_clone).await }
    });
    let store_radicals_action_result = store_radicals_action.value();
    let intermediate =  move || with!(|idb, store_radicals_action_result| {
        IntermediateStruct::new(idb, store_radicals_action_result)
    });
    let radicals_from_idb = create_local_resource(intermediate, |intermediate_struct| {
        async move { match (intermediate_struct.idb, intermediate_struct.action_result) {
            (Some(idb), Some(true)) => idb.get_radicals().await,
            _ => Err(WasmError::Anyhow(anyhow::anyhow!("Failed to get radicals from IndexedDB")))
        }}
    });
    let store_to_idb_button =  move || with!(|idb, radicals| match (idb, radicals) {
        (None, _) => view!{}.into_view(),
        (_, None) => view!{}.into_view(),
        (Some(Err(err)), Some(Err(err2))) => view! { {format!{"{}{}", err, err2}} }.into_view(),
        (Some(_), Some(Err(err))) => view! { {format!{"{}", err}} }.into_view(),
        (Some(Err(err)), Some(_)) => view! { {format!{"{}", err}} }.into_view(),
        (Some(Ok(idb)), Some(Ok(radicals))) => {
            let idb_clone = idb.clone();
            let radicals = radicals.clone();
            view!{
                <button on:click=move |_| store_radicals_action.dispatch((idb_clone.clone(), radicals.clone()))>
                    "Store to indexed_db"
                </button>
            }.into_view()
        }
    });
    
    view! {
        <div>
            <RadicalTable radicals=radicals/>
            {store_to_idb_button}
            <button on:click=move |_| radicals.refetch()>
                "Refetch from CSV"
            </button>
            <RadicalTable radicals=radicals_from_idb/>
        </div>
    }
}