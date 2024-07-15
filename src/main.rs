use clap::Parser;

/// Password Generator CLI
#[derive(Parser, Debug)]
#[command(version = "1.0", about = "Generates passwords with various complexities")]
struct Args {
    /// Length of the password
    #[arg(short, long, default_value_t = 12)]
    length: usize,

    /// Number of passwords to generate
    #[arg(long, default_value_t = 1)]
    quantity: usize,

    /// Complexity: simple, secure, complex
    #[arg(short, long, default_value = "secure")]
    complexity: String,

    /// Include special characters
    #[arg(short, long)]
    special: bool,
}

fn main() {
    let args = Args::parse();

    println!("special {:?}", args.special);
    println!("complexity {:?}", args.complexity);
    println!("count {:?}", args.quantity);
    println!("length {:?}", args.length);
}
