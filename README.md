# cargo-maintained

Checks crates are up to date.

More specifically, checks that all crates in the dependency tree use the latest major version (excluding pre-release versions) of all their dependencies.

Running it on this crate with a depth of 3 results in:

```text
Some of the 873 dependencies are not up to date.
There are 110 offending crates
The offending crates are {"auto_enums", "serde_with", "core-foundation", "libloading", "tower-service", "tokio-rustls", "futures", "pcap", "quinn-proto", "smol", "der-parser", "system-configuration", "bincode", "thiserror", "sha2", "term", "tokio-executor", "tokio-io", "tempfile", "potential_utf", "rust_decimal", "hkdf", "base64", "heapless", "hmac", "hickory-resolver", "x509-parser", "android-tzdata", "encoding_rs", "tokio-native-tls", "zerovec", "rusty-fork", "faster-hex", "async-std", "litemap", "h2", "rand_pcg", "native-tls", "typenum", "hex", "jsonschema", "sysctl", "uuid", "smallstring", "cexpr", "itertools", "hdrhistogram", "bigdecimal", "byteorder", "metrohash", "zerovec-derive", "asn1-rs", "garde", "rustix", "bson", "moka", "yoke", "serde_cbor", "displaydoc", "rustls-native-certs", "parking_lot_core", "jiff", "tokio-sync", "rand", "x86", "camino", "seahash", "test-cert-gen", "tester", "trycmd", "clang-sys", "defmt", "detone", "ipnet", "cargo_metadata", "futures-time", "icu_provider", "debugger_test", "futures-util", "fxhash", "parking_lot", "hyper", "diesel", "aes-gcm", "hickory-proto", "rustls-platform-verifier", "anstream", "reqwest", "http-body", "ipnetwork", "tinystr", "arraystring", "borsh", "zerotrie", "http", "winnow", "adler32", "hyper-tls", "tempdir", "tokio-socks", "pnet_datalink", "rand_xorshift", "petgraph", "smallvec", "rstest_macros", "lending-stream", "chrono", "signal-hook", "fst", "bindgen"}
```

It's quite slow mostly due to the bottleneck of the crates.io API.

I would recommend running it with a depth of 1 or 0.

Running with a depth of 1 results in:

```text
Some of the 322 dependencies are not up to date.
There are 32 offending crates
The offending crates are {"potential_utf", "zerovec", "tokio-native-tls", "tinystr", "tower-service", "h2", "native-tls", "icu_provider", "reqwest", "displaydoc", "yoke", "zerovec-derive", "http-body", "cargo_metadata", "ipnet", "http", "core-foundation", "chrono", "rustix", "hyper-tls", "futures-util", "base64", "zerotrie", "hyper", "thiserror", "tempfile", "camino", "system-configuration", "encoding_rs", "futures", "smallvec", "litemap"}
```
