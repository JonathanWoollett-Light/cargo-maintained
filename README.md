# cargo-maintained

[![Crates.io](https://img.shields.io/crates/v/cargo-maintained)](https://crates.io/crates/cargo-maintained)

A tool to check crates are up to date.

More specifically, checks that all crates in the dependency tree use the latest version of all their dependencies.

## Installation

```text
cargo install cargo-maintained
```

## Usage

```text
Usage: cargo-maintained.exe [OPTIONS]

Options:
      --max-depth <MAX_DEPTH>  Maximum depth to explore the dependency tree. A depth of 0 means only the direct dependencies of the current crate [default: 1]
      --prerelease             Whether to include pre-release versions in the check
      --tree                   Whether to print the dependency tree
  -h, --help                   Print help
```

## Examples

Running it on this crate results in:

```text
Some of the 390 dependencies are not up to date.
There are 72 offending crates
The offending crates are {"toml_edit", "toml_write", "potential_utf", "tokio", "writeable", "icu_collections", "core-foundation", "base64", "native-tls", "tempfile", "security-framework", "unicode-xid", "schannel", "anstyle-query", "camino", "foreign-types", "cargo-util-schemas", "serde-value", "icu_locale_core", "chrono", "mio", "openssl-sys", "unicode-ident", "yoke", "fastrand", "wasi", "io-uring", "tracing", "bumpalo", "icu_normalizer", "anstream", "icu_provider", "smallvec", "indicatif", "zerovec-derive", "portable-atomic", "openssl", "httparse", "futures", "ordered-float", "reqwest", "serde-untagged", "socket2", "tinystr", "backtrace", "tokio-native-tls", "addr2line", "toml", "zerotrie", "rustls-pki-types", "web-time", "zerovec", "windows-sys", "litemap", "rustix", "hashbrown", "object", "tower", "cargo_metadata", "http", "hyper-util", "tower-http", "futures-util", "anstyle-wincon", "ansi_term", "anstyle-parse", "want", "winnow", "windows-targets", "tower-service", "ipnet", "displaydoc"}
```

It's quite slow mostly due to the bottleneck of the crates.io API.

Running it on [axum](https://github.com/tokio-rs/axum) results in:

```text
Some of the 753 dependencies are not up to date.
There are 131 offending crates
The offending crates are {"hickory-resolver", "sharded-slab", "base64", "eventsource-stream", "brotli", "async-lock", "core-foundation", "crypto-common", "idna_adapter", "tower", "itertools", "hashlink", "thiserror", "tokio-native-tls", "inout", "tinystr", "webpki-roots", "native-tls", "example-validator", "generic-array", "sqlx-core", "ctr", "quinn-proto", "icu_locid_transform", "rand", "hmac", "tokio", "tokio-tungstenite", "h2", "block-buffer", "toml", "typenum", "litemap", "aead", "aes-gcm", "byteorder", "ipnet", "hdrhistogram", "polyval", "postgres-types", "tokio-postgres", "which", "http-body", "async-session", "example-customize-extractor-error", "tungstenite", "zerovec-derive", "hkdf", "system-configuration", "encoding_rs", "cexpr", "android-tzdata", "signature", "sqlformat", "chrono", "tower-http", "askama_parser", "metrics-exporter-prometheus", "http", "rsa", "blake3", "bson", "combine", "resolv-conf", "rustls", "aws-lc-rs", "quickcheck_macros", "regex-automata", "metrics-util", "md-5", "futures-util", "icu_normalizer", "example-compression", "redis", "uuid", "tempfile", "diesel", "icu_provider", "hyper", "const-oid", "yoke", "sqlx-postgres", "typed-builder", "async-compression", "listenfd", "dotenvy", "zerovec", "sha1", "serde_bytes", "example-oauth", "serde_with", "winnow", "diesel-async", "bindgen", "hyper-rustls", "pbkdf2", "reqwest", "sha2", "hex", "phf_shared", "pq-sys", "sha-1", "reqwest-eventsource", "simple_asn1", "mongodb", "smallvec", "spki", "sqlx-mysql", "tokio-rustls", "parking_lot", "tower-service", "aes", "libloading", "clang-sys", "ghash", "displaydoc", "bitvec", "parking_lot_core", "digest", "toml_edit", "hickory-proto", "icu_properties", "oauth2", "axum-server", "der", "wasm-bindgen-futures", "matchit", "rustix", "pkcs8", "pkcs1", "crypto-mac"}
```
