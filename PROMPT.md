# Mission Briefing: The AetherDB Ecosystem
You are an expert Rust software architect. Your task is to generate one of the following Rust crates. You must adhere strictly to the provided specifications, as your work will be integrated with other components being developed in parallel.

- **Universal Engineering Commandments (Apply to ALL projects):**
- Production-Ready From Day One: The code must be clean, robust, well-documented, and free of placeholders or unimplemented! macros.
- **Integrity is Non-Negotiable:** The system must be designed for data safety and consistency.
- **Architecture Before Code:** The implementation must strictly follow the provided modular architecture.
- **Modularity and Separation of Concerns (SOLID/KISS):** Each module must have a single, clear responsibility.
- **Performance is the Goal:** Every design choice must prioritize speed and low overhead.

## Project: aether-protocol
**Type:** Public, Open-Source Library (publish = true)
**License:** Apache-2.0


## Primary Objective:
Create a self-contained Rust library named aether-protocol. This crate's sole purpose is to define the data structures for the network communication protocol used by AetherDB. It is the "universal translator" that allows clients and servers to communicate. It must have zero dependencies on any other AetherDB crates.

## File Structure:

```
src/
├── lib.rs
├── request.rs
├── response.rs
└── types.rs
```

## Core Requirements:
- **Dependencies**: Use serde for serialization and bincode for the binary format. Use serde_json for the Value type.
- **src/types.rs**: Define all shared data structures: Record, RecordSet, Filter, QueryOptions, Direction, and DbStats. These must be serializable.
- **src/request.rs**: Define a single, top-level Request enum that contains a variant for every possible command a client can send to the server, as defined in the AetherDB V7 API blueprint.
- **src/response.rs**: Define a single, top-level Response enum that contains a variant for every possible reply the server can send back.
- **src/lib.rs**: Declare all modules and re-export the primary public types for convenient use by other crates.
- **Testing**: The crate must include a comprehensive suite of unit tests that prove every Request and Response variant can be successfully serialized and deserialized without data loss.