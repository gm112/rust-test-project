use wasm_bindgen_test::*;
#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(add(1, 2).unwrap(), 3);
}
