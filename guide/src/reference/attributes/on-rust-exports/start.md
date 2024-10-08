# `start`

When attached to a function this attribute will configure the `start`
section of the Wasm executable to be emitted, executing the tagged function as
soon as the Wasm module is instantiated.

```rust
#[wasm_bindgen(start)]
fn start() {
    // executed automatically ...
}
```

The `start` section of the Wasm executable will be configured to execute the
`start` function here as soon as it can. Note that due to various practical
limitations today the start section of the executable may not literally point to
`start`, but the `start` function here should be started up automatically when the
wasm module is loaded.

There's a few caveats to be aware of when using the `start` attribute:

* The `start` function must take no arguments and must either return `()` or
  `Result<(), JsValue>`
* Only one `start` function can be placed into a module, including its
  dependencies. If more than one is specified then `wasm-bindgen` will fail when
  the CLI is run. It's recommended that only applications use this attribute.
* The `start` function will not be executed when testing.
* Note that the `start` function is relatively new, so if you find any bugs with
  it, please feel free to report an issue!
