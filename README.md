# RustyKV 🦀

**A Lightweight, LSM-Tree inspired Key-Value Store built in Rust.**

RustyKV is an educational yet robust in-memory database with disk persistence. It implements the core mechanics of a **Log-Structured Merge-Tree (LSM)** architecture, the same engine powering industry giants like RocksDB and Cassandra.

This project was built from scratch to explore Rust's memory safety (Borrow Checker), File I/O operations, and Enum-based command parsing.

---

## ✨ Features

* **Interactive REPL:** A fast, continuous command-line interface to interact with the database in real-time.
* **MemTable (RAM Storage):** Immediate `O(1)` reads and writes using Rust's native `HashMap`.
* **SSTable Persistence:** Automatic flushing to disk when the MemTable reaches a specific threshold, ensuring fast sequential writes.
* **Cascading Reads:** Unified read logic that transparently checks the fast RAM cache first, and falls back to scanning the disk if the key is not in memory.
* **Graceful Shutdown:** Safely flushes all remaining in-memory data to the disk upon exiting, guaranteeing zero data loss.

---

## 🚀 Getting Started

### Prerequisites

Make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your system.

### Installation & Execution

Clone the repository and run the project directly using Cargo:

```bash
git clone https://github.com/YOUR_USERNAME/rusty-kv.git
cd rusty-kv
cargo run
```

---

## 💻 Usage

Once the REPL starts, you can interact with the database using the following commands:

```text
--- Rust-LSM Started ---
Supported commands: SET <key> <value>, GET <key>, QUIT

> SET user_1 admin
key and value setted!
> SET server_port 8080
key and value setted!
> GET user_1
admin
> GET unknown_key
(nil)
> QUIT
Shutting down database... Goodbye!
```

---

## 🏗️ Architecture

The project is cleanly divided into modular components:

* **parser.rs:** Safely sanitizes user input and converts raw strings into strongly-typed Command Enums.
* **memtable.rs:** Manages the internal state (the HashMap) and handles File System I/O (BufReader and File) to read and write the `sstable.txt`.
* **main.rs:** Acts as the orchestrator, running the REPL loop, tracking memory capacity, and triggering the disk flushes.

---

## 🗺️ Roadmap / Future Enhancements

* [ ] **Tombstones:** Implement the DELETE command by writing tombstone markers to the SSTable.
* [ ] **Compaction:** Create a background process to merge multiple SSTable files and clean up overwritten/deleted keys.
* [ ] **Bloom Filters:** Optimize disk reads by checking a Bloom Filter before scanning the file system.

---

Built with 🦀 and memory safety in mind.
