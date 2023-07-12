use serde::Deserialize;
use super::DepShape;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DepValue {
    String(String),
    DepShape(DepShape),
}
