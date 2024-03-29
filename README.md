# Rusty Read
# Rust Command Line File Browser

This project is a command line file browser built in Rust, utilizing the `crossterm` and `tui` libraries for cross-platform terminal handling and to build a text user interface (TUI), respectively. It's designed to offer a simple, intuitive interface for navigating and viewing the contents of your filesystem.

![rusty_read screenshot](https://i.imgur.com/5a2fU0F.png)

## Features

- Cross-platform support, thanks to `crossterm`.
- Interactive TUI for browsing file system directories.
- Supports keyboard navigation with arrow keys, enter to navigate into directories, and backspace to go up.
- File Previews
- File Info

## Installation

Quickly install `rusty_read` by running the following command in your terminal:

```sh
curl -sSL -o install_rusty_read.sh https://raw.githubusercontent.com/charlesinwald/rusty_read/main/install.sh && chmod +x install_rusty_read.sh && ./install_rusty_read.sh
```

After installation, you might need to restart your terminal or source your shell configuration file:

- For Bash: `source ~/.bashrc`
- For Zsh: `source ~/.zshrc`


For detailed instructions and troubleshooting, visit [our GitHub repository](https://github.com/charlesinwald/rusty_read).

