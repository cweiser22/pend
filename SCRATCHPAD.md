# Rust Scheduler

## Core Architecture

### Daemon main loop
1. Set up logging and read in envvars
2. Load all task definitions from tasks dir
3. Store all task definitions in memory
4. Start main loop
   1. Calculate current time
   2. Compare all task definitions to that current time
   3. Fire off every task that matches
   4. Sleep 60 seconds~~

### New Editing System
1. Require every task have a user supplied, filename-sage unique name.
2. Add embedded editing functionality with `vi` like in Git.
   1. TODO: allow editor binary path to be configurable so users can use whatever editor 
   they prefer
3. 