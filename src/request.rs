// File: src/request.rs
// =============================================================================
// This file defines the top-level `Request` enum. This is the single, unified
// type that represents every possible command a client can send to the server.

use crate::types::{BatchRequest, Filter, QueryOptions, Record};
use serde::{Deserialize, Serialize};

/// The primary enum representing all possible client requests.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Request {
    // --- Database Management ---
    CreateDatabase { db_name: String },
    DropDatabase { db_name: String },
    ListDatabases,

    // --- Collection Management ---
    ListCollections,
    CreateCollection { db_name: String, collection_name: String },
    DropCollection { db_name: String, collection_name: String },
    GetStats,
    Flush,

    // --- Index Management ---
    CreateIndex { db_name: String, collection: String, field_name: String },
    DropIndex { db_name: String, collection: String, field_name: String },
    ListIndexes { db_name: String, collection: String },

    // --- Record Operations (CRUD) ---
    CreateRecord { db_name: String, collection: String, record_id: String, data: Record },
    UpdateRecord { db_name: String, collection: String, record_id: String, data: Record },
    UpsertRecord { db_name: String, collection: String, record_id: String, data: Record },
    GetRecord { db_name: String, collection: String, record_id: String },
    DeleteRecord { db_name: String, collection: String, record_id: String, cascade: bool },
    GetLastInsertId,

    // --- Querying & Relational ---
    FindRecords {
        db_name: String,
        collection: String,
        filter: Filter,
        options: Option<QueryOptions>,
    },
    CountRecords {
        db_name: String,
        collection: String,
        filter: Filter,
    },
    GetRecordWithRelated {
        db_name: String,
        primary_collection: String,
        primary_record_id: String,
        relation_key_field: String,
        related_collection: String, 
    },
    ExecuteBatchGet(BatchRequest),
}