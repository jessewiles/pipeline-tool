use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    pub name: String,
    pub exe: String,
    pub args: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipelines {
    pub pipelines: Vec<Pipeline>,
}
