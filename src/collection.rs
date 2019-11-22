use crate::cmn::*;
use crate::value::*;

/// A selection of a collection, such as `messages as m1`.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct CollectionSelector {
    /// When false, selects only collections that are immediate children of
    /// the `parent` specified in the containing `RunQueryRequest`.
    /// When true, selects all descendant collections.
    #[serde(rename="allDescendants")]
    pub all_descendants: Option<bool>,
    /// The collection ID.
    /// When set, selects only collections with this ID.
    #[serde(rename="collectionId")]
    pub collection_id: Option<String>,
}

impl Part for CollectionSelector {}

/// A position in a query result set.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Cursor {
    /// The values that represent a position, in the order they appear in
    /// the order by clause of a query.
    ///
    /// Can contain fewer values than specified in the order by clause.
    pub values: Option<Vec<Value>>,
    /// If the position is just before or just after the given values, relative
    /// to the sort order defined by the query.
    pub before: Option<bool>,
}

impl Part for Cursor {}
