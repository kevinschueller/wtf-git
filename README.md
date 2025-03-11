<div align="center">

# WTF Git

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

_A command-line tool that explains Git repositories in plain language using AI._

</div>

## 📖 Overview

**WTF Git** is a developer tool that helps you understand Git repositories by providing:

- 📝 Plain language descriptions of what a project is about
- 🔍 Simple explanations of recent commits
- 💡 Detailed analysis of code changes in human-readable terms

Perfect for when you join a new project, return to old code, or just need to understand "WTF is going on with this Git repo?"

## ✨ Features

- **Project Description**: Generates a concise summary of what the project is about based on the README
- **Commit Analysis**: Explains recent commits in plain language
- **Code Change Insights**: Provides human-readable explanations of what changed and why
- **Customizable Analysis**: Control the depth and scope of the analysis

## 🚀 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- An [OpenAI API key](https://platform.openai.com/account/api-keys)

### From Source

```bash
# Clone the repository
git clone https://github.com/kevinschueller/wtf-git.git

# Navigate to the project directory
cd wtf-git

# Install the tool
cargo install --path .
```

## 🔧 Configuration

Create a `.env` file in your project directory with your OpenAI API key:

```
OPENAI_API_KEY=your_api_key_here
```

## 📋 Usage

```bash
# Analyze the current directory
wtf .

# Analyze a specific repository
wtf /path/to/repository

# Analyze with a specific number of commits
wtf . --num-commits 10
```

## 🧩 Dependencies

- `git2`: Git repository interaction
- `clap`: Command-line argument parsing
- `reqwest`: HTTP client for API requests
- `serde`: Serialization/deserialization
- `tokio`: Asynchronous runtime
- `anyhow`: Error handling
- `dotenv`: Environment variable management

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add some amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

- OpenAI for providing the API that powers the natural language explanations
- The Rust community for the excellent libraries and tools
