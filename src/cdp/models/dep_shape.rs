use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DepShape {
    pub(crate) version: String,
}
