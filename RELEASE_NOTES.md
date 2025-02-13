# Password Generator CLI v0.1.0

Initial release of the Password Generator CLI tool with interactive mode.

## Features

- Interactive mode by default with user-friendly prompts
- Multiple password types:
  - Standard (lowercase, uppercase, numbers, and special characters)
  - Alphabets only (lowercase and uppercase)
  - Numbers only
  - Alphanumeric (no special characters)
- Password strength indicator with color coding
- Configurable password length
- Generate multiple passwords at once
- Automatic clipboard copying
- Complex password generation with guaranteed character types
- Cross-platform support

## Installation

```bash
cargo install --git https://github.com/samyouel84/password-gen.git
```

## Usage

Simply run:
```bash
cargo run
```

For command-line mode:
```bash
cargo run -- -C [options]
```

See README.md for full documentation and options. 