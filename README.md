# rgrep

> A simple pattern matching CLI tool written in Rust — inspired by `grep`, powered by learning.

![Rust](https://img.shields.io/badge/Made_with-Rust-orange?logo=rust)
![Status](https://img.shields.io/badge/Status-WIP-yellow)
![License](https://img.shields.io/badge/license-MIT-blue)

---

## 📦 What is `rgrep`?

`rgrep` is a basic command-line tool that searches for a given string pattern in one or more files — built from scratch in Rust 🦀.

This project is part of my journey into systems programming and low-level concepts using Rust. It aims to replicate the core behavior of Unix `grep` — reading lines, matching patterns, and optionally highlighting or counting matches.

---

## ✨ Features

- ✅ Match and print lines containing a given string pattern
- ✅ Highlight matched pattern in red (using ANSI escape codes)
- ✅ Support for reading from **single or multiple files**
- ✅ Show line numbers
- ✅ Basic error handling (e.g., missing files, no arguments)

---

## 🚀 Usage

### 🔧 Run the program

```bash
rgrep -- <pattern> <file1.txt> <file2.txt> ...
```

---

## 🧪 Example

```bash
rgrep -- Rust notes.txt log.txt
```

- This will:

  - Print each line from notes.txt and log.txt that contains "Rust"
  - Highlight "Rust" in red
  - Show the line number
  - Show total number of matches

---

📦 Binaries

Pre-built binaries are available for:

- ✅ Windows: rgrep.exe

- ✅ Linux: rgrep

You can download them from the release section.

Make sure to run them from the terminal:

```bash
<!-- Windows -->
./rgrep.exe "pattern" file.txt

<!-- Linux -->
./rgrep "pattern" file.txt
```

---

🛠️ Building

- To build the executable:

```bash
git clone https://github.com/rajanchoksi79/rgrep.git
cd rgrep
cargo build --release
```

- This will create the binary at:

```bash
./target/release/rgrep
```

---

## ❌ Contribution

- This project is currently not accepting external contributions, as it's a personal learning project. Once stable, contributions may be opened up.

---

## 📜 License

- Licensed under the MIT License.
- See LICENSE for details.

---

## 👋 Author

- Rajan Choksi — Full Stack Developer diving into Rust & systems programming.

- 📢 Say hi on Twitter - https://x.com/Rajanchoksi_79 or check out more of my work on GitHub - https://github.com/rajanchoksi79

---

Made with ❤️ and Rust.

---
