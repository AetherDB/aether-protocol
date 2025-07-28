# aether-protocol

This crate defines the official, stable network protocol for communicating with an AetherDB instance. It contains all request and response data structures, serialized using `bincode` for maximum performance.

## Features
-   Database Management
-   Collection Management
-   Index Management
-   Record Operations (CRUD)
-   Querying & Relational


## Usage

Add `aether-protocol` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
aether-protocol = "0.5.0"
```

```rust
use aether_protocol::{Request, Response};

let request = Request::CreateDatabase { db_name: "testdb".to_string() };
let response = Response::Success;
```

## License

Apache-2.0


