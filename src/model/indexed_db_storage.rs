
use indexed_db_futures::prelude::*;
use std::{sync::atomic::{AtomicUsize, Ordering}, future::IntoFuture};
use js_sys::wasm_bindgen::JsValue;
use serde_wasm_bindgen::{to_value, from_value};
use web_sys::IdbTransactionMode::Readwrite;

use crate::model::error::WasmResult;

use super::radical::Radical;

static IDB_INSTANCE_COUNT: AtomicUsize = AtomicUsize::new(0);
const DB_NAME: &str = "zhongchar";
const DB_VERSION: u32 = 1;
const DB_RADICALS_STORE: &str = "radicals";
const DB_RADICAL_FORMS_DATA_STORE: &str = "radical_forms_data";

#[derive(Debug)]
pub struct IndexedDBStorage {
    db: IdbDatabase,
    instance_number: usize,
}

impl IndexedDBStorage {
    pub async fn new() -> WasmResult<Self> {
        let mut db_req: OpenDbRequest = IdbDatabase::open_u32(DB_NAME, DB_VERSION)?;
        db_req.set_on_upgrade_needed(Some(|evt: &IdbVersionChangeEvent| -> Result<(), JsValue> {
            // if evt.db().name() != DB_RADICALS_STORE {
            //     return Ok(())
            // }
            // TODO: deal with upgrades
            let radical_object_store_params = IdbObjectStoreParameters::new()
                    .key_path(Some(&IdbKeyPath::new(JsValue::from_str("number"))))
                    .to_owned();
            evt.db().create_object_store_with_params(DB_RADICALS_STORE, &radical_object_store_params)?;

            let radical_form_stats_store_params = IdbObjectStoreParameters::new()
                    .key_path(Some(&IdbKeyPath::new(JsValue::from_str("radical_form"))))
                    .to_owned();
            evt.db().create_object_store_with_params(DB_RADICAL_FORMS_DATA_STORE, &radical_form_stats_store_params)?;
            Ok(())
        }));

        let db: IdbDatabase = db_req.into_future().await?;
        let instance_number = IDB_INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst);
        Ok(IndexedDBStorage { db, instance_number })
    }

    pub fn get_database(&self) -> *const IdbDatabase {
        &self.db
    }

    pub async fn store_radical(&self, radical: &Radical) -> WasmResult<()> {
        let tx = self
            .db
            .transaction_on_one_with_mode(DB_RADICALS_STORE, Readwrite)?;
        let store = tx.object_store(DB_RADICALS_STORE)?;
        let _ = store.put_val(&to_value(radical)?)?;
        Ok(tx.await.into_result()?)
    }

    pub async fn store_radicals(&self, radicals: Vec<Radical>) -> WasmResult<()> {
        let tx = self.db.transaction_on_one_with_mode(DB_RADICALS_STORE, Readwrite)?;
        let store = tx.object_store(DB_RADICALS_STORE)?;
        for radical in radicals.iter().rev() {
            let _ = store.put_val(&to_value(radical)?)?;
        }
        Ok(tx.await.into_result()?)
    }

    pub async fn get_radicals(&self) -> WasmResult<Vec<Radical>> {
        let tx = self
                    .db
                    .transaction_on_one(DB_RADICALS_STORE)?;
        let store = tx.object_store(DB_RADICALS_STORE)?;
        let raw_values = store.get_all()?.await?;
        let parsed_radicals: Result<Vec<Radical>, _> = raw_values
            .into_iter()
            .map(from_value)
            .collect();

        Ok(parsed_radicals?)
    }

    pub async fn store_radical_form_data(&self) -> WasmResult<()> {
        todo!()
    }
}

impl PartialEq for IndexedDBStorage {
    fn eq(&self, other: &Self) -> bool {
        self.instance_number == other.instance_number
    }
}