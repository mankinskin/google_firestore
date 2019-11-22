mod filter;
mod projection;

pub use filter::*;
pub use projection::*;

use crate::cmn::*;
use crate::collection::*;
pub use crate::document::*;
pub use crate::transaction::*;

/// An order on a field.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Order {
    /// The field to order by.
    pub field: Option<FieldReference>,
    /// The direction to order by. Defaults to `ASCENDING`.
    pub direction: Option<String>,
}

impl Part for Order {}

/// A Firestore query.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct StructuredQuery {
    /// The order to apply to the query results.
    ///
    /// Firestore guarantees a stable ordering through the following rules:
    ///
    ///  * Any field required to appear in `order_by`, that is not already
    ///    specified in `order_by`, is appended to the order in field name order
    ///    by default.
    ///  * If an order on `__name__` is not specified, it is appended by default.
    ///
    /// Fields are appended with the same sort direction as the last order
    /// specified, or 'ASCENDING' if no order was specified. For example:
    ///
    ///  * `SELECT * FROM Foo ORDER BY A` becomes
    ///    `SELECT * FROM Foo ORDER BY A, __name__`
    ///  * `SELECT * FROM Foo ORDER BY A DESC` becomes
    ///    `SELECT * FROM Foo ORDER BY A DESC, __name__ DESC`
    ///  * `SELECT * FROM Foo WHERE A > 1` becomes
    ///    `SELECT * FROM Foo WHERE A > 1 ORDER BY A, __name__`
    #[serde(rename="orderBy")]
    pub order_by: Option<Vec<Order>>,
    /// A starting point for the query results.
    #[serde(rename="startAt")]
    pub start_at: Option<Cursor>,
    /// A end point for the query results.
    #[serde(rename="endAt")]
    pub end_at: Option<Cursor>,
    /// The maximum number of results to return.
    ///
    /// Applies after all other constraints.
    /// Must be >= 0 if specified.
    pub limit: Option<i32>,
    /// The number of results to skip.
    ///
    /// Applies before limit, but after all other constraints. Must be >= 0 if
    /// specified.
    pub offset: Option<i32>,
    /// The collections to query.
    pub from: Option<Vec<CollectionSelector>>,
    /// The filter to apply.
    #[serde(rename="where")]
    pub where_: Option<Filter>,
    /// The projection to return.
    pub select: Option<Projection>,
}

impl Part for StructuredQuery {}

/// The request for Firestore.RunQuery.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
//// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [databases documents run query projects](struct.ProjectDatabaseDocumentRunQueryCall.html) (request)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunQueryRequest {
    /// Starts a new transaction and reads the documents.
    /// Defaults to a read-only transaction.
    /// The new transaction ID will be returned as the first response in the
    /// stream.
    #[serde(rename="newTransaction")]
    pub new_transaction: Option<TransactionOptions>,
    /// Reads documents in a transaction.
    pub transaction: Option<String>,
    /// A structured query.
    #[serde(rename="structuredQuery")]
    pub structured_query: Option<StructuredQuery>,
    /// Reads documents as they were at the given time.
    /// This may not be older than 60 seconds.
    #[serde(rename="readTime")]
    pub read_time: Option<String>,
}

impl RequestValue for RunQueryRequest {}

/// The response for Firestore.RunQuery.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
//// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [databases documents run query projects](struct.ProjectDatabaseDocumentRunQueryCall.html) (response)
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct RunQueryResponse {
    /// The number of results that have been skipped due to an offset between
    /// the last response and the current response.
    #[serde(rename="skippedResults")]
    pub skipped_results: Option<i32>,
    /// The transaction that was started as part of this request.
    /// Can only be set in the first response, and only if
    /// RunQueryRequest.new_transaction was set in the request.
    /// If set, no other fields will be set in this response.
    pub transaction: Option<String>,
    /// A query result.
    /// Not set when reporting partial progress.
    pub document: Option<Document>,
    /// The time at which the document was read. This may be monotonically
    /// increasing; in this case, the previous documents in the result stream are
    /// guaranteed not to have changed between their `read_time` and this one.
    ///
    /// If the query returns no results, a response with `read_time` and no
    /// `document` will be sent, and this represents the time at which the query
    /// was run.
    #[serde(rename="readTime")]
    pub read_time: Option<String>,
}

impl ResponseResult for RunQueryResponse {}
