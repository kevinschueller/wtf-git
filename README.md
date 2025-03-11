# WTF Git

A command-line tool that explains Git repositories in plain language using AI.

## Overview

WTF Git is a developer tool that helps you understand Git repositories by providing:

- Plain language descriptions of what a project is about
- Simple explanations of recent commits
- Detailed analysis of code changes in human-readable terms

Perfect for when you join a new project, return to old code, or just need to understand "WTF is going on with this Git repo?"

## Installation

```bash
# Clone the repository
git clone https://github.com/kevinschueller/wtf-git.git

# Navigate to the project directory
cd wtf-git

# Install the tool
cargo install --path .
```

## Usage

# Analyze the current directory

wtf .

# Analyze a specific repository

wtf /path/to/repository

# Analyze with a specific number of commits

wtf . --num-commits 10

## Features

- Project Description : Generates a concise summary of what the project is about based on the README
- Commit Analysis : Explains recent commits in plain language
- Code Change Insights : Provides human-readable explanations of what changed and why

## Requirements

- Rust (latest stable version)
- An OpenAI API key

## Configuration

Create a .env file in your project directory with your OpenAI API key:

OPENAI_API_KEY=your_api_key_here

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
