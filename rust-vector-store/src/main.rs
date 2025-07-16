use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyModule};
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> PyResult<()> {
    let py_path =
        PathBuf::from("/Users/mrityunjay/Code/2025/from-scratch/python-embeddings/embed.py");
    if !py_path.exists() {
        panic!("Embedding file not found at {:?}", py_path);
    }
    println!("Enter text to embed:");
    print!("> ");

    io::stdout().flush().unwrap();
}
