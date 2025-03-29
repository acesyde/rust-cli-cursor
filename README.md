# Rust CLI Tool

A simple command-line interface (CLI) tool built with Rust that provides two main functionalities:
- Greeting users with a personalized hello message
- Retrieving your public IP address

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

Clone this repository:
```bash
git clone <repository-url>
cd rust-cli
```

Build the project:
```bash
cargo build --release
```

The binary will be available in `target/release/rust-cli`

## Usage

### Hello Command
To greet someone:
```bash
cargo run -- hello "John Doe"
```

### IP Command
To get your public IP address:
```bash
cargo run -- ip
```

### Help
To see all available commands and their descriptions:
```bash
cargo run -- --help
```

## Dependencies

- `clap` (v4.4) - Command line argument parsing
- `reqwest` (v0.11) - HTTP client for making API requests
- `tokio` (v1.0) - Async runtime
- `serde_json` (v1.0) - JSON parsing

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests. 