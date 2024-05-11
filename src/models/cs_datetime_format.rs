use std::borrow::Borrow;

use serde::{Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum CSDateTimeFormat {
    Basic,
    Any(String),
}

impl From<String> for CSDateTimeFormat {
    fn from(value: String) -> Self {
        match value.borrow() {
            "MM/dd/yyyy" => Self::Basic,
            _ => Self::Any(value),
        }
    }
}

impl From<CSDateTimeFormat> for String {
    fn from(value: CSDateTimeFormat) -> Self {
        match value {
            CSDateTimeFormat::Basic => "MM/dd/yyyy".to_string(),
            CSDateTimeFormat::Any(any) => any,
        }
    }
}

impl From<&CSDateTimeFormat> for String {
    fn from(value: &CSDateTimeFormat) -> Self {
        match value {
            CSDateTimeFormat::Basic => "MM/dd/yyyy".to_string(),
            CSDateTimeFormat::Any(any) => any.to_string(),
        }
    }
}

impl Default for CSDateTimeFormat {
    fn default() -> Self {
        Self::Basic
    }
}

impl Serialize for CSDateTimeFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&String::from(self))
    }
}
