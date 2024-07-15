#[cfg(test)]
mod test;

use clap::{Parser, ValueEnum};
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
}
