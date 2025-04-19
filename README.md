# `pend`: a Rust Task Scheduler

This project is a simple cron-like task scheduler written in Rust. It allows users to
define, via json files, basic commands composed of an executable and `n` arguments.

## Main executable
The main executable runs in a 24/7 loop, evaluating every 60 seconds if any tasks need
to be fired off. The tasks are loaded from a pre-defined directory of JSON task definitions,
and a file watcher that runs in the background will refresh the task definitions if any changes
to the task directory are detected.

## Secondary executables

### Create Task

#### Usage (dev)
`cargo run --bin create_task -- <cron_expr> <executable> [<args>...]`

