// File: src/lib.rs
// =============================================================================
// This is the main library file. Its primary job is to declare the modules
// and re-export their contents so they are easily accessible to any crate
// that uses `aether-protocol`.

//! # AetherDB Network Protocol
//!
//! This crate defines the official, stable network protocol for communicating
//! with an AetherDB instance. It contains all request and response data
//! structures, serialized using `bincode` for maximum performance.

// Declare the modules that make up our library.
pub mod request;
pub mod response;
pub mod types;

// Re-export the most important structs and enums for convenience.
pub use request::Request;
pub use response::Response;
pub use types::{BatchRequest, BatchResponse, DbStats, Direction, Filter, QueryOptions, Record, RecordSet};

#[cfg(test)]
mod tests {
    use crate::types::{BatchRequest, BatchResponse, DbStats, Direction, Filter, QueryOptions, Record, RecordSet};
    use crate::{Request, Response};
    use serde_json::json;
    use std::collections::HashMap;

    // Helper functions to test serialization/deserialization roundtrip
    
    // Use serde_json for testing since it handles serde_json::Value better than bincode
    fn test_serialization_json<T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug>(
        value: T,
    ) -> T {
        let serialized = serde_json::to_string(&value).expect("Failed to serialize to JSON");
        let deserialized = serde_json::from_str(&serialized).expect("Failed to deserialize from JSON");
        assert_eq!(value, deserialized, "Data loss during JSON serialization roundtrip");
        deserialized
    }
    
    // For non-JSON-Value types, we can still use bincode to ensure it works
    fn test_serialization_bincode<T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug>(
        value: T,
    ) -> T {
        let serialized = bincode::serialize(&value).expect("Failed to serialize");
        let deserialized = bincode::deserialize(&serialized).expect("Failed to deserialize");
        assert_eq!(value, deserialized, "Data loss during bincode serialization roundtrip");
        deserialized
    }

    #[test]
    fn test_record_serialization() {
        let mut record = Record::new();
        record.insert("name".to_string(), json!("John Doe"));
        record.insert("age".to_string(), json!(30));
        record.insert("active".to_string(), json!(true));
        record.insert("scores".to_string(), json!([85, 90, 78]));
        
        let deserialized = test_serialization_json(record);
        assert_eq!(deserialized["name"], json!("John Doe"));
    }

    #[test]
    fn test_recordset_serialization() {
        let mut record1 = Record::new();
        record1.insert("id".to_string(), json!(1));
        record1.insert("name".to_string(), json!("Record 1"));
        
        let mut record2 = Record::new();
        record2.insert("id".to_string(), json!(2));
        record2.insert("name".to_string(), json!("Record 2"));
        
        let recordset = RecordSet {
            records: vec![record1, record2],
        };
        
        test_serialization_json(recordset);
    }

    #[test]
    fn test_filter_serialization() {
        // Test each Filter variant
        let filters = vec![
            Filter::Equals {
                field: "status".to_string(),
                value: json!("active"),
            },
            Filter::NotEquals {
                field: "deleted".to_string(),
                value: json!(true),
            },
            Filter::GreaterThan {
                field: "age".to_string(),
                value: 18.0,
            },
            Filter::LessThan {
                field: "price".to_string(),
                value: 100.0,
            },
            Filter::In {
                field: "category".to_string(),
                values: vec![json!("electronics"), json!("books")],
            },
            Filter::And(vec![
                Filter::Equals {
                    field: "active".to_string(),
                    value: json!(true),
                },
                Filter::GreaterThan {
                    field: "score".to_string(),
                    value: 70.0,
                },
            ]),
            Filter::Or(vec![
                Filter::Equals {
                    field: "type".to_string(),
                    value: json!("premium"),
                },
                Filter::Equals {
                    field: "special".to_string(),
                    value: json!(true),
                },
            ]),
        ];
        
        for filter in filters {
            test_serialization_json(filter);
        }
    }

    #[test]
    fn test_query_options_serialization() {
        let options = QueryOptions {
            sort_by: Some(("created_at".to_string(), Direction::Desc)),
            limit: Some(100),
            offset: Some(20),
        };
        
        // Can use bincode for this since it doesn't have serde_json::Value
        test_serialization_bincode(options);
    }

    #[test]
    fn test_db_stats_serialization() {
        let stats = DbStats {
            collection_count: 5,
            record_count: 1000,
        };
        
        // Can use bincode for this since it doesn't have serde_json::Value
        test_serialization_bincode(stats);
    }

    #[test]
    fn test_batch_request_serialization() {
        let mut requests = HashMap::new();
        requests.insert("key1".to_string(), ("testdb".to_string(), "users".to_string(), "user_1".to_string()));
        requests.insert("key2".to_string(), ("testdb".to_string(), "products".to_string(), "product_1".to_string()));
        
        let batch_request = BatchRequest { requests };
        // Can use bincode for this since it doesn't have serde_json::Value
        test_serialization_bincode(batch_request);
    }

    #[test]
    fn test_batch_response_serialization() {
        let mut record1 = Record::new();
        record1.insert("id".to_string(), json!("user_1"));
        record1.insert("name".to_string(), json!("John Doe"));
        
        let mut record2 = Record::new();
        record2.insert("id".to_string(), json!("product_1"));
        record2.insert("name".to_string(), json!("Widget"));
        
        let mut results = HashMap::new();
        results.insert("key1".to_string(), Some(record1));
        results.insert("key2".to_string(), Some(record2));
        results.insert("key3".to_string(), None); // Test None case
        
        let batch_response = BatchResponse { results };
        test_serialization_json(batch_response);
    }

    #[test]
    fn test_request_serialization() {
        // Test all Request variants
        let requests = vec![
            // Database Management
            Request::CreateDatabase { db_name: "testdb".to_string() },
            Request::DropDatabase { db_name: "testdb".to_string() },
            Request::ListDatabases,
            
            // Collection Management
            Request::ListCollections,
            Request::CreateCollection { db_name: "users".to_string(), collection_name: "users".to_string() },
            Request::DropCollection { db_name: "users".to_string(), collection_name: "users".to_string() },
            Request::GetStats,
            Request::Flush,
            
            // Index Management
            Request::CreateIndex {
                db_name: "users".to_string(),
                collection: "users".to_string(),
                field_name: "email".to_string(),
            },
            Request::DropIndex {
                db_name: "users".to_string(),
                collection: "users".to_string(),
                field_name: "email".to_string(),
            },
            Request::ListIndexes {
                db_name: "users".to_string(),
                collection: "users".to_string(),
            },
            
            // CRUD Operations
            Request::CreateRecord {
                db_name: "users".to_string(),
                collection: "users".to_string(),
                record_id: "user123".to_string(),
                data: {
                    let mut record = Record::new();
                    record.insert("name".to_string(), json!("Alice"));
                    record.insert("email".to_string(), json!("alice@example.com"));
                    record
                },
            },
            Request::UpdateRecord {
                db_name: "users".to_string(),
                collection: "users".to_string(),
                record_id: "user123".to_string(),
                data: {
                    let mut record = Record::new();
                    record.insert("active".to_string(), json!(false));
                    record
                },
            },
            Request::UpsertRecord {
                db_name: "users".to_string(),
                collection: "users".to_string(),
                record_id: "user123".to_string(),
                data: {
                    let mut record = Record::new();
                    record.insert("name".to_string(), json!("Alice"));
                    record.insert("email".to_string(), json!("updated@example.com"));
                    record
                },
            },
            Request::GetRecord {
                db_name: "users".to_string(),
                collection: "users".to_string(),
                record_id: "user123".to_string(),
            },
            Request::DeleteRecord {
                db_name: "users".to_string(),
                collection: "users".to_string(),
                record_id: "user123".to_string(),
                cascade: true,
            },
            Request::GetLastInsertId,
            
            // Querying & Relational
            Request::FindRecords {
                db_name: "users".to_string(),
                collection: "users".to_string(),
                filter: crate::types::Filter::And(vec![
                    crate::types::Filter::Equals {
                        field: "active".to_string(),
                        value: json!(true),
                    },
                    crate::types::Filter::GreaterThan {
                        field: "age".to_string(),
                        value: 21.0,
                    },
                ]),
                options: Some(crate::types::QueryOptions {
                    sort_by: Some(("created_at".to_string(), crate::types::Direction::Desc)),
                    limit: Some(50),
                    offset: Some(0),
                }),
            },
            Request::CountRecords {
                db_name: "users".to_string(),
                collection: "users".to_string(),
                filter: crate::types::Filter::Equals {
                    field: "active".to_string(),
                    value: json!(true),
                },
            },
            Request::GetRecordWithRelated {
                db_name: "users".to_string(),
                primary_collection: "orders".to_string(),
                primary_record_id: "order123".to_string(),
                relation_key_field: "user_id".to_string(),
                related_collection: "users".to_string(),
            },
            Request::ExecuteBatchGet({
                let mut requests = HashMap::new();
                requests.insert("key1".to_string(), ("testdb".to_string(), "users".to_string(), "user123".to_string()));
                requests.insert("key2".to_string(), ("testdb".to_string(), "products".to_string(), "product456".to_string()));
                crate::types::BatchRequest { requests }
            }),
        ];
        
        for request in requests {
            test_serialization_json(request);
        }
    }

    #[test]
    fn test_response_serialization() {
        // Test all Response variants
        let responses = vec![
            // General Responses
            Response::Success,
            Response::Error("Invalid request format".to_string()),
            
            // Database Management Responses
            Response::DatabaseList(vec![
                "testdb".to_string(),
                "userdb".to_string(),
                "analytics".to_string(),
            ]),
            Response::DatabaseCreated(true),
            Response::DatabaseDropped(true),
            
            // Collection Management Responses
            Response::CollectionList(vec![
                "users".to_string(),
                "products".to_string(),
                "orders".to_string(),
            ]),
            Response::Stats(crate::types::DbStats {
                collection_count: 3,
                record_count: 1500,
            }),
            Response::IndexList(vec![
                "email".to_string(),
                "username".to_string(),
            ]),
            
            // Record & Query Responses
            Response::Record(Some({
                let mut record = Record::new();
                record.insert("id".to_string(), json!("user123"));
                record.insert("name".to_string(), json!("Bob"));
                record.insert("email".to_string(), json!("bob@example.com"));
                record
            })),
            Response::Record(None), // Test None case
            Response::RecordSet(crate::types::RecordSet {
                records: vec![
                    {
                        let mut record = Record::new();
                        record.insert("id".to_string(), json!("1"));
                        record.insert("name".to_string(), json!("Item 1"));
                        record
                    },
                    {
                        let mut record = Record::new();
                        record.insert("id".to_string(), json!("2"));
                        record.insert("name".to_string(), json!("Item 2"));
                        record
                    },
                ],
            }),
            Response::RecordCount(42),
            Response::RecordDeleted(true),
            Response::LastInsertId(123),
            Response::RecordWithRelated(Some(({
                let mut order = Record::new();
                order.insert("id".to_string(), json!("order123"));
                order.insert("amount".to_string(), json!(99.99));
                order
            }, {
                let mut user = Record::new();
                user.insert("id".to_string(), json!("user456"));
                user.insert("name".to_string(), json!("Charlie"));
                user
            }))),
            Response::RecordWithRelated(None), // Test None case
            Response::BatchResponse({
                let mut results = HashMap::new();
                let mut user_record = Record::new();
                user_record.insert("id".to_string(), json!("user123"));
                user_record.insert("name".to_string(), json!("Dave"));
                
                let mut product_record = Record::new();
                product_record.insert("id".to_string(), json!("product456"));
                product_record.insert("name".to_string(), json!("Gadget"));
                
                results.insert("key1".to_string(), Some(user_record));
                results.insert("key2".to_string(), Some(product_record));
                results.insert("key3".to_string(), None); // Test None case
                
                crate::types::BatchResponse { results }
            }),
        ];
        
        for response in responses {
            test_serialization_json(response);
        }
    }
}