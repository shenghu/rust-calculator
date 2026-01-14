use rust_calculator::Calculator;

#[test]
fn test_display_string_initial() {
    let calc = Calculator::new();
    assert_eq!(calc.display_string(), "0");
}

#[test]
fn test_display_string_short_expression() {
    let mut calc = Calculator::new();
    calc.expression = "123+456".to_string();
    assert_eq!(calc.display_string(), "123+456");
}

#[test]
fn test_display_string_with_trailing_operator() {
    let mut calc = Calculator::new();
    calc.expression = "123+".to_string();
    assert_eq!(calc.display_string(), "123+");
}

#[test]
fn test_display_string_large_number() {
    let mut calc = Calculator::new();
    calc.expression = "123456789012345678901234567890".to_string();
    let display = calc.display_string();
    assert!(display.contains("e"));
    assert!(display.len() <= 15);
}

#[test]
fn test_display_string_very_long_expression() {
    let mut calc = Calculator::new();
    calc.expression =
        "123456789012345678901234567890123456789012345678901234567890+abcdef".to_string();
    let display = calc.display_string();
    assert!(!display.ends_with("..."));
    assert!(display.len() > 10);
}

#[test]
fn test_display_string_single_large_number() {
    let mut calc = Calculator::new();
    calc.expression = "123456789012345678901234567890".to_string();
    let display = calc.display_string();
    // Should format as scientific notation
    assert!(display.contains("e"));
}

#[test]
fn test_display_string_result_formatting() {
    let mut calc = Calculator::new();

    // Large number should use scientific notation when it's a plain number result
    calc.expression = "123456789012".to_string(); // 12 chars, no decimal
    let display = calc.display_string();
    assert!(display.contains("e"));

    // Numbers with decimals are not formatted
    calc.expression = "123.456".to_string();
    assert_eq!(calc.display_string(), "123.456");

    // Shorter numbers are not formatted
    calc.expression = "123456789".to_string(); // 9 chars
    assert_eq!(calc.display_string(), "123456789");
}

#[test]
fn test_display_string_preserves_decimals() {
    let mut calc = Calculator::new();
    // display_string just shows what's in expression, doesn't format
    calc.expression = "8.0".to_string();
    assert_eq!(calc.display_string(), "8.0");

    calc.expression = "8.50".to_string();
    assert_eq!(calc.display_string(), "8.50");

    calc.expression = "8.000".to_string();
    assert_eq!(calc.display_string(), "8.000");
}

// Test from lib_tests.rs for display-related function
#[test]
fn test_format_large_numbers() {
    let calc = Calculator::new();

    // Test large number
    assert_eq!(calc.format_large_numbers("123456789012"), "1.2e11");

    // Test small number
    assert_eq!(calc.format_large_numbers("0.000000123"), "1.2e-7");

    // Test normal number
    assert_eq!(calc.format_large_numbers("123"), "123");

    // Test number with operator
    assert_eq!(calc.format_large_numbers("123+4567890123"), "123+4.6e9");

    // Test decimal number
    assert_eq!(calc.format_large_numbers("123.456"), "123.456");

    // Test already scientific notation
    assert_eq!(calc.format_large_numbers("1.23e10"), "1.2e10");

    // Test medium number not formatted
    assert_eq!(calc.format_large_numbers("123456789"), "123456789");
}
