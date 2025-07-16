use std::io::{self, Write};
use std::process::Command;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let py_path = PathBuf::from("../python-embeddings/embed.py");
    if !py_path.exists() {
        panic!("Embedding file not found at {:?}", py_path);
    }
    
    println!("Enter text to embed:");
    print!("> ");
    io::stdout().flush().unwrap();

    // user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    let python_code = format!(
        r#"
import sys
sys.path.insert(0, '../python-embeddings')
from embed import embed
import json

text = "{}"
result = embed(text)
print(json.dumps(result))
"#,
        input.replace('"', r#"\""#)
    );

    let python_executable = "../python-embeddings/.venv/bin/python";
    
    let output = Command::new(python_executable)
        .arg("-c")
        .arg(&python_code)
        .output()?;

    if !output.status.success() {
        eprintln!("Python script failed:");
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        return Err("Python execution failed".into());
    }

    let output_str = String::from_utf8(output.stdout)?;
    let embedding: Vec<f32> = serde_json::from_str(output_str.trim())?;

    println!("Embedding: {:?}", &embedding[..8.min(embedding.len())]);
    println!("Dimensionality: {}", embedding.len());
    
    Ok(())
}
