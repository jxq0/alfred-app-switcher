use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AlfredItem {
    #[serde(rename = "type")]
    item_type: String,

    title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<String>,

    variables: HashMap<String, String>,
}

impl AlfredItem {
    pub fn new(title: String) -> Self {
        Self {
            item_type: "default".to_string(),
            title: title.to_owned(),
            subtitle: None,
            variables: HashMap::from([("profile".to_string(), title)]),
        }
    }

    pub fn new_with_sub(title: String, subtitle: String) -> Self {
        Self {
            item_type: "default".to_string(),
            title: title.to_owned(),
            subtitle: Some(subtitle),
            variables: HashMap::from([("profile".to_string(), title)]),
        }
    }
}
