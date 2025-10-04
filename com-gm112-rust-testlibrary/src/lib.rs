#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use wee_alloc::WeeAlloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[cfg(feature = "console_error_panic_hook")]
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Adds two numbers.
///
/// @example <caption>Add two numbers in TypeScript</caption>
/// ```typescript
/// // In TypeScript:
/// import { add } from 'com-gm112-rust-testlibrary';
/// console.log(add(1, 2)); // 3
/// ```
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> Result<i32, JsValue> {
    internal_add(a, b).map_err(|e| JsValue::from(e))
}

/// Adds two numbers.
///
/// @example <caption>Add two numbers in Rust</caption>
/// ```rust
/// // In Rust:
/// use com_gm112_rust_testlibrary::add;
/// assert_eq!(add(1, 2).unwrap(), 3);
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn add(a: i32, b: i32) -> Result<i32, i32> {
    internal_add(a, b)
}

fn internal_add(a: i32, b: i32) -> Result<i32, i32> {
    Ok(a + b)
}
