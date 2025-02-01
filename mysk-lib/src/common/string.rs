use serde::{Deserialize, Serialize};

// NOTE: The "en" field is renamed to "en-US" in the JSON representation
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MultiLangString {
    #[serde(rename = "en-US")]
    pub en: Option<String>,
    pub th: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlexibleMultiLangString {
    #[serde(rename = "en-US")]
    pub en: Option<String>,
    pub th: Option<String>,
}

impl MultiLangString {
    pub fn new(th: String, en: Option<String>) -> MultiLangString {
        MultiLangString { en, th }
    }
}

impl FlexibleMultiLangString {
    pub fn new(th: Option<String>, en: Option<String>) -> FlexibleMultiLangString {
        FlexibleMultiLangString { en, th }
    }
}
