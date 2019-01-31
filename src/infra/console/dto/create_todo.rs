use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CTodoInput {
    pub name: String,
}

impl CTodoInput{
    pub fn new(name: String) -> CTodoInput{
        CTodoInput{name}
    }
}
