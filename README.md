# Cosmwasm template

### Setup


1/ Install rustup to get default toolchain

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

See instructions here if you're on Windows
https://www.rust-lang.org/tools/install

This will install cargo, rustc .etc


2/ Install wasm target

```
rustup target add wasm32-unknown-unknown
```

3/ Clone the project

```
git clone https://github.com/vinhphuctadang/hello-wasm.git
```

### Build the contract

```
cd path/to/hello-wasm
cargo wasm
```

The .wasm built file is in `target/wasm32-unknown-unknown/release/hello_wasm.wasm`

### Generate schema

```
cd path/to/hello-wasm
cargo run
```

Schema will be generated in `./schema`