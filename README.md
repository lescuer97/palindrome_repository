# PALINDROME SEARCHER
Simple function for searching all palindrome numbers inside of an inputed range.
Runed for testing in Deno

## Details of compilation of rust.

Because in package Wasm-Pack Deno as a target is not yet available.
I used the wasm-bindgen CLI for outputting the modules.

WARNING!: wasm_bindgen and its CLI must be the same version or you will have a world of trouble.

if you leave the directories in order and as named, this are the command to compile and output the wasm module.

For testing the wasm module you don't need to compile the rust code.
just run in the base folder: 
```
$ cd ..
$ deno run --allow-read index.ts
```

Installing the wasm_bindgen CLI 

```
$ cargo install wasm-bindgen-cli --version 0.2.70 
```

Go to palindrom_wasm
```
$ cd palindrom_wasm/
```

Compile rust and output module (inside palindrom_wasm folder)
```
$ cargo build --release --target wasm32-unknown-unknown
$ wasm-bindgen --target deno --out-dir ./deno_wasm ./target/wasm32-unknown-unknown/release/palindrom_wasm.wasm
```
Running the module in deno 
```
$ cd ..
$ deno run --allow-read index.ts
```
if you want to run the Typescript module just comment the import of WASM and uncomment the Typescript import
