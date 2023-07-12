mod fetch_metadata;

use std::collections::BTreeMap;
use std::fs;
use std::process::Command;
use serde::Deserialize;
use crate::fetch_metadata::Metadata;

#[derive(Deserialize, Debug)]
struct CargoFile {
    dependencies: BTreeMap<String, DepValue>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DepValue {
    String(String),
    DepShape(DepShape),
}

#[derive(Debug, Deserialize)]
struct DepShape {
    version: String,
}

#[derive(Deserialize, Debug)]
struct PackageInfo {
    description: Option<String>,
    authors: Option<Vec<String>>,
}

fn main() {
    let contents = fs::read_to_string("./Cargo.toml")
        .expect("Should have been able to read the file");

    let cargo_file: CargoFile = toml::from_str(&contents).unwrap();
    let mut dataset: Vec<Metadata> = vec![];

    for (name, dep_value) in cargo_file.dependencies {
        match dep_value {
            DepValue::String(version) => {
                // println!("Package: {} Version: {}", name, version);
                dataset.push(fetch_package_info(&name, &version));
            }
            DepValue::DepShape(dep_shape) => {
                // println!("Package: {} Version: {}", name, dep_shape.version);
                dataset.push(fetch_package_info(&name, &dep_shape.version));
            }
        }
    }
    println!("{}", serde_json::to_string(&dataset).expect("Failed to serialize object"));
}

fn fetch_package_info(package_name: &str, version: &str) -> Metadata {
    let output = Command::new("cargo")
        .args(&[
            "metadata",
            "--format-version=1",
            "--locked", // Use the locked version specified in Cargo.lock
        ])
        .output()
        .expect("Failed to execute 'cargo' command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let cargo_metadata: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let mut metadata = Metadata {
        name: package_name.to_string(),
        version: version.to_string(),
        authors: "".to_string(),
        publish_date: "".to_string(),
        description: "".to_string(),
    };

    // Extract the package information for the specific version from the cargo metadata
    let package_info = cargo_metadata["packages"]
        .as_array()
        .and_then(|packages| {
            packages.iter().find(|package| {
                package["name"].as_str().unwrap_or("") == package_name
                    && package["version"].as_str().unwrap_or("") == version
            })
        })
        .map(|package| {
            serde_json::from_value::<PackageInfo>(package.clone()).unwrap()
        });

    if let Some(package) = package_info {
        metadata = fetch_metadata::fetch_metadata(&package_name.to_string(), &version.to_string());
        metadata.description = package.description.unwrap_or_else(|| "".to_string());
        metadata.authors = package.authors.unwrap_or_else(Vec::new)
            .iter()
            .map(|author| author.clone())
            .collect::<Vec<_>>()
            .join(", ");
    }
    return metadata;
}
