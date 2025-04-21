# 🕒 `pend`: A Rust Task Scheduler

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight, efficient cron-like task scheduler written in Rust. `pend` allows you to define tasks via simple JSON files, with each task consisting of an executable and its arguments.

## ✨ Features

- 🔄 Continuous operation with 60-second evaluation intervals
- 📁 JSON-based task definitions
- 👀 File watching for automatic task reloading
- 🚀 Simple and efficient execution model

## 🏗️ Architecture

### 🔍 Main Executable

The core scheduler runs in a persistent loop, checking every 60 seconds for tasks that need to be executed. It:

- Loads task definitions from a configured directory
- Maintains a file watcher to detect changes to task definitions
- Automatically refreshes the task list when changes are detected
- Executes tasks as scheduled

### 🛠️ Utility Tools

#### Create Task

A companion utility to easily define new scheduled tasks.

##### Usage (Development)

```bash
cargo run --bin create_task -- <cron_expr> <executable> [<args>...]
```

## 🚀 Getting Started

_todo..._

## 📚 Documentation

_todo..._

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.