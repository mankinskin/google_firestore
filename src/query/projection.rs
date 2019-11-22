use crate::document::*;

/// The projection of document's fields to return.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Projection {
    /// The fields to return.
    ///
    /// If empty, all fields are returned. To only return the name
    /// of the document, use `['__name__']`.
    pub fields: Option<Vec<FieldReference>>,
}

impl Part for Projection {}
