mod cdp;

use std::process::Command;
use cdp::models;
use cdp::fetch_metadata;
use crate::models::{Metadata, PackageInfo};

fn main() {
    let dataset = fetch_package_info();
    println!("{}", serde_json::to_string(&dataset).expect("Failed to serialize object"));
}

fn fetch_package_info() -> Vec<Metadata> {
    let current_installation_metadata = Command::new("cargo")
        .args(&[
            "metadata",
            "--format-version=1",
            "--no-deps",
            "--locked", // Use the locked version specified in Cargo.lock
        ])
        .output()
        .expect("Failed to execute 'cargo' command");

    let stdout = String::from_utf8_lossy(&current_installation_metadata.stdout);
    let cargo_metadata: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    let full_installation_metadata = Command::new("cargo")
        .args(&[
            "metadata",
            "--format-version=1",
            "--locked", // Use the locked version specified in Cargo.lock
        ])
        .output()
        .expect("Failed to execute 'cargo' command");

    let full_installation_stdout = String::from_utf8_lossy(&full_installation_metadata.stdout);
    let full_cargo_metadata: serde_json::Value = serde_json::from_str(&full_installation_stdout).unwrap();

    let mut collection: Vec<Metadata> = Vec::new();

    for package in cargo_metadata["packages"].as_array().unwrap_or(&vec![]) {
        let deps = package.get("dependencies").expect("There weren't any dependencies");
        for item in deps.as_array().unwrap_or(&vec![]) {
            let package_info = serde_json::from_value::<models::PackageInfo>(item.clone()).unwrap();
            let full_package_info = get_package_info_for_name(&full_cargo_metadata, &package_info.name.clone().unwrap().to_string());
            let mut metadata = fetch_metadata(&package_info.name.clone().unwrap().to_string(), &full_package_info.version.unwrap());
            metadata.description = full_package_info.description.unwrap_or_else(|| "".to_string());
            metadata.authors = full_package_info.authors.unwrap_or_else(Vec::new)
                .iter()
                .map(|author| author.clone())
                .collect::<Vec<_>>()
                .join(", ");
            collection.push(metadata);
        }
    }

    collection
}

fn get_package_info_for_name(full_metadata: &serde_json::Value, name: &String) -> PackageInfo {
    full_metadata["packages"]
        .as_array()
        .and_then(|packages| {
            packages.iter().find(|package| {
                package["name"].as_str().unwrap_or("") == name
            })
        })
        .map(|package| {
            serde_json::from_value::<PackageInfo>(package.clone()).unwrap()
        })
        .unwrap()
}
