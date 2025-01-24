# Example of Propagating FIPS through Dependency Tree

## Overview

This example demonstrates how to propagate FIPS mode through a dependency tree in Rust. The key idea
is to enable the FIPS feature in the `Cargo.toml` of the final binary, even if it doesn't have a
direct dependency on the FIPS library.

## Building the App on Ubuntu

> Note: `aws-lc-rs` doesn't have bindings for Windows, so it's recommended to use a Linux distribution instead.

```bash
sudo apt update && sudo apt install -y cmake golang-go
git clone https://github.com/Vaiz/rust-fips-examples.git
cd rust-fips-examples
cargo run --manifest-path ./crypto_app/Cargo.toml
cargo run --manifest-path ./crypto_app_with_fips/Cargo.toml
```

Expected output:

- `crypto_app`: `Error enabling FIPS mode: FIPS mode not enabled!`

- `crypto_app_with_fips`: `FIPS mode enabled`

## Enabling FIPS Mode

To enable FIPS mode, you need to add the FIPS feature to the `Cargo.toml` of the final binary. This
example includes two binaries: `crypto_app` and `crypto_app_with_fips`. The latter enables FIPS mode
by specifying the FIPS feature in its `Cargo.toml`.

### `crypto_app/Cargo.toml`

```toml
...[dependencies]
some_crypto_lib = { path = "../some_crypto_lib" }
```

### `crypto_app_with_fips/Cargo.toml`

```toml
...
[dependencies]
some_crypto_lib = { path = "../some_crypto_lib" }
aws-lc-rs = { version = "1.12.2", features = ["fips"] }
```

## Cargo Tree

Using `cargo tree` command, it's possible to see that the `fips` feature adds the `aws-lc-fips-sys`
crate into the dependencies.

The following command shows the dependency tree for `crypto_app`:

```bash
cargo tree --manifest-path ./crypto_app/Cargo.toml
crypto_app v0.1.0 (rust-fips-examples/crypto_app)
└── some_crypto_lib v0.1.0 (rust-fips-examples/some_crypto_lib)
    └── aws-lc-rs v1.12.2
        ├── aws-lc-sys v0.25.0
        │   └── paste v1.0.15 (proc-macro)
        │   [build-dependencies]
        │   ├── cc v1.2.10
        │   │   ├── jobserver v0.1.32
        │   │   │   └── libc v0.2.169
        │   │   ├── libc v0.2.169
        │   │   └── shlex v1.3.0
        │   ├── cmake v0.1.52
        │   │   └── cc v1.2.10 (*)
        │   ├── dunce v1.0.5
        │   └── fs_extra v1.3.0
        ├── paste v1.0.15 (proc-macro)
        ├── untrusted v0.7.1
        └── zeroize v1.8.1
```

The following command shows the dependency tree for `crypto_app_with_fips`:

```bash
cargo tree --manifest-path ./crypto_app_with_fips/Cargo.toml
crypto_app_with_fips v0.1.0 (rust-fips-examples/crypto_app_with_fips)
├── aws-lc-rs v1.12.2
│   ├── aws-lc-fips-sys v0.13.2
│   │   └── paste v1.0.15 (proc-macro)
│   │   [build-dependencies]
│   │   ├── cc v1.2.10
│   │   │   ├── jobserver v0.1.32
│   │   │   │   └── libc v0.2.169
│   │   │   ├── libc v0.2.169
│   │   │   └── shlex v1.3.0
│   │   ├── cmake v0.1.52
│   │   │   └── cc v1.2.10 (*)
│   │   ├── dunce v1.0.5
│   │   ├── fs_extra v1.3.0
│   │   └── regex v1.11.1
│   │       ├── aho-corasick v1.1.3
│   │       │   └── memchr v2.7.4
│   │       ├── memchr v2.7.4
│   │       ├── regex-automata v0.4.9
│   │       │   ├── aho-corasick v1.1.3 (*)
│   │       │   ├── memchr v2.7.4
│   │       │   └── regex-syntax v0.8.5
│   │       └── regex-syntax v0.8.5
│   ├── aws-lc-sys v0.25.0
│   │   └── paste v1.0.15 (proc-macro)
│   │   [build-dependencies]
│   │   ├── cc v1.2.10 (*)
│   │   ├── cmake v0.1.52 (*)
│   │   ├── dunce v1.0.5
│   │   └── fs_extra v1.3.0
│   ├── paste v1.0.15 (proc-macro)
│   ├── untrusted v0.7.1
│   └── zeroize v1.8.1
└── some_crypto_lib v0.1.0 (rust-fips-examples/some_crypto_lib)
    └── aws-lc-rs v1.12.2 (*)
```
