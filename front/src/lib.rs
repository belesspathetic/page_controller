use web_sys::window;
use web_sys::wasm_bindgen::UnwrapThrowExt;
pub mod components;
pub mod pages;


pub fn get_keys() -> Vec<String> {
    let window = window().unwrap_throw();
    let local_storage = window.local_storage().unwrap_throw().unwrap_throw();

    let string = local_storage.get_item("keys").unwrap_throw().unwrap_or_default();

    let keys: Vec<String> = serde_json::from_str(&string).unwrap_or_else(|_| Vec::new());

    keys
}