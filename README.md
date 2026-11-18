<p align="center">
  <a href="README.md"><b>English</b></a>
  &nbsp;·&nbsp;
  <a href="README.fr.md">Français</a>
  &nbsp;·&nbsp;
  <a href="README.es.md">Español</a>
</p>

# 🦀 Rust LLM Manager

![Rust](https://img.shields.io/badge/Rust-2021-eddede?style=for-the-badge&logo=rust)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Version](https://img.shields.io/badge/Version-0.1.0-blue?style=for-the-badge)

> A modern, interactive CLI tool for managing and testing local Large Language Models (LLMs) using Ollama.

## 📋 Table of Contents

- [✨ Features](#-features)
- [🛠️ Prerequisites](#️-prerequisites)
- [📦 Installation](#-installation)
- [🚀 Usage](#-usage)
- [🎯 Menu Options](#-menu-options)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)

## ✨ Features

- **🤖 Install Popular Models**: One-click installation for Llama 3, Qwen 2, and Mistral
- **🔗 GitHub Integration**: Clone LLMs directly from GitHub repositories
- **📋 Model Management**: List all installed Ollama models
- **💬 Interactive Chat**: Chat directly with any installed model
- **🗑️ Uninstall Models**: Remove unwanted models or cloned repositories
- **🎨 Beautiful CLI**: Colorful, user-friendly interactive menu interface

## 🛠️ Prerequisites

Before using this project, ensure you have the following installed:

### Required Dependencies

| Tool | Version | Description |
|------|---------|-------------|
| **Rust** | 1.56+ | Rust programming language |
| **Cargo** | Latest | Rust package manager |
| **Ollama** | Latest | Local LLM runtime |
| **Git** | Latest | For cloning repositories |

### Installing Prerequisites

#### Rust & Cargo
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

#### Ollama
```bash
# Install Ollama (Linux/macOS)
curl -fsSL https://ollama.com/install.sh | sh

# Verify installation
ollama --version
```

#### Git
```bash
# Ubuntu/Debian
sudo apt-get update && sudo apt-get install git

# macOS
brew install git

# Verify installation
git --version
```

## 📦 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/llm_tools.git
cd llm_tools
```

### 2. Build the Project

```bash
# Build in debug mode
cargo build

# Build in release mode (recommended for better performance)
cargo build --release
```

### 3. Run the Application

```bash
# Debug mode
cargo run

# Release mode
cargo run --release
```

## 🚀 Usage

When you launch the application, you'll see an interactive menu:

```
--- RUST LLM MANAGER PRO ---
Prêt à gérer et tester tes modèles locaux ?

┌─ Main Menu ─────────────────────────────┐
│  1. Installer un modèle (Llama 3, Qwen 2, Mistral)  │
│  2. Cloner un modèle depuis GitHub                    │
│  3. Lister mes modèles installés (Ollama)            │
│  4. 💬 Lancer un Chat avec un modèle                  │
│  5. Désinstaller / Supprimer un modèle                │
│  6. Quitter                                             │
└──────────────────────────────────────────┘
```

## 🎯 Menu Options

### Option 1: Install a Model
Installs popular open-source models:
- **Llama 3** - Meta's latest language model
- **Qwen 2** - Alibaba's multilingual model
- **Mistral** - Efficient open-source model

### Option 2: Clone from GitHub
Clones LLM repositories directly from any GitHub URL.

### Option 3: List Installed Models
Displays all models currently installed in your Ollama instance.

### Option 4: Chat with a Model
Starts an interactive chat session with your chosen model. Type `/exit` to quit.

### Option 5: Uninstall Model
Two sub-options:
- Remove an Ollama model
- Delete a cloned GitHub repository folder

### Option 6: Exit
Exits the application gracefully.

## 🤝 Contributing

We welcome contributions! Here's how you can help:

### Ways to Contribute

- 🐛 **Report Bugs**: Create an issue with detailed reproduction steps
- 💡 **Suggest Features**: Open an issue to discuss new functionality
- 📝 **Improve Documentation**: Submit PRs to enhance README or code comments
- 🔧 **Code Contributions**: Fix bugs, add features, or improve performance

### Development Setup

```bash
# Fork the repository
# Clone your fork
git clone https://github.com/YOUR_USERNAME/llm_tools.git

# Create a feature branch
git checkout -b feature/your-feature-name

# Make your changes
# ... edit code ...

# Commit your changes
git commit -m "Add your feature description"

# Push to your fork
git push origin feature/your-feature-name

# Create a Pull Request
```

### Code Style

- Follow standard Rust conventions
- Use meaningful variable and function names
- Add comments for complex logic
- Ensure code compiles without warnings: `cargo clippy`

### Testing

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Check for issues
cargo clippy
```

### Pull Request Guidelines

1. **Describe your changes** - Explain what you fixed or added
2. **Reference related issues** - Link to any related issues
3. **Include tests** - Add tests for new functionality
4. **Update documentation** - Update README if needed
5. **Keep it focused** - One feature or fix per PR

## 📁 Project Structure

```
llm_tools/
├── .gitignore           # Git ignore rules
├── Cargo.lock           # Dependency lock file
├── Cargo.toml           # Project manifest
├── README.md            # This file
└── src/
    └── main.rs          # Main application code
```

## 🧩 Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `dialoguer` | 0.11 | Interactive CLI menus |
| `console` | 0.15 | Terminal colors & styling |
| `tokio` | 1.x | Async runtime |
| `indicatif` | 0.17 | Progress bars (optional) |

## ⚠️ Troubleshooting

### Ollama Not Found
```bash
# Ensure Ollama is installed and in your PATH
export PATH="$PATH:/usr/local/bin"
ollama --version
```

### Permission Denied
```bash
# Check file permissions
ls -la

# Fix if needed
chmod +x your-script.sh
```

### Compilation Errors
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
cargo clean
cargo build
```

## 📄 License

This project is proprietary and protected by copyright laws. See the [LICENSE](LICENSE) file for full terms.

**Summary:**
- ✅ Personal, non-commercial use allowed
- ❌ Commercial use prohibited without written permission
- 💰 Only the owner can commercialize this software

---

<div align="center">

**Made with ❤️ in Rust**

*Stars are welcome!* ⭐

</div>

