use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub id: String,
    pub name: String,
    // consider making these default to false instead of being options.
    pub title: Option<String>,
    pub online: Option<bool>,
    pub playing: Option<bool>,
    pub streaming: Option<bool>,
    pub patron: Option<bool>,
}
