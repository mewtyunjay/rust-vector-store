# Python Embeddings

🚧 **Work In Progress** 🚧

Python implementation for generating and working with text embeddings from first principles.

## Overview

This component implements:
- Text preprocessing and tokenization
- Embedding generation using various models
- Embedding manipulation and analysis
- Integration with the Rust vector store

## Files

- `main.py` - Main entry point and CLI interface
- `embed.py` - Core embedding functionality
- `pyproject.toml` - Project dependencies and configuration

## Setup

This project uses `uv` for dependency management:

```bash
# Install dependencies
uv sync

# Run the main script
uv run python main.py
```

## Features (Planned)

- [ ] Support for multiple embedding models
- [ ] Batch processing of text data
- [ ] Embedding similarity calculations
- [ ] Export embeddings to various formats
- [ ] Integration with Rust vector store

## Dependencies

See `pyproject.toml` for the current dependency list.
