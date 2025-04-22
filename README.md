# 🕒 `pend`: A Rust Task Scheduler

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight, efficient cron-like task scheduler written in Rust. `pend` allows you to define tasks via simple JSON files, with each task consisting of an executable and its arguments.

## ✨ Features

- 🔄 Continuous operation with 60-second evaluation intervals
- 📁 JSON-based task definitions
- 👀 File watching for automatic task reloading
- ⚡ Launches tasks in parallel with Tokio's `multi_thread` runtime

## 🏗️ Project Structure

The project is organized into three crates:

- 📦 `pend-core` - Core library containing shared functionality
- 🖥️ `pend-daemon` - Background service to run tasks
- 🔧 `pend-cli` - Command-line interface for managing tasks

## 🛠️ Components

### 🔍 Daemon (`pend-daemon`)

The daemon runs as a persistent service, checking every 60 seconds for tasks that need to be executed. It:

- Loads task definitions from a configured directory
- Maintains a file watcher to detect changes to task definitions
- Automatically refreshes the task list when changes are detected
- Executes tasks as scheduled

### 💻 CLI (`pend-cli`)

A unified command-line interface that provides all management functionality:

- TODO

#### Usage

```bash
# Create a new task
pend-cli create-task -n <task_name> | --name <task_name> [--edit]

# Edit an existing task
pend-cli edit-task -n <task_name> | --name <task_name>
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.