# Cargo-Metadata-Resolver

## Description
This is an executable designed to get you the current package's metadata in json format. This was a "learn how to rust" project for me so don't expect a large amount of updates/improvements.

## Installation Instructions
```bash
cargo install cargo-metadata-resolver
```

## Usage
In the directory you want to get the metadata of run
```bash
cargo-metadata-resolver
```
For example when run in this project you'll see:
```
[{"name":"chrono","version":"0.4.26","authors":"","publish_date":"05-30-2023 14:38:13","description":"Date and time library for Rust"},{"name":"dict","version":"0.1.5","authors":"nacho <nacho@ownyourbits.com>","publish_date":"04-17-2018 11:00:57","
description":"Exercise crate implementing real associative arrays, also known as dictionaries"},{"name":"reqwest","version":"0.11.18","authors":"Sean McArthur <sean@seanmonstar.com>","publish_date":"05-16-2023 16:26:33","description":"higher level 
HTTP client library"},{"name":"serde","version":"1.0.171","authors":"Erick Tryzelaar <erick.tryzelaar@gmail.com>, David Tolnay <dtolnay@gmail.com>","publish_date":"07-09-2023 20:10:15","description":"A generic serialization/deserialization framewor
k"},{"name":"serde_json","version":"1.0.100","authors":"","publish_date":"","description":""},{"name":"toml","version":"0.7.6","authors":"Alex Crichton <alex@alexcrichton.com>","publish_date":"07-05-2023 15:50:42","description":"A native Rust encoder and decoder of TOML-formatted files and streams. Provides\nimplementations of the standard Serialize/Deserialize traits for TOML data to\nfacilitate deserializing and serializing Rust structures.\n"}]
```
