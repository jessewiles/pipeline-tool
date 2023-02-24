use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Pipeline {
    name: String,
    exe: String,
    args: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipelines {
    pipelines: Vec<Pipeline>,
}
