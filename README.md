# Learn Rust

## Create a new Rust project (new folder)

```bash
cargo new hello_world --bin
```

## Create a new Rust project (in the current folder)

```bash
cargo init --bin
```

## Check your project

```bash
cargo check
```

## Run your project

```bash
cargo run
```

## Watch the compiler

```bash
cargo install cargo-watch

cargo watch -x run
```

## Build your project

```bash
cargo build
```

## Build your project with release mode

produces a faster, optimized binary

```bash
cargo build --release
```

---

Note: if we start with file number and create new project. Cargo will complain unless we use --name ` cargo new 02-data-types --name data-types`
