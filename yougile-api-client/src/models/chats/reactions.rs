use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Reactions {
    #[serde(flatten)]
    pub reactions: HashMap<String, Vec<String>>,
}

impl Reactions {
    pub fn new() -> Self {
        Self {
            reactions: HashMap::new(),
        }
    }

    pub fn add_reaction(&mut self, emoji: String, user_ids: Vec<String>) {
        self.reactions.insert(emoji, user_ids);
    }
}

