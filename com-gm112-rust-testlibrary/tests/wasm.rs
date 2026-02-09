use com_gm112_rust_testlibrary::add;
use wasm_bindgen_test::*;
#[wasm_bindgen_test]
fn one_plus_two_equals_three() {
    assert_eq!(add(1, 2).unwrap(), 3);
}
