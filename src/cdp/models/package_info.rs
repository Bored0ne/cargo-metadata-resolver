use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PackageInfo {
    pub(crate) description: Option<String>,
    pub(crate) authors: Option<Vec<String>>,
}