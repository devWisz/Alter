#Alter
A lightweight command-line file compression tool built with Rust and the `flate2` crate.

This project compresses files into `.gz` format while displaying useful compression statistics such as original size, compressed size, and execution time.
The application is simple, fast, and designed to demonstrate practical file handling, buffered I/O, and compression techniques in Rust.

---
# Features

- Compress files using Gzip compression
- Fast buffered file reading
- Displays original and compressed file sizes
- Shows total compression time
- Interactive terminal prompt for repeated usage


# Installation

## 1. Install Rust

Download and install Rust from:

```bash
https://www.rust-lang.org/tools/install
```

Verify installation:

```bash
rustc --version
cargo --version
```

---

## 2. Clone the Project

```bash
git clone <repository-url>
cd <project-folder>
```

---

## 3. Add Dependency

Add the following dependency inside `Cargo.toml`:

```toml
[dependencies]
flate2 = "1"
```

---

## 4. Build the Project

```bash
cargo build --release
```

---

# Running the Program

```bash
cargo run -- <source_file> <target_file.gz>
```

Example:

```bash
cargo run -- notes.txt notes.txt.gz
```

---

# Example Output

```bash
Source len: 204800
Target len: 52310
Elapsed:    12.4ms

Do you want to compress one more file?(y/n):
```

# License

This project is open for learning, modification, and experimentation.