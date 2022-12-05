use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub name: String,
    pub content: String,
}
