# 🦀 Git Repository Analyzer — Rust CLI

A command-line tool written in **Rust** that analyzes Git repositories and generates statistics: commit history, author contributions, file change tracking, and output to CSV/JSON.

> Academic project — Systems Programming module, L3 Computer Science @ UVSQ (group project)

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Git](https://img.shields.io/badge/Git-F05032?style=flat&logo=git&logoColor=white)
![License](https://img.shields.io/badge/License-MIT-green)

---

## Features

- Scans a local directory for Git repositories
- Extracts commit metadata: author, date, message, files changed
- Generates per-author statistics (commit count, lines added/removed)
- Outputs results to CSV and JSON formats
- Handles multiple repositories in a single scan

## Tech stack

| Component | Technology |
|-----------|-----------|
| Language | Rust |
| Git interaction | libgit2 / git2-rs crate |
| Output formats | CSV, JSON |
| Build system | Cargo |

## Getting started

```bash
git clone https://github.com/AmZzPYJS/Analyse-Rust-Depot-Git.git
cd Analyse-Rust-Depot-Git
cargo build --release
cargo run -- /path/to/directory
```

## What I learned

- Writing idiomatic Rust: ownership, borrowing, lifetimes, error handling with `Result`
- Using Rust crates (git2, serde, csv) and managing dependencies with Cargo
- Parsing and processing Git objects programmatically
- Collaborating on a shared codebase with Git (branching, merging, pull requests)
- Structuring a Rust project with modules and clean separation of concerns

## License

MIT
