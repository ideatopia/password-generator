# Password Generator CLI

A command-line interface (CLI) tool for generating passwords with various complexities and options.

<p align="center">
    <a href="https://github.com/ideatopia/password-generator/actions/workflows/rust.yml">
        <img src="https://github.com/ideatopia/password-generator/actions/workflows/rust.yml/badge.svg" alt="Test">
    </a>
</p>

## Features

- Generates passwords of specified length and complexity.
- Supports simple, secure, and complex password types.
- Includes options to include special characters and hide passwords from display.
- Copies generated passwords to the clipboard if requested.

## Usage

### Installation

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/ideatopia/password-generator
cd password_generator
cargo build --release
```

### Command-line Options

```
Generates passwords with various complexities

Usage: password_generator [OPTIONS]

Options:
  -l, --length <LENGTH>          Length of the password [default: 12]
  -q, --quantity <QUANTITY>      Number of passwords to generate [default: 1]
  -c, --complexity <COMPLEXITY>  Level of complexity [default: secure] [possible values: simple, secure, complex]
  -s, --special                  Include special characters
      --hide                     Hide password from terminal display [default: false]
      --copy                     Copy password to clipboard [default: false]
      --export <EXPORT>          Export's file path
  -h, --help                     Print help
  -V, --version                  Print version
```

### Examples

Generate a secure password of length 16 with special characters and copy it to clipboard:

```bash
password_generator -l 16 --complexity secure --special --copy
```

Generate 3 complex passwords of length 20, hide them from display, and copy to clipboard:

```bash
password_generator -l 20 -q 3 --complexity complex --hide --copy
```

Generate 5 complex passwords of length 20, hide them from display, and export to passwords.txt:

```bash
password_generator -l 20 -q 5 --complexity complex --hide --export passwords.txt
```

### Notes

- Ensure your system supports clipboard operations for the `--copy` option to work properly.
- Use the `--help` option to view all available command-line options and usage information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
