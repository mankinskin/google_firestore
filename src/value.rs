pub use crate::cmn::*;
use std::collections::HashMap;
/// A message that can hold any of the supported value types.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Value {
    /// A bytes value.
    ///
    /// Must not exceed 1 MiB - 89 bytes.
    /// Only the first 1,500 bytes are considered by queries.
    #[serde(rename="bytesValue")]
    pub bytes_value: Option<String>,
    /// A timestamp value.
    ///
    /// Precise only to microseconds. When stored, any additional precision is
    /// rounded down.
    #[serde(rename="timestampValue")]
    pub timestamp_value: Option<String>,
    /// A geo point value representing a point on the surface of Earth.
    #[serde(rename="geoPointValue")]
    pub geo_point_value: Option<LatLng>,
    /// A reference to a document. For example:
    /// `projects/{project_id}/databases/{database_id}/documents/{document_path}`.
    #[serde(rename="referenceValue")]
    pub reference_value: Option<String>,
    /// A double value.
    #[serde(rename="doubleValue")]
    pub double_value: Option<f64>,
    /// A map value.
    #[serde(rename="mapValue")]
    pub map_value: Option<MapValue>,
    /// A string value.
    ///
    /// The string, represented as UTF-8, must not exceed 1 MiB - 89 bytes.
    /// Only the first 1,500 bytes of the UTF-8 representation are considered by
    /// queries.
    #[serde(rename="stringValue")]
    pub string_value: Option<String>,
    /// A boolean value.
    #[serde(rename="booleanValue")]
    pub boolean_value: Option<bool>,
    /// An array value.
    ///
    /// Cannot directly contain another array value, though can contain an
    /// map which contains another array.
    #[serde(rename="arrayValue")]
    pub array_value: Option<ArrayValue>,
    /// An integer value.
    #[serde(rename="integerValue")]
    pub integer_value: Option<String>,
    /// A null value.
    #[serde(rename="nullValue")]
    pub null_value: Option<String>,
}

impl Part for Value {}

impl std::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.null_value == other.null_value &&
        self.boolean_value == other.boolean_value &&
        self.integer_value == other.integer_value &&
        self.bytes_value == other.bytes_value &&
        self.string_value == other.string_value &&
        self.double_value == other.double_value &&
        self.reference_value == other.reference_value &&
        self.array_value == other.array_value &&
        self.map_value == other.map_value &&
        self.timestamp_value == other.timestamp_value &&
        self.geo_point_value == other.geo_point_value
    }
}

/// An array value.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ArrayValue {
    /// Values in the array.
    pub values: Option<Vec<Value>>,
}

impl Part for ArrayValue {}

impl std::cmp::PartialEq for ArrayValue {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}

/// A map value.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MapValue {
    /// The map's fields.
    ///
    /// The map keys represent field names. Field names matching the regular
    /// expression `__.*__` are reserved. Reserved field names are forbidden except
    /// in certain documented contexts. The map keys, represented as UTF-8, must
    /// not exceed 1,500 bytes and cannot be empty.
    pub fields: Option<HashMap<String, Value>>,
}

impl Part for MapValue {}

impl std::cmp::PartialEq for MapValue {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields
    }
}

/// An object representing a latitude/longitude pair. This is expressed as a pair
/// of doubles representing degrees latitude and degrees longitude. Unless
/// specified otherwise, this must conform to the
/// <a href="http://www.unoosa.org/pdf/icg/2012/template/WGS_84.pdf">WGS84
/// standard</a>. Values must be within normalized ranges.
///
/// This type is not used in any activity, and only used as *part* of another schema.
///
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct LatLng {
    /// The latitude in degrees. It must be in the range [-90.0, +90.0].
    pub latitude: Option<f64>,
    /// The longitude in degrees. It must be in the range [-180.0, +180.0].
    pub longitude: Option<f64>,
}

impl Part for LatLng {}

impl std::cmp::PartialEq for LatLng {
    fn eq(&self, other: &Self) -> bool {
        self.latitude == other.latitude &&
        self.longitude == other.longitude
    }
}

/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
///
/// ````text
/// service Foo {
///   rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
/// }
/// ````
///
/// The JSON representation for `Empty` is empty JSON object `{}`.
///
/// # Activities
///
/// This type is used in activities, which are methods you may call on this type or where this type is involved in.
//// The list links the activity name, along with information about where it is used (one of *request* and *response*).
///
/// * [databases documents delete projects](struct.ProjectDatabaseDocumentDeleteCall.html) (response)
/// * [databases operations cancel projects](struct.ProjectDatabaseOperationCancelCall.html) (response)
/// * [databases operations delete projects](struct.ProjectDatabaseOperationDeleteCall.html) (response)
/// * [databases collection groups indexes delete projects](struct.ProjectDatabaseCollectionGroupIndexeDeleteCall.html) (response)
/// * [databases documents rollback projects](struct.ProjectDatabaseDocumentRollbackCall.html) (response)
#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Empty { _never_set: Option<bool> }

impl ResponseResult for Empty {}
