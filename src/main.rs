#[cfg(test)]
mod test;

use std::iter;
use clap::{Parser, ValueEnum};
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

    println!("special {:?}", args.special);
    println!("complexity {:?}", args.complexity);
    println!("count {:?}", args.quantity);
    println!("length {:?}", args.length);

    for _ in 0..args.quantity {
        let password = generate_password(args.length, args.special, &args.complexity);
        println!("{}", password);
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
