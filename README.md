# Rust Key-Value Store

A simple Redis-like key-value store implemented in Rust. This project implements a basic TCP server that handles GET, SET, and DEL commands similar to Redis.

## Features

- In-memory key-value storage
- Concurrent access using DashMap
- Support for basic Redis-like commands:
  - SET key value
  - GET key
  - DEL key

## Building and Running

To build and run the project:

```bash
# Build the project
cargo build --release

# Run the server
cargo run --release
```

The server will start listening on `127.0.0.1:6379`.

## Usage

You can connect to the server using netcat (nc) or telnet:

```bash
nc 127.0.0.1 6379
```

Example commands:

```
SET mykey value123
GET mykey
DEL mykey
```

## Implementation Details

- Uses async/await with Tokio for handling concurrent connections
- Thread-safe storage using DashMap
- Simple text-based protocol similar to Redis