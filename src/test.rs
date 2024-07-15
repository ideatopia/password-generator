use crate::*;

#[test]
fn parse_long_args_valid_options() {
    // Valid long arguments for secure password with special characters
    let args = Args::try_parse_from(["password_generator", "--length", "16", "--quantity", "3", "--special"]).unwrap();

    assert_eq!(args.length, 16);
    assert_eq!(args.quantity, 3);
    assert_eq!(args.complexity, "secure".to_string());
    assert!(args.special);
}

#[test]
fn parse_args_valid_options() {
    // Valid short arguments for secure password with special characters
    let args = Args::try_parse_from(["password_generator", "-l", "16", "-q", "3", "-c", "secure", "--special"]).unwrap();

    assert_eq!(args.length, 16);
    assert_eq!(args.quantity, 3);
    assert_eq!(args.complexity, "secure".to_string());
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
    let args = Args::try_parse_from(["password_generator", "-l", "16", "-q", "2", "--special"]).unwrap();

    assert_eq!(args.complexity, "secure".to_string());
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
    let result = Args::try_parse_from(["password_generator", "-l", "10", "-q", "1", "--complexity", "invalid"]);
    assert!(result.is_err()); // Expect an error
}
