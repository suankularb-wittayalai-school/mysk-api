use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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

impl Display for MultiLangString {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{{ en: {:?}, th: {} }}", self.en, self.th)
    }
}

impl MultiLangString {
    pub fn new(th: String, en: Option<String>) -> MultiLangString {
        MultiLangString { en, th }
    }
}

impl Display for FlexibleMultiLangString {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{{ en: {:?}, th: {:?} }}", self.en, self.th)
    }
}

impl FlexibleMultiLangString {
    pub fn new(th: Option<String>, en: Option<String>) -> FlexibleMultiLangString {
        FlexibleMultiLangString { en, th }
    }
}
