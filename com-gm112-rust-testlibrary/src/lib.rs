use wasm_bindgen::prelude::*;

/// Adds two numbers.
/// 
/// ```
/// use com_gm112_rust_testlibrary::add;
/// assert_eq!(add(1, 2), 3);
/// ```
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}