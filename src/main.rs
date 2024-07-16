#[cfg(test)]
mod test;

use clap::{Parser, ValueEnum};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use rand::{
    seq::{IteratorRandom, SliceRandom},
    Rng,
};
use self_update::backends::github::Update;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::{env, iter};
use strum_macros::{Display, EnumProperty, EnumString};

/// Password Generator CLI
#[derive(Parser, Debug)]
#[command(
    name = "Password Generator",
    version = env!("CARGO_PKG_VERSION"),
    about = "Generates passwords with various complexities",
    author = "ideatopia"
)]
#[command(help_template = "\
{before-help}{name} {version}
 {about}
 by {author} https://github.com/ideatopia

{usage-heading} {usage}

{all-args}{after-help}
")]
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

    /// Hide password from terminal display
    #[arg(long)]
    hide: bool,

    /// Copy password to clipboard
    #[arg(long)]
    copy: bool,

    /// Export's file path
    #[arg(long, default_value = "", hide_default_value = true)]
    export: String,

    /// Self update from latest release
    #[arg(long)]
    update: bool,
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

    if args.update {
        self_update().expect("Unable to update the binary. Try from project's GitHub repository");
    }

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
        // Check if clipboard is available
        if let Ok(mut ctx) = ClipboardContext::new() {
            // Copy password to clipboard or fail
            match ctx.set_contents(passwords_string.to_owned()) {
                Ok(_) => println!("Password(s) copied to clipboard."),
                Err(e) => eprintln!("Failed to copy to clipboard: {}", e),
            }
        } else {
            eprintln!("Clipboard is not available on this system.");
        }
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

pub fn self_update() -> Result<(), Box<dyn std::error::Error>> {
    let os_type = env::consts::OS;
    let repo_owner = "ideatopia";
    let repo_name = "password-generator";

    let asset_name = match os_type {
        "windows" => "pwdgen-windows.exe",
        "linux" => "pwdgen-ubuntu",
        "macos" => "pwdgen-macos",
        _ => return Err("Unsupported operating system".into()),
    };

    let bin_name = match os_type {
        "windows" => "pwdgen.exe",
        _ => "pwdgen",
    };

    println!("Initiating self-update for {} system...", os_type);

    let status = Update::configure()
        .repo_owner(repo_owner)
        .repo_name(repo_name)
        .bin_name(bin_name)
        .target(asset_name)
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;

    if status.updated() {
        println!("Update successful!");
        println!("New version: {}", status.version());

        // Restart the application
        let args: Vec<String> = env::args().collect();
        let _ = Command::new(&args[0]).arg("-h").spawn();
        std::process::exit(0);
    } else {
        println!("No update available. Current version is up to date.");
    }

    Ok(())
}
