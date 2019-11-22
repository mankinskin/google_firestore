use crate::cmn::*;

/// Options for creating a new transaction.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct TransactionOptions {
    /// The transaction can be used for both read and write operations.
    #[serde(rename="readWrite")]
    pub read_write: Option<ReadWrite>,
    /// The transaction can only be used for read operations.
    #[serde(rename="readOnly")]
    pub read_only: Option<ReadOnly>,
}

impl Part for TransactionOptions {}

/// Options for a transaction that can be used to read and write documents.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadWrite {
    /// An optional transaction to retry.
    #[serde(rename="retryTransaction")]
    pub retry_transaction: Option<String>,
}

impl Part for ReadWrite {}

/// Options for a transaction that can only be used to read documents.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ReadOnly {
    /// Reads documents at the given time.
    /// This may not be older than 60 seconds.
    #[serde(rename="readTime")]
    pub read_time: Option<String>,
}

impl Part for ReadOnly {}
