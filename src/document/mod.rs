use std::collections::HashMap;

mod fields;
pub use fields::*;
pub use crate::value::*;

/// A Firestore document.
///
/// Must not exceed 1 MiB - 4 bytes.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
/// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [databases documents get projects](struct.ProjectDatabaseDocumentGetCall.html) (response)
/// * [databases documents create document projects](struct.ProjectDatabaseDocumentCreateDocumentCall.html) (request|response)
/// * [databases documents patch projects](struct.ProjectDatabaseDocumentPatchCall.html) (request|response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    /// The document's fields.
    ///
    /// The map keys represent field names.
    ///
    /// A simple field name contains only characters `a` to `z`, `A` to `Z`,
    /// `0` to `9`, or `_`, and must not start with `0` to `9`. For example,
    /// `foo_bar_17`.
    ///
    /// Field names matching the regular expression `__.*__` are reserved. Reserved
    /// field names are forbidden except in certain documented contexts. The map
    /// keys, represented as UTF-8, must not exceed 1,500 bytes and cannot be
    /// empty.
    ///
    /// Field paths may be used in other contexts to refer to structured fields
    /// defined here. For `map_value`, the field path is represented by the simple
    /// or quoted field names of the containing fields, delimited by `.`. For
    /// example, the structured field
    /// `"foo" : { map_value: { "x&y" : { string_value: "hello" }}}` would be
    /// represented by the field path `foo.x&y`.
    ///
    /// Within a field path, a quoted field name starts and ends with `` ` `` and
    /// may contain any character. Some characters, including `` ` ``, must be
    /// escaped using a `\`. For example, `` `x&y` `` represents `x&y` and
    /// `` `bak\`tik` `` represents `` bak`tik ``.
    pub fields: Option<HashMap<String, Value>>,
    /// Output only. The time at which the document was last changed.
    ///
    /// This value is initially set to the `create_time` then increases
    /// monotonically with each change to the document. It can also be
    /// compared to values from other documents and the `read_time` of a query.
    #[serde(rename="updateTime")]
    pub update_time: Option<String>,
    /// Output only. The time at which the document was created.
    ///
    /// This value increases monotonically when a document is deleted then
    /// recreated. It can also be compared to values from other documents and
    /// the `read_time` of a query.
    #[serde(rename="createTime")]
    pub create_time: Option<String>,
    /// The resource name of the document, for example
    /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    pub name: Option<String>,
}

impl RequestValue for Document {}
impl ResponseResult for Document {}

impl std::cmp::PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.fields == other.fields
    }
}
