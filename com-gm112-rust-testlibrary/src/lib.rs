use wasm_bindgen::prelude::*;
/// Adds two numbers.
///
/// @example <caption>Add two numbers in Rust</caption>
/// ```rust
/// // In Rust:
/// use com_gm112_rust_testlibrary::add;
/// assert_eq!(add(1, 2).unwrap(), 3);
/// ```
/// @example <caption>Add two numbers in TypeScript</caption>
/// ```typescript
/// // In TypeScript:
/// import { add } from 'com-gm112-rust-testlibrary';
/// console.log(add(1, 2)); // 3
/// ```
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> Result<i32, JsValue> {
    Ok(a + b)
}
