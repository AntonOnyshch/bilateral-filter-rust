[All additional information about Rust and WebAssembly you can find here](https://rustwasm.github.io/docs/wasm-pack/introduction.html)

# Description

## Why?
The very first I must say is that this is a small research in addition to my [first bilateral filter which was written in typescript](https://github.com/AntonOnyshch/bilateral-filter).
I wanted to increase speed of filter calculation and try Rust.
Rust I tried but speed doesn't shift at all. If you know Rust and how to increase the speed you'r welcome.

## SIMD
As far as I understood looking to my code there's no way to vectorize this one

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build --release --target web
```
## --target web - is needed here if you want to call your functions from browser.

### 🔬 Test

All tests is written inside lib.rs file.
Go to lib.rs file and press "Run all tests" below "#[cfg(test)]" attribute.

### Cargo.toml
Note that I add [lto = true](https://rustwasm.github.io/docs/book/game-of-life/code-size.html) line under [profile.release] and set opt-level to 3.

## Useful links

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
* `LICENSE-APACHE` and `LICENSE-MIT`: most Rust projects are licensed this way, so these are included for you