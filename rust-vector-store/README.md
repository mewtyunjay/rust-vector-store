# Rust Vector Store

🚧 **Work In Progress** 🚧

A vector database implementation built from first principles in Rust.

## Overview

This component implements:
- Vector storage and indexing
- Similarity search algorithms
- Efficient data structures for high-dimensional vectors
- API for vector operations (insert, search, delete)

## Files

- `src/main.rs` - Main entry point and CLI interface
- `Cargo.toml` - Project dependencies and configuration

## Setup

```bash
# Build the project
cargo build

# Run the vector store
cargo run

# Run tests
cargo test
```

## Features (Planned)

- [ ] In-memory vector storage
- [ ] Cosine similarity search
- [ ] Approximate nearest neighbor search (ANN)
- [ ] Persistent storage to disk
- [ ] HTTP API for vector operations
- [ ] Integration with Python embeddings
- [ ] Performance benchmarking

## Architecture Goals

- Fast vector similarity search
- Memory-efficient storage
- Concurrent access support
- Modular design for different indexing strategies

## Dependencies

See `Cargo.toml` for the current dependency list. 