use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct MultiLangString {
    #[serde(rename = "en-US")]
    pub en: Option<String>,
    pub th: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Clone)]
pub struct FlexibleMultiLangString {
    #[serde(rename = "en-US")]
    pub en: Option<String>,
    pub th: Option<String>,
}

impl std::fmt::Display for MultiLangString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ en: {:?}, th: {} }}", self.en, self.th)
    }
}

impl MultiLangString {
    pub fn new(th: String, en: Option<String>) -> MultiLangString {
        MultiLangString { en, th }
    }
}

impl std::fmt::Display for FlexibleMultiLangString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ en: {:?}, th: {:?} }}", self.en, self.th)
    }
}

impl FlexibleMultiLangString {
    pub fn new(th: Option<String>, en: Option<String>) -> FlexibleMultiLangString {
        FlexibleMultiLangString { en, th }
    }
}
