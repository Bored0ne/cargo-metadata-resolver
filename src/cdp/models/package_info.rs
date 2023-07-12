use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PackageInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
}