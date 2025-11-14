# AI Agent Framework

An educational Rust framework for building autonomous AI agents with memory, planning, tool execution, and safety guardrails.

## Overview

This framework demonstrates how to build production-quality AI agents in Rust, with clear architectural boundaries and incremental complexity. It's designed as a learning resource with a modular workspace structure where each component can be understood independently.

## Architecture

The framework is organized into three layers:

### Foundation Layer
- **core** - Fundamental types (Message, Role, AgentError) and error handling
- **config** - Configuration management from files and environment variables
- **communication** - HTTP client utilities with retry logic

### Capability Layer
- **llm** - LLM provider interfaces (OpenAI, Anthropic)
- **memory** - Conversation storage with token-aware context management
- **tools** - Tool system with registry and example implementations

### Intelligence Layer
- **planner** - Task decomposition using LLM reasoning
- **executor** - Step-by-step execution of plans
- **guardrails** - Safety validation before execution
- **rules** - Behavior customization through prompt modification
- **cli** - Command-line interface (REPL and single-turn modes)
- **examples** - Example agents demonstrating framework capabilities

## Project Structure

```
ai-agent-framework/
├── Cargo.toml              # Workspace manifest
├── README.md               # This file
├── core/                   # Fundamental traits and types
├── config/                 # Configuration management
├── communication/          # HTTP client and API utilities
├── llm/                    # LLM provider implementations
├── memory/                 # Conversation storage
├── tools/                  # Tool trait and implementations
├── planner/                # Task decomposition
├── executor/               # Step execution
├── guardrails/             # Safety validation
├── rules/                  # Behavior customization
├── cli/                    # Command-line interface
└── examples/               # Example agents
```

## Getting Started

### Prerequisites

- Rust 1.70 or later
- An API key for OpenAI or Anthropic

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd ai-agent-framework
```

2. Set up your API key:
```bash
export OPENAI_API_KEY="your-api-key-here"
# or
export ANTHROPIC_API_KEY="your-api-key-here"
```

3. Build the project:
```bash
cargo build
```

4. Run tests:
```bash
cargo test --workspace
```

### Running Examples

The framework includes three example agents:

1. **Simple Chatbot** - Basic conversation with no tools
```bash
cargo run --example chatbot
```

2. **Research Assistant** - Uses web search and file reading tools
```bash
cargo run --example research
```

3. **File Manager** - File operations with path guardrails
```bash
cargo run --example file_manager
```

## Component Overview

### Core Crate
Defines fundamental types used throughout the framework:
- `Message` - Represents conversation turns with role, content, and timestamp
- `Role` - Enum for System, User, and Assistant roles
- `AgentError` - Common error type with structured error information
- `Result<T>` - Type alias for convenient error handling

### Config Crate
Manages framework configuration:
- Load settings from YAML/TOML files
- Override with environment variables
- Validate required fields (API keys, model names)
- Provide sensible defaults

### LLM Crate
Unified interface for multiple LLM providers:
- `LLMProvider` trait for consistent API
- OpenAI implementation (GPT-3.5, GPT-4)
- Anthropic implementation (Claude)
- Automatic retry with exponential backoff

### Memory Crate
Conversation storage with token awareness:
- Store messages with timestamps
- Retrieve recent conversation history
- Token-aware context management
- Support for different storage backends

### Tools Crate
Extensible tool system:
- `Tool` trait for custom tools
- `ToolRegistry` for tool management
- Example tools: Calculator, FileReader, WebSearchStub
- JSON Schema parameter validation

### Planner Crate
LLM-based task decomposition:
- Break down user goals into executable steps
- Generate structured plans with reasoning
- Validate tool availability
- Support for multi-step workflows

### Executor Crate
Step-by-step plan execution:
- Execute plans sequentially
- Invoke tools with parameters
- Collect and store results
- Handle errors gracefully

### Guardrails Crate
Safety validation system:
- `Guardrail` trait for custom safety rules
- File path restrictions
- API rate limiting
- Prevent unauthorized actions

### Rules Crate
Behavior customization:
- `Rule` trait for modifying agent behavior
- Priority-based rule ordering
- Prompt modification
- Response constraints

### CLI Crate
Command-line interface:
- Interactive REPL mode with history
- Single-turn query mode
- Colored output
- Conversation history display

## Development

### Building

```bash
# Build all crates
cargo build --workspace

# Build in release mode
cargo build --release --workspace
```

### Testing

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p core

# Run integration tests
cargo test --test agent_flow
```

### Documentation

```bash
# Generate and open documentation
cargo doc --open --no-deps

# Check documentation
cargo doc --workspace --no-deps
```

### Code Quality

```bash
# Run clippy for linting
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:
- Adding new LLM providers
- Creating custom tools
- Implementing guardrails and rules
- Code style and testing requirements

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Resources

### Rust Learning
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Async Rust Book](https://rust-lang.github.io/async-book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### AI Agent Architecture
- [ReAct Paper](https://arxiv.org/abs/2210.03629) - Reasoning and Acting in Language Models
- [Chain-of-Thought Paper](https://arxiv.org/abs/2201.11903) - Prompting strategies
- [OpenAI Function Calling](https://platform.openai.com/docs/guides/function-calling)

## Acknowledgments

This framework was built as an educational resource to demonstrate production-quality AI agent architecture in Rust. It prioritizes clarity and learning over performance optimization.
