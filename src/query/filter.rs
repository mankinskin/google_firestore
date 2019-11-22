use crate::document::*;

/// A filter with a single operand.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct UnaryFilter {
    /// The field to which to apply the operator.
    pub field: Option<FieldReference>,
    /// The unary operator to apply.
    pub op: Option<String>,
}

impl Part for UnaryFilter {}

/// A filter.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    /// A composite filter.
    #[serde(rename="compositeFilter")]
    pub composite_filter: Option<CompositeFilter>,
    /// A filter on a document field.
    #[serde(rename="fieldFilter")]
    pub field_filter: Option<FieldFilter>,
    /// A filter that takes exactly one argument.
    #[serde(rename="unaryFilter")]
    pub unary_filter: Option<UnaryFilter>,
}

impl Part for Filter {}

/// A filter that merges multiple other filters using the given operator.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CompositeFilter {
    /// The list of filters to combine.
    /// Must contain at least one filter.
    pub filters: Option<Vec<Filter>>,
    /// The operator for combining multiple filters.
    pub op: Option<String>,
}

impl Part for CompositeFilter {}

