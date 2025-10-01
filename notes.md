# Notes

## Setting up wasm in a rust project

Make sure you have wasm-pack: cargo install wasm-pack

Then in your lib.rs file.

```rust
// Required for wasm_bindgen
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen] //Add this to your external methods
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

```

Then, run

```
wasm-pack build --target web
```

