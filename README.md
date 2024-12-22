# History Fuzzy Search (hfs)

`history-fuzzy-search` is a command line tool designed to enhance your command line experience by allowing you to quickly search through your command history using fuzzy matching. It supports both Bash and Zsh shell environments.

## Features

- Reads command line history from the appropriate files for Bash and Zsh.
- Fuzzy search functionality to filter command history.
- User-friendly interface for selecting commands using keyboard navigation.
- Copies the selected command to the clipboard for easy pasting.

## Installation

### By Cargo
To install `hfs` using Cargo, run the following command:

```bash
cargo install hfs
```

### By GIT

To install `hfs`, clone the repository and build the project using Cargo:

```bash
git clone <repository-url>
cd history-fuzzy-search
cargo build --release
```

## Development

```shell
## Build it
cargo build

## Run it 
cargo run

# To install the `fhs` binary in ~/.cargo/bin for local testing, run the following command
# ~/.cargo/bin expected to be included in $PATH
cargo install --path .
```

## Usage

After building the project, you can run the `hfs` command from your terminal:

```bash
./target/release/hfs
```

## Key Bindings

- Use the Up/Down arrow keys to navigate through the command history.
- Press `Enter` to copy the selected command to the clipboard.
- Press `Esc` to exit the selection interface.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.