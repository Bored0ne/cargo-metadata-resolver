use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Metadata {
    pub name: String,
    pub version: String,
    pub authors: String,
    pub publish_date: String,
    pub description: String,
}