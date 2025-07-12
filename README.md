# cargo-maintained

[![Crates.io](https://img.shields.io/crates/v/cargo-maintained)](https://crates.io/crates/cargo-maintained)

A tool to check crates are up to date.

More specifically, it checks that all crates in the dependency tree use the latest versions of all their dependencies.

## How is this different to [cargo-outdated](https://crates.io/crates/cargo-outdated)?

`cargo-outdated` tells you when to run `cargo-update` to update `Cargo.lock`.

This tool tells you when to update `Cargo.toml`.

`cargo-outdated` tells you when version requirements support using a newer version of a crate. *`cargo update --check` if that existed.*

This tool tells you when version requirements are preventing using the latest versions of crates. *So you can post an issue on their repository to update their crate before it starts breaking your compilation process, and maybe a follow-up so they start using dependabot, and maybe a followup so they automate releases.*

## Why would I use this?

Becuase a large number of Rust crates only patch their latest release so if you (or any of your dependencies) stop using their latest release you are insecure.

Becuase a large number of Rust crates release breaking changes capable of breaking your compilation if the whole eco-system doesn't update (see issues with crate compilation breaking as a result of `rand` `2.*.*`).

## Installation

```text
cargo install cargo-maintained
```

## Usage

```text
Usage: cargo-maintained.exe [OPTIONS]

Options:
      --prerelease  Whether to include pre-release versions in the check
      --hidden      Whether to hide the progress bar
  -h, --help        Print help
```

## Examples

Running it on this crate results in:

```text
████████████████████ 199/199 [00:01:31 / 00:00:00] 2.1729/s                 
Some of the 199 dependencies are not up to date.
There are 64 offending crates.
The offending crates are {"toml_write", "bumpalo", "tokio-native-tls", "io-uring", "tempfile", "rustls-pki-types", "tokio", "fastrand", "want", "web-time", "icu_locale_core", "displaydoc", "http", "tower", "tracing", "tower-service", "backtrace", "anstyle-wincon", "schannel", "indicatif", "anstream", "potential_utf", "winnow", "cargo_metadata", "writeable", "hashbrown", "security-framework", "cargo-util-schemas", "rustix", "zerotrie", "reqwest", "openssl-sys", "socket2", "mio", "serde-value", "clap", "icu_provider", "zerovec", "litemap", "icu_collections", "ordered-float", "portable-atomic", "serde-untagged", "native-tls", "object", "chrono", "hyper-util", "anstyle-parse", "yoke", "wasi", "unicode-xid", "anstyle-query", "icu_normalizer", "unicode-ident", "tinystr", "camino", "openssl", "futures-util", "ipnet", "tower-http", "futures", "zerovec-derive", "httparse", "base64"}
```

It's quite slow mostly due to the bottleneck of the crates.io API.

Running it on [axum](https://github.com/tokio-rs/axum) results in:

```text
Some of the 575 dependencies are not up to date.
There are 200 offending crates.
The offending crates are {"rsa", "writeable", "webpki-roots", "blake3", "num-bigint-dig", "sqlformat", "byteorder", "simple_asn1", "icu_locid", "uuid", "android-tzdata", "ctr", "sha1_smol", "sqlx-mysql", "tower-service", "valuable", "quickcheck", "reqwest", "quanta", "rustix", "hashbrown", "sqlx-postgres", "spki", "signature", "nom", "metrics-util", "tempfile", "askama_derive", "native-tls", "fastrand", "wasi", "num-bigint", "h2", "tracing", "askama_escape", "httpdate", "iana-time-zone", "combine", "flume", "aws-lc-rs", "hyper-util", "chrono", "async-session", "openssl-sys", "unicode-bidi", "icu_locid_transform", "backtrace", "pkcs1", "zerovec-derive", "md-5", "eventsource-stream", "polyval", "rustc_version", "base64", "sha2", "httparse", "futures-util", "winapi-util", "unicode-ident", "postgres-types", "futures-intrusive", "widestring", "sqlx-sqlite", "rustls-pki-types", "metrics", "jsonwebtoken", "sha-1", "event-listener", "listenfd", "assert-json-diff", "crypto-mac", "bitvec", "tungstenite", "quinn-udp", "tokio-postgres", "ghash", "mio", "displaydoc", "hickory-proto", "sketches-ddsketch", "hmac", "subtle", "winnow", "clang-sys", "winreg", "aes", "aes-gcm", "http", "cookie", "rand", "atoi", "parking_lot", "dsl_auto_type", "serde_with_macros", "zstd-sys", "socket2", "openssl", "hickory-resolver", "tokio", "zerocopy-derive", "reqwest-eventsource", "tokio-tungstenite", "icu_provider", "zerocopy", "typed-json", "tinyvec", "raw-cpuid", "inout", "pem", "icu_collections", "crossbeam-queue", "generic-array", "bindgen", "pkcs8", "object", "aws-lc-sys", "web-time", "deadpool", "askama", "oauth2", "atomic-waker", "mongodb", "jobserver", "tokio-rustls", "crypto-common", "der", "regex-automata", "tower", "rustc-hash", "crc32fast", "bson", "yoke", "encoding_rs", "portable-atomic", "tower-http", "typenum", "etcetera", "pbkdf2", "litemap", "quinn", "axum-server", "matchit", "sqlx", "cexpr", "http-range-header", "constant_time_eq", "postgres-protocol", "spin", "sharded-slab", "security-framework", "sqlx-core", "multer", "block-buffer", "aho-corasick", "nu-ansi-term", "zerovec", "dotenvy", "home", "want", "mime_guess", "hex", "askama_parser", "time", "quinn-proto", "aead", "digest", "libloading", "thread_local", "glob", "parking_lot_core", "redis", "itertools", "convert_case", "ring", "sha1", "icu_normalizer", "hkdf", "tracing-subscriber", "arc-swap", "tinystr", "validator_derive", "bincode", "crossbeam-epoch", "futures-timer", "tracing-log", "schannel", "system-configuration", "ipnet", "which", "ahash", "regex", "hdrhistogram", "tokio-native-tls", "concurrent-queue", "mongodb-internal-macros", "crossbeam-utils", "hostname", "ipconfig", "lazy_static", "bumpalo"}
```
