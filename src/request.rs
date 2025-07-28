// File: src/request.rs
// =============================================================================
// This file defines the top-level `Request` enum. This is the single, unified
// type that represents every possible command a client can send to the server.

use crate::types::{BatchRequest, Filter, QueryOptions, Record};
use serde::{Deserialize, Serialize};

/// The primary enum representing all possible client requests.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Request {
    // --- DB Management ---
    ListCollections,
    CreateCollection { name: String },
    DropCollection { name: String },
    GetStats,
    Flush,

    // --- Index Management ---
    CreateIndex { collection: String, field_name: String },
    DropIndex { collection: String, field_name: String },
    ListIndexes { collection: String },

    // --- Record Operations (CRUD) ---
    CreateRecord { collection: String, record_id: String, data: Record },
    UpdateRecord { collection: String, record_id: String, data: Record },
    UpsertRecord { collection: String, record_id: String, data: Record },
    GetRecord { collection: String, record_id: String },
    DeleteRecord { collection: String, record_id: String, cascade: bool },
    GetLastInsertId,

    // --- Querying & Relational ---
    FindRecords {
        collection: String,
        filter: Filter,
        options: Option<QueryOptions>,
    },
    CountRecords {
        collection: String,
        filter: Filter,
    },
    GetRecordWithRelated {
        primary_collection: String,
        primary_record_id: String,
        relation_key_field: String,
        related_collection: String,
    },
    ExecuteBatchGet(BatchRequest),
}