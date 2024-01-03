use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ZigDist {
    pub master: Master,
    #[serde(flatten)]
    pub versions: HashMap<String, Empty>,
}

#[derive(Deserialize)]
pub struct Master {
    pub version: String,
}

#[derive(Deserialize)]
pub struct Empty {}
