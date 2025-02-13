# Password Generator CLI

A secure and flexible command-line password generator written in Rust. Generate strong passwords with various options and customizations.

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

1. Make sure you have Rust installed. If not, install it from [rustup.rs](https://rustup.rs/)

2. Clone the repository:
```bash
git clone https://github.com/yourusername/password-gen.git
cd password-gen
```

3. Build the project:
```bash
cargo build --release
```

The executable will be available in `target/release/password-gen`

## Usage

### Interactive Mode (Default)

Simply run the program without any arguments:
```bash
cargo run
```

You'll be guided through a series of prompts to:
1. Select password type
2. Set password length
3. Choose number of passwords to generate
4. Enable/disable complexity requirements
5. Choose whether to copy to clipboard

### Command-Line Mode

Use the `-C` or `--cli-mode` flag to run in command-line mode:

```bash
# Generate a standard password
cargo run -- -C

# Generate a password with specific length
cargo run -- -C --length 16

# Generate multiple passwords
cargo run -- -C --count 5

# Generate a specific type of password
cargo run -- -C --password-type alphanumeric
```

### Available Options

- `-l, --length <LENGTH>`: Set password length (default: 12)
- `-p, --password-type <TYPE>`: Choose password type:
  - `standard`: Include all character types (default)
  - `alphabets-only`: Only letters (a-z, A-Z)
  - `numbers-only`: Only digits (0-9)
  - `alphanumeric`: Letters and numbers
- `-n, --count <COUNT>`: Number of passwords to generate (default: 1)
- `-c, --complex`: Ensure at least one character from each required category (default: true)
- `--copy`: Copy the last generated password to clipboard
- `-C, --cli-mode`: Use command-line mode instead of interactive mode
- `-h, --help`: Show help information
- `-V, --version`: Show version information

### Examples

```bash
# Use interactive mode (default)
cargo run

# Generate a 20-character alphanumeric password in CLI mode
cargo run -- -C -l 20 -p alphanumeric

# Generate 5 passwords and copy the last one in CLI mode
cargo run -- -C -n 5 --copy

# Generate a simple password without complexity requirements in CLI mode
cargo run -- -C --complex false

# Generate a numbers-only password in CLI mode
cargo run -- -C -p numbers-only
```

## Password Strength

The password strength is indicated with color-coded levels:
- 🔴 Very Weak
- 🟡 Weak
- 🔵 Moderate
- 🟢 Strong
- 🟢 Very Strong

The strength is calculated based on:
- Password length
- Character type variety
- Complexity requirements

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. 