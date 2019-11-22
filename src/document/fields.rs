pub use crate::cmn::*;
use crate::value::*;
/// A reference to a field, such as `max(messages.time) as max_time`.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldReference {
    /// no description provided
    #[serde(rename="fieldPath")]
    pub field_path: Option<String>,
}

impl Part for FieldReference {}

/// A filter on a specific field.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct FieldFilter {
    /// The field to filter by.
    pub field: Option<FieldReference>,
    /// The value to compare to.
    pub value: Option<Value>,
    /// The operator to filter by.
    pub op: Option<String>,
}

impl Part for FieldFilter {}

