# Password Generator CLI

A command-line interface (CLI) tool for generating passwords with various complexities and options.

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

### Notes

- Ensure your system supports clipboard operations for the `--copy` option to work properly.
- Use the `--help` option to view all available command-line options and usage information.

## Todo
- [ ] copy generated password directly into the memory (like `CTRL/CMD + C`)
- [ ] add author to clap definition
- [ ] evaluate password strength (if user entered his own password)
- [ ] generated bin for linux, windows and darwin (mac) using GitHub Action
- [ ] check if password is already leaked like [Pwned Passwords](https://haveibeenpwned.com/Passwords)
- [ ] fix fails and remove todos from test.rs
- [ ] write test for copy to clipboard
- [ ] export generated password to 
  - [ ] plain text file
  - [ ] csv
  - [ ] json
- [ ] improve random password generation

  Why? Sometimes, certain tests fail (e.g., generate_password_complexity_complex) because characters are randomly selected without consideration for their composition. This can result in missing numbers or other 'required' elements. While the generated password is 'secure,' it may cause test failures.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
