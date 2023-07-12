use std::collections::BTreeMap;
use serde::Deserialize;
use super::DepValue;

#[derive(Deserialize, Debug)]
pub struct CargoFile {
    pub(crate) dependencies: BTreeMap<String, DepValue>,
}
