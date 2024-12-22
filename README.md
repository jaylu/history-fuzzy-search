# History Fuzzy Search (hfs)

`history-fuzzy-search` is a command line tool designed to enhance your command line experience by allowing you to quickly search through your command history using fuzzy matching. It supports both Bash and Zsh shell environments.

## Features

- Reads command line history from the appropriate files for Bash and Zsh.
- Fuzzy search functionality to filter command history.
- User-friendly interface for selecting commands using keyboard navigation.
- Copies the selected command to the clipboard for easy pasting.

## Usage

After installing the command line `hfs`, you can invoke it from terminal, and start type for searching command line from history

![Screenshot](./screenshot.png)

- Use the Up/Down arrow keys to navigate through the command history.
- Press `Enter` to copy the selected command to the clipboard.
- Press `Esc` to exit the selection interface.

## Installation

You can use either way to install `hfs` command in `~/.cargo/bin`. By including `~/.cargo/bin` in `$PATH`, you can run `hfs` directly from terminal.

### Install by Cargo

```bash
cargo install history-fuzzy-search
```

### Install by GIT

```bash
git clone https://github.com/jaylu/history-fuzzy-search.git
cd history-fuzzy-search
cargo build --release
cargo install --path .
```

## Development

```shell
## Build it
## After building the project, you can run the `hfs` command from your terminal:
## ./target/release/hfs
cargo build

## Run it 
cargo run

# To install the `hfs` binary in ~/.cargo/bin for local testing, run the following command
# ~/.cargo/bin expected to be included in $PATH
cargo install --path .
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.