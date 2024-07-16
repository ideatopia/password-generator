# Password Generator CLI

A command-line interface (CLI) tool for generating passwords with various complexities and options.

<p align="center">
    <a href="https://github.com/ideatopia/password-generator/actions/workflows/rust.yml">
        <img src="https://github.com/ideatopia/password-generator/actions/workflows/rust.yml/badge.svg" alt="rust.yml">
    </a>
    <a href="https://github.com/ideatopia/password-generator/actions/workflows/release.yml">
        <img src="https://github.com/ideatopia/password-generator/actions/workflows/release.yml/badge.svg" alt="release.yml">
    </a>
</p>

## Features

- Generates passwords of specified length and complexity.
- Supports simple, secure, and complex password types.
- Includes options to include special characters and hide passwords from display.
- Copies generated passwords to the clipboard if requested.
- Export password(s) text plain text file.

## Usage

### Installation

#### Build from source

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/ideatopia/password-generator
cd password-generator
cargo build --release
```

#### Ready-to-go

Download binaries according to your system

- Windows
```bash
curl -L https://github.com/ideatopia/password-generator/releases/latest/download/pwdgen-windows.exe -o pwdgen.exe
pwdgen.exe -h
```

- Ubuntu
```bash
curl -L https://github.com/ideatopia/password-generator/releases/latest/download/pwdgen-ubuntu -o pwdgen
chmod +x pwdgen
./pwdgen -h
```

- MacOS
```bash
curl -L https://github.com/ideatopia/password-generator/releases/latest/download/pwdgen-macos -o pwdgen
chmod +x pwdgen
./pwdgen -h
```

### Command-line Options

```
demo@ideatopia:~$ pwdgen -h

Password Generator 1.0.0
 Generates passwords with various complexities
 by ideatopia https://github.com/ideatopia

Usage: pwdgen [OPTIONS]

Options:
  -l, --length <LENGTH>          Length of the password [min: 8] [default: 12]
  -q, --quantity <QUANTITY>      Number of passwords to generate [default: 1]
  -c, --complexity <COMPLEXITY>  Level of complexity [default: secure] [possible values: simple, secure, complex]
  -s, --special                  Include special characters
      --hide                     Hide password from terminal display
      --copy                     Copy password to clipboard
      --export <EXPORT>          Export's file path
  -h, --help                     Print help
  -V, --version                  Print version
```

### Examples

Generate a secure password of length 16 with special characters and copy it to clipboard:

```bash
pwdgen -l 16 --complexity secure --special --copy
```

Generate 3 complex passwords of length 20, hide them from display, and copy to clipboard:

```bash
pwdgen -l 20 -q 3 --complexity complex --hide --copy
```

Generate 5 complex passwords of length 20, hide them from display, and export to passwords.txt:

```bash
pwdgen -l 20 -q 5 --complexity complex --hide --export passwords.txt
```

### Notes

- Ensure your system supports clipboard operations for the `--copy` option to work properly.
- Use the `--help` option to view all available command-line options and usage information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
