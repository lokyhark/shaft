# Shaft

[<img alt="github" src="https://img.shields.io/badge/github-lokyhark/shaft-2e2459?style=for-the-badge&logo=github">](https://github.com/lokyhark/shaft)
[<img alt="crates.io" src="https://img.shields.io/crates/v/shaft.svg?style=for-the-badge&color=ffc832&logo=rust">](https://crates.io/crates/shaft)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-shaft-a72145?style=for-the-badge&logo=docs.rs">](https://docs.rs/shaft)
[<img alt="github actions" src="https://img.shields.io/github/actions/workflow/status/lokyhark/shaft/default.yaml?branch=default&color=0b7261&label=Actions&logo=GitHub%20Actions&logoColor=white&style=for-the-badge">](https://github.com/lokyhark/shaft/actions/workflows/default.yaml)

A minimal and straightforward binary serde implementation.

# Usage

```console
$ cargo add serde shaft
```

# Examples

```rust
// Bring std Error type into scope.
use std::error::Error;
// Bring std filesystem module into scope.
use std::fs;
// Bring serde Serialize/Deserialize derivable traits into scope.
use serde::{Deserialize, Serialize};

// Define custom struct.
#[derive(Deserialize, Serialize)]
struct MyStruct {
    name: String,
    score: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Create value to serialize.
    let value = MyStruct {
        name: "Ferris".to_owned(),
        score: 42,
    };

    // Serialize value to bytes.
    let bytes = shaft::to_bytes(&value)?;

    // Export value to file.
    fs::write("value.bin", bytes)?;

    // Retrieve bytes from file.
    let bytes = fs::read("value.bin")?;

    // Deserialize value from bytes.
    let value: MyStruct = shaft::from_bytes(&bytes)?;

    // Check struct fields.
    assert_eq!(value.name, "Ferris");
    assert_eq!(value.score, 42);

    // Clean file.
    fs::remove_file("value.bin")?;

    Ok(())
}
```

# License

MIT