// File: src/types.rs
// =============================================================================
// This file defines all the shared data structures that are used in both
// requests and responses. Keeping them separate ensures they can be reused
// without creating circular dependencies.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// A type alias for a single record, represented as a map of field names to JSON values.
pub type Record = HashMap<String, Value>;

/// Represents a set of records returned from a query.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct RecordSet {
    pub records: Vec<Record>,
}

/// Defines a filter for querying records (the "WHERE" clause).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Filter {
    Equals { field: String, value: Value },
    NotEquals { field: String, value: Value },
    GreaterThan { field: String, value: f64 },
    LessThan { field: String, value: f64 },
    In { field: String, values: Vec<Value> },
    And(Vec<Filter>),
    Or(Vec<Filter>),
}

/// Defines query modifiers like sorting, limiting, and pagination.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct QueryOptions {
    pub sort_by: Option<(String, Direction)>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Enum for sorting direction.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Asc,
    Desc,
}

/// A struct to hold database statistics.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DbStats {
    pub collection_count: usize,
    pub record_count: usize,
}

/// A request object for a batch of read operations.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct BatchRequest {
    pub requests: HashMap<String, (String, String)>, // Key -> (Collection, Record ID)
}

/// The response from a batch read operation.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct BatchResponse {
    pub results: HashMap<String, Option<Record>>,
}