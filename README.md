# RustyKV 🦀

**A Lightweight, LSM-Tree Inspired Key-Value Store Built in Rust.**

RustyKV is an educational key-value store with in-memory storage and disk persistence. It implements the core mechanics of a **Log-Structured Merge-Tree (LSM)** architecture — the same engine powering systems like RocksDB and Apache Cassandra.

Built from scratch to explore Rust's ownership model, File I/O, and enum-based command parsing.

---

## ✨ Features

- **Interactive REPL** — A continuous command-line interface for real-time interaction with the database.
- **MemTable (In-Memory Storage)** — Fast `O(1)` reads and writes using Rust's `HashMap`.
- **Append-Only SSTable Persistence** — When the MemTable reaches its capacity threshold, entries are flushed to an append-only `sstable.txt` file on disk, preserving write history.
- **Cascading Reads** — On a cache miss, the engine scans the SSTable and returns the most recent value for the requested key, correctly handling multiple entries from successive flushes.
- **Graceful Shutdown** — On `QUIT`, any remaining in-memory data is safely flushed to disk before exit.

---

## ⚠️ Current Limitations

This is an educational project. The following known limitations reflect the early stage of the implementation:

- **Sequential disk scan** — Disk reads perform a full linear scan of the SSTable. A production LSM-tree would use sorted, indexed SSTables to enable binary search. This is tracked in the roadmap.
- **Single SSTable file** — All flushed data is appended to a single file. Compaction (merging and deduplication) is not yet implemented.
- **No DELETE support** — Tombstone markers are not yet implemented.

---

## 🚀 Getting Started

### Prerequisites

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

### Installation & Execution

```bash
git clone https://github.com/mattiabandini1/RustyKV.git
cd rusty-kv
cargo run
```

### Running Tests

```bash
cargo test
```

---

## 💻 Usage

```text
--- Rust-LSM Started ---
Supported commands: SET <key> <value>, GET <key>, QUIT

> SET user_1 admin
Key and value successfully set!
> SET server_port 8080
Key and value successfully set!
> GET user_1
admin
> GET unknown_key
Key not found!
> QUIT
Database shutdown in progress...
```

---

## 🏗️ Architecture

The project is divided into three focused modules:

- **`parser.rs`** — Parses raw user input into strongly-typed `Command` enums (`Get`, `Set`, `Unknown`). Fully unit-tested.
- **`memtable.rs`** — Manages the in-memory `HashMap` and handles all File I/O: appending to the SSTable on flush and scanning it on disk reads.
- **`main.rs`** — Orchestrates the REPL loop, tracks MemTable size, and triggers flushes when the threshold is reached.

---

## 🗺️ Roadmap

- [ ] **Tombstones** — Implement `DELETE` via tombstone markers in the SSTable.
- [ ] **Compaction** — Background process to merge SSTable files and remove stale/deleted entries.
- [ ] **Bloom Filters** — Probabilistic structure to avoid unnecessary disk scans on missing keys.
- [ ] **Sorted SSTables** — Sort keys on flush to enable binary search and eliminate linear scans.

---

Built with 🦀 and memory safety in mind.
