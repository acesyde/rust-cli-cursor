# 🦀 Rust CLI Tool

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A modern command-line interface (CLI) tool built with Rust that demonstrates clean architecture and best practices in CLI development.

## ✨ Features

- 👋 **Hello Command**: Greet users with a personalized message
- 🌐 **IP Command**: Retrieve your public IP address using the ipify API
- 🎯 Clean and modular architecture
- ✅ Comprehensive test coverage
- 📝 Well-documented code

## 🚀 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd rust-cli
```

2. Build the project:
```bash
cargo build --release
```

The binary will be available at `target/release/rust-cli`

## 📖 Usage

### Hello Command
Greet someone with a personalized message:
```bash
rust-cli hello "John Doe"
```

### IP Command
Get your public IP address:
```bash
rust-cli ip
```

### Help
View all available commands and their descriptions:
```bash
rust-cli --help
```

## 🏗️ Project Structure

```
rust-cli/
├── src/
│   ├── commands/     # Command implementations
│   ├── models/       # Data structures and types
│   ├── services/     # External service integrations
│   └── main.rs       # Application entry point
├── tests/            # Integration tests
├── .editorconfig     # Editor configuration
├── .gitignore       # Git ignore rules
├── Cargo.toml       # Project dependencies
└── README.md        # Project documentation
```

## 🔧 Development

### Running Tests

Run the test suite:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

### Code Style

This project uses:
- EditorConfig for consistent coding style
- Rust 2021 edition idioms
- Standard Rust formatting (`cargo fmt`)
- Clippy for linting (`cargo clippy`)

## 📦 Dependencies

- `clap` (v4.4) - Modern command line argument parser
- `reqwest` (v0.12) - Powerful HTTP client
- `tokio` (v1.0) - Asynchronous runtime
- `serde` (v1.0) - Serialization framework
- `thiserror` (v1.0) - Error handling
- `mockito` (v1.2) - HTTP mocking for tests

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [ipify](https://www.ipify.org/) for providing the IP lookup service
- The Rust community for excellent documentation and support 