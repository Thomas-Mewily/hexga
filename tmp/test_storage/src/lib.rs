use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, IdbDatabase, IdbOpenDbRequest, console};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue>
{
    console::log_1(&"Hello from Rust WASM 👋".into());

    let window = window().unwrap();
    let idb_factory = window.indexed_db()?.unwrap();

    let request: IdbOpenDbRequest = idb_factory.open_with_u32("MyDB", 1)?;

    // Upgrade: create store on first open
    let on_upgrade = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let request = event
            .target()
            .unwrap()
            .dyn_into::<IdbOpenDbRequest>()
            .unwrap();
        let db = request.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
        let _ = db.create_object_store("users");
    }) as Box<dyn FnMut(_)>);
    request.set_onupgradeneeded(Some(on_upgrade.as_ref().unchecked_ref()));
    on_upgrade.forget();

    // Success: insert a record
    let on_success = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let request = event
            .target()
            .unwrap()
            .dyn_into::<IdbOpenDbRequest>()
            .unwrap();
        let db = request.result().unwrap().dyn_into::<IdbDatabase>().unwrap();

        // Transaction in readwrite mode
        let tx = db
            .transaction_with_str_and_mode("users", web_sys::IdbTransactionMode::Readwrite)
            .unwrap();
        let store = tx.object_store("users").unwrap();

        // Serialize Rust struct -> JsValue
        let user = User { id: 1, name: "Alice".into() };
        let js_val = serde_wasm_bindgen::to_value(&user).unwrap();

        store.put_with_key(&js_val, &JsValue::from_str("user_1")).unwrap();
    }) as Box<dyn FnMut(_)>);
    request.set_onsuccess(Some(on_success.as_ref().unchecked_ref()));
    on_success.forget();

    Ok(())
}


// fn main()
// {
//     run().unwrap();
// }


/*

wasm-pack build --target web

cargo watch -s "wasm-pack build --target web"


*/