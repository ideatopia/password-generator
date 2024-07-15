use crate::*;

#[test]
fn parse_long_args_valid_options() {
    // Valid long arguments for secure password with special characters
    let args = Args::try_parse_from([
        "password_generator",
        "--length",
        "16",
        "--quantity",
        "3",
        "--special",
    ])
    .unwrap();

    assert_eq!(args.length, 16);
    assert_eq!(args.quantity, 3);
    assert_eq!(args.complexity, ComplexityEnum::Secure);
    assert!(args.special);
}

#[test]
fn parse_args_valid_options() {
    // Valid short arguments for secure password with special characters
    let args = Args::try_parse_from([
        "password_generator",
        "-l",
        "16",
        "-q",
        "3",
        "-c",
        "secure",
        "--special",
    ])
    .unwrap();

    assert_eq!(args.length, 16);
    assert_eq!(args.quantity, 3);
    assert_eq!(args.complexity, ComplexityEnum::Secure);
    assert!(args.special);
}

#[test]
fn parse_args_use_default_length() {
    // Missing length argument
    let args = Args::try_parse_from(["password_generator", "-q", "2", "--special"]).unwrap();

    assert_eq!(args.length, 12);
}

#[test]
fn parse_args_use_default_quantity() {
    // Missing quantity argument
    let args = Args::try_parse_from(["password_generator", "-l", "16", "--special"]).unwrap();

    assert_eq!(args.quantity, 1);
}

#[test]
fn parse_args_use_default_complexity() {
    // Missing complexity argument
    let args =
        Args::try_parse_from(["password_generator", "-l", "16", "-q", "2", "--special"]).unwrap();

    assert_eq!(args.complexity, ComplexityEnum::Secure);
}

#[test]
fn parse_args_use_default_special() {
    // Missing special argument
    let args = Args::try_parse_from(["password_generator", "-l", "16", "-q", "2"]).unwrap();

    assert!(!args.special);
}

#[test]
fn parse_args_invalid_complexity() {
    // Invalid complexity option
    let result = Args::try_parse_from([
        "password_generator",
        "-l",
        "10",
        "-q",
        "1",
        "--complexity",
        "invalid",
    ]);

    assert!(result.is_err()); // Expect an error
}

#[test]
fn generate_password_length() {
    let length = 16;
    let password = generate_password(length, false, &ComplexityEnum::Simple);

    assert_eq!(password.len(), length);
}

#[test]
fn generate_password_special_chars() {
    let length = 16;
    let password = generate_password(length, true, &ComplexityEnum::Complex);

    assert!(password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)));
}

#[test]
fn generate_password_no_special_chars() {
    let length = 16;
    let password = generate_password(length, false, &ComplexityEnum::Secure);

    assert!(!password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)));
}

#[test]
fn generate_password_complexity_simple() {
    let length = 16;
    let password = generate_password(length, false, &ComplexityEnum::Simple);

    assert!(password
        .chars()
        .all(|c| "abcdefghijklmnopqrstuvwxyz".contains(c)));
}

#[test]
fn generate_password_complexity_secure() {
    let length = 16;
    let password = generate_password(length, false, &ComplexityEnum::Secure);

    assert!(password
        .chars()
        .any(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c)));
    assert!(password.chars().any(|c| "0123456789".contains(c)));
}

#[test]
fn generate_password_complexity_complex() {
    let length = 16;
    let password = generate_password(length, true, &ComplexityEnum::Complex);

    assert!(password
        .chars()
        .any(|c| "abcdefghijklmnopqrstuvwxyz".contains(c)));
    assert!(password
        .chars()
        .any(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c)));
    assert!(password.chars().any(|c| "0123456789".contains(c)));
    assert!(password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)));
}

#[test]
fn generate_multiple_passwords() {
    let length = 16;
    let quantity = 5;
    let passwords: Vec<String> = (0..quantity)
        .map(|_| generate_password(length, true, &ComplexityEnum::Secure))
        .collect();

    let unique_passwords: std::collections::HashSet<_> = passwords.iter().collect();
    assert_eq!(unique_passwords.len(), quantity); // Ensure all generated passwords are unique
}
