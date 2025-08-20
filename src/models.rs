use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelMetadata {
    pub description: String,
    pub name: String,
    pub commended: bool,
    pub local: bool,
    pub version: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tile {
    Int(f64),
    Object {
        meta: serde_json::Value,
        tile: f64,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    pub key: Vec<String>,
    pub map: Vec<Vec<Vec<Tile>>>,
    pub metadata: LevelMetadata,
    pub solution: Vec<i32>,
    pub size: [i32; 2],
    pub spawn: [i32; 2],
}
