#[cfg(test)]
mod test;

use std::iter;
use clap::{Parser, ValueEnum};
use clipboard::{ClipboardContext, ClipboardProvider};
use rand::Rng;
use strum_macros::{Display, EnumProperty, EnumString};

/// Password Generator CLI
#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Generates passwords with various complexities")]
struct Args {
    /// Length of the password
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

        if !args.hide {
            // Print the password
            println!("{}", password);
        }

        if args.copy {
            passwords.push_str(&password);
            passwords.push_str(newline);
        }

        passwords_generated += 1;
    }

    if args.copy {
        // Copy password to clipboard
        clipboard.set_contents(passwords.trim().parse().unwrap()).unwrap();
        println!("Password(s) copied to clipboard.");
    }
}

pub fn generate_password(length: usize, use_special_chars: bool, complexity: &ComplexityEnum) -> String {
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
        ComplexityEnum::Complex => &format!("{}{}{}{}", lowercase, uppercase, numbers, special_chars),
    };

    let charset: Vec<char> = if use_special_chars {
        format!("{}{}", charset, special_chars).chars().collect()
    } else {
        charset.chars().collect()
    };

    iter::repeat_with(|| charset[rng.gen_range(0..charset.len())])
        .take(length)
        .collect()
}
