// Field Type Enum - Simplified for initial implementation
// Full Diesel integration will be added after schema update

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldType {
    Text,
    Textarea,
    Wysiwyg,
    Json,
    Number,
    Boolean,
    Date,
    Datetime,
    FileReference,
    PageReference,
    Select,
    MultiSelect,
}

impl FieldType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldType::Text => "text",
            FieldType::Textarea => "textarea",
            FieldType::Wysiwyg => "wysiwyg",
            FieldType::Json => "json",
            FieldType::Number => "number",
            FieldType::Boolean => "boolean",
            FieldType::Date => "date",
            FieldType::Datetime => "datetime",
            FieldType::FileReference => "file_reference",
            FieldType::PageReference => "page_reference",
            FieldType::Select => "select",
            FieldType::MultiSelect => "multi_select",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "text" => Some(FieldType::Text),
            "textarea" => Some(FieldType::Textarea),
            "wysiwyg" => Some(FieldType::Wysiwyg),
            "json" => Some(FieldType::Json),
            "number" => Some(FieldType::Number),
            "boolean" => Some(FieldType::Boolean),
            "date" => Some(FieldType::Date),
            "datetime" => Some(FieldType::Datetime),
            "file_reference" => Some(FieldType::FileReference),
            "page_reference" => Some(FieldType::PageReference),
            "select" => Some(FieldType::Select),
            "multi_select" => Some(FieldType::MultiSelect),
            _ => None,
        }
    }
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::Text
    }
}
