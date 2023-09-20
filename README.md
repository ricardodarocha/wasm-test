# wasm-test

This is a Wasm test with Rust and Svelte following the tutorial https://blog.logrocket.com/integrating-svelte-app-rust-webassembly/

## How it was made

At an empty folder, made a new lib project

```rust
cargo new --lib wasm-test
cd wasm-test
cargo add wasm-bindgen
cargo check
code .
```

```js
npx degit sveltejs/template www
cd www
npm install @wasm-tool/rollup-plugin-rust
npm i
```

```rust
>wasm-test> cargo build
```

```js
>wasm-test/www> npm run dev
```

## Checkout styled branch

This is the styled branch with Carbon

https://carbon-components-svelte.onrender.com/

```git
git checkout styled
```



## The branch called full

This is the branch with full features described at the article https://blog.logrocket.com/integrating-svelte-app-rust-webassembly/

This next section will be slightly more advanced and complex. We will define a struct in Rust and use it from JavaScript to pass values to a function. We will also see how we can expose impl methods and call them from JavaScript.

```git
git checkout full
```




