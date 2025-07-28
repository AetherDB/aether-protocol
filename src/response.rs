// File: src/response.rs
// =============================================================================
// This file defines the top-level `Response` enum. This is the single, unified
// type that represents every possible reply the server can send to a client.

use crate::types::{BatchResponse, DbStats, Record, RecordSet};
use serde::{Deserialize, Serialize};

/// The primary enum representing all possible server responses.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Response {
    // --- General Responses ---
    Success,
    Error(String),

    // --- Database Management Responses ---
    DatabaseList(Vec<String>),
    DatabaseCreated(bool),
    DatabaseDropped(bool),

    // --- Collection Management Responses ---
    CollectionList(Vec<String>),
    Stats(DbStats),
    IndexList(Vec<String>),

    // --- Record & Query Responses ---
    Record(Option<Record>),
    RecordSet(RecordSet),
    RecordCount(u64),
    RecordDeleted(bool),
    LastInsertId(u64),
    RecordWithRelated(Option<(Record, Record)>),
    BatchResponse(BatchResponse),
    RecordIdSet(Vec<String>),
}