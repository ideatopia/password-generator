#[cfg(test)]
mod test;

use std::fs::File;
use clap::{Parser, ValueEnum};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use std::io::Write;
use std::iter;
use std::path::Path;
use strum_macros::{Display, EnumProperty, EnumString};

/// Password Generator CLI
#[derive(Parser, Debug)]
#[command(
    version = "1.0",
    about = "Generates passwords with various complexities",
    author = "ideatopia"
)]
struct Args {
    /// Length of the password [min: 8]
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    /// Number of passwords to generate
    #[arg(short, long, default_value_t = 1)]
    quantity: usize,

    /// Level of complexity
    #[arg(short, long, default_value_t = ComplexityEnum::Secure, value_enum)]
    complexity: ComplexityEnum,

    /// Include special characters
    #[arg(short, long)]
    special: bool,

    /// Hide password from terminal display [default: false]
    #[arg(long)]
    hide: bool,

    /// Copy password to clipboard [default: false]
    #[arg(long)]
    copy: bool,

    /// Export's file path
    #[arg(long, default_value = "")]
    export: String,
}

/// Complexity levels for password generation
#[derive(Debug, EnumString, Display, Clone, EnumProperty, ValueEnum, PartialEq)]
#[strum(serialize_all = "lowercase")]
pub enum ComplexityEnum {
    Simple,
    Secure,
    Complex,
}

fn main() {
    let args = Args::parse();

    // why 8 ? 'cuz we got 8 bits
    let required_length = 8;

    if args.length < required_length {
        panic!("Password length must be at least {}.", required_length);
    }

    let newline = if cfg!(target_os = "windows") {
        "\r\n"
    } else {
        "\n"
    };

    let mut passwords = String::new();
    let mut passwords_generated = 0;
    let mut clipboard = ClipboardContext::new().unwrap(); // Initialize clipboard provider

    while passwords_generated < args.quantity {
        let password = generate_password(args.length, args.special, &args.complexity);

        passwords.push_str(&password);
        passwords.push_str(newline);

        passwords_generated += 1;
    }

    let passwords_string: String = passwords.trim().parse().unwrap();

    if !args.hide {
        // Print the password(s)
        println!("{}", passwords_string);
    }

    // Ensure the export path is not empty
    if !args.export.is_empty() {
        // Check if the file already exists
        if Path::new(&args.export).exists() {
            eprintln!("File already exists: {}", &args.export);
            std::process::exit(1);
        }

        let mut file = File::create(&args.export).expect("Failed to create file");

        match file.write_all(passwords_string.as_bytes()) {
            Ok(_) => println!("Password(s) exported to {}", args.export),
            Err(e) => eprintln!("Failed to export passwords: {}", e),
        }
    }

    if args.copy {
        // Copy password to clipboard
        clipboard
            .set_contents(passwords_string)
            .unwrap();
        println!("Password(s) copied to clipboard.");
    }
}

pub fn generate_password(
    length: usize,
    use_special_chars: bool,
    complexity: &ComplexityEnum,
) -> String {
    let mut rng = rand::thread_rng();
    let (lowercase, uppercase, numbers, special_chars) = (
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "0123456789",
        "!@#$%^&*()_+-=[]{}|;:,.<>?",
    );

    let charset = match complexity {
        ComplexityEnum::Simple => lowercase,
        ComplexityEnum::Secure => &format!("{}{}{}", lowercase, uppercase, numbers),
        ComplexityEnum::Complex => {
            &format!("{}{}{}{}", lowercase, uppercase, numbers, special_chars)
        }
    };

    let charset: Vec<char> = if use_special_chars {
        format!("{}{}", charset, special_chars).chars().collect()
    } else {
        charset.chars().collect()
    };

    let mut password: Vec<char> = Vec::new();

    if matches!(complexity, ComplexityEnum::Secure | ComplexityEnum::Complex) {
        // Select at least one character from each required category for Secure and Complex complexity
        password.push(lowercase.chars().choose(&mut rng).unwrap());
        password.push(uppercase.chars().choose(&mut rng).unwrap());
        password.push(numbers.chars().choose(&mut rng).unwrap());
    }

    if use_special_chars {
        password.push(special_chars.chars().choose(&mut rng).unwrap());
    }

    // Fill the rest of the password with random characters from the charset
    password.extend(
        iter::repeat_with(|| charset[rng.gen_range(0..charset.len())])
            .take(length - password.len()),
    );

    // Shuffle the password to ensure randomness
    password.shuffle(&mut rng);

    password.iter().collect()
}
