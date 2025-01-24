# Example of propagating FIPS through dependency tree

## Building app on Ubuntu

> aws-lc-rs doesn't have bindings for windows, so it's better to use linux distibutive instead

```bash
sudo apt update && sudo apt install -y cmake golang-go
git clone https://github.com/Vaiz/rust-fips-examples.git
cd rust-fips-examples/
cargo run --manifest-path ./crypto_app/Cargo.toml
cargo run --manifest-path ./crypto_app_with_fips/Cargo.toml
```

Expected output:
crypto_app: `Error enabling FIPS mode: FIPS mode not enabled!`
crypto_app_with_fips: `FIPS mode enabled`

## cargo tree

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
