use rust_calculator::{Calculator, Operation};

#[test]
fn test_handle_number_input_basic() {
    let mut calc = Calculator::new();
    calc.handle_number_input(5);
    assert_eq!(calc.expression, "5");
    assert_eq!(calc.display, "5");
}

#[test]
fn test_handle_percentage_with_invalid_display() {
    let mut calc = Calculator::new();
    calc.expression = "Error".to_string();
    calc.display = "Error".to_string();
    calc.handle_percentage_input();
    // Should do nothing when display is not a valid number
}

#[test]
fn test_handle_percentage_with_unparseable_display() {
    let mut calc = Calculator::new();
    calc.expression = "abc".to_string();
    calc.display = "abc".to_string();
    calc.handle_percentage_input();
    // Should do nothing when display cannot be parsed as f64 (covers lines 361-362)
}

#[test]
fn test_handle_number_input_after_error() {
    let mut calc = Calculator::new();
    calc.display = "Error".to_string();
    calc.handle_number_input(7);
    assert_eq!(calc.expression, "7");
    assert_eq!(calc.display, "7");
    assert!(!calc.new_input);
}

#[test]
fn test_handle_number_input_after_operation() {
    let mut calc = Calculator::new();
    calc.handle_number_input(5);
    calc.handle_operation_input(Operation::Add);
    calc.handle_number_input(3);
    assert_eq!(calc.expression, "5+3");
    assert_eq!(calc.display, "3");
    assert!(!calc.new_input);
}

#[test]
fn test_handle_operation_input_basic() {
    let mut calc = Calculator::new();
    calc.handle_number_input(5);
    calc.handle_operation_input(Operation::Add);
    assert_eq!(calc.expression, "5+");
    assert!(calc.new_input);
}

#[test]
fn test_handle_operation_input_replaces_last_operator() {
    let mut calc = Calculator::new();
    calc.handle_number_input(7);
    calc.handle_operation_input(Operation::Add);
    assert_eq!(calc.expression, "7+");
    calc.handle_operation_input(Operation::Subtract);
    assert_eq!(calc.expression, "7-");
    calc.handle_operation_input(Operation::Multiply);
    assert_eq!(calc.expression, "7x");
    calc.handle_operation_input(Operation::Divide);
    assert_eq!(calc.expression, "7÷");
}

#[test]
fn test_handle_operation_input_after_error() {
    let mut calc = Calculator::new();
    calc.display = "Error".to_string();
    calc.handle_operation_input(Operation::Add);
    // Should do nothing when display is Error
}

#[test]
fn test_handle_equals_input_basic() {
    let mut calc = Calculator::new();
    calc.handle_number_input(5);
    calc.handle_operation_input(Operation::Add);
    calc.handle_number_input(3);
    calc.handle_equals_input();
    assert_eq!(calc.display, "8");
    assert_eq!(calc.expression, "8");
    assert!(calc.new_input);
}

#[test]
fn test_handle_equals_input_with_error() {
    let mut calc = Calculator::new();
    calc.expression = "10/0".to_string();
    calc.handle_equals_input();
    assert_eq!(calc.display, "Division by zero");
    assert_eq!(calc.expression, "0");
}

#[test]
fn test_handle_equals_input_after_error() {
    let mut calc = Calculator::new();
    calc.display = "Error".to_string();
    calc.handle_equals_input();
    // Should do nothing when display is Error
}

#[test]
fn test_handle_decimal_input_basic() {
    let mut calc = Calculator::new();
    calc.handle_number_input(5);
    calc.handle_decimal_input();
    assert_eq!(calc.expression, "5.");
    assert_eq!(calc.display, "5.");
}

#[test]
fn test_handle_decimal_input_at_start() {
    let mut calc = Calculator::new();
    calc.handle_decimal_input();
    assert_eq!(calc.expression, "0.");
    assert_eq!(calc.display, "0.");
}

#[test]
fn test_handle_decimal_input_after_operation() {
    let mut calc = Calculator::new();
    calc.handle_number_input(5);
    calc.handle_operation_input(Operation::Add);
    calc.handle_decimal_input();
    assert_eq!(calc.expression, "5+0.");
    assert_eq!(calc.display, "0.");
}

#[test]
fn test_handle_decimal_input_after_error() {
    let mut calc = Calculator::new();
    calc.display = "Error".to_string();
    calc.handle_decimal_input();
    assert_eq!(calc.expression, "0.");
    assert_eq!(calc.display, "0.");
    assert!(!calc.new_input);
}

#[test]
fn test_handle_decimal_input_multiple_times() {
    let mut calc = Calculator::new();
    calc.handle_number_input(5);
    calc.handle_decimal_input();
    calc.handle_decimal_input(); // Should do nothing
    assert_eq!(calc.expression, "5.");
    assert_eq!(calc.display, "5.");
}

#[test]
fn test_handle_backspace_basic() {
    let mut calc = Calculator::new();
    calc.expression = "123".to_string();
    calc.display = "123".to_string();
    calc.handle_backspace_input();
    assert_eq!(calc.expression, "12");
    assert_eq!(calc.display, "12");
}

#[test]
fn test_handle_backspace_to_single_digit() {
    let mut calc = Calculator::new();
    calc.expression = "5".to_string();
    calc.display = "5".to_string();
    calc.handle_backspace_input();
    assert_eq!(calc.expression, "0");
    assert_eq!(calc.display, "0");
    assert!(!calc.new_input);
}

#[test]
fn test_handle_backspace_after_error() {
    let mut calc = Calculator::new();
    calc.display = "Error".to_string();
    calc.handle_backspace_input();
    assert_eq!(calc.expression, "0");
    assert_eq!(calc.display, "0");
    assert!(!calc.new_input);
}

#[test]
fn test_handle_percentage_basic() {
    let mut calc = Calculator::new();
    calc.expression = "50".to_string();
    calc.display = "50".to_string();
    calc.handle_percentage_input();
    assert_eq!(calc.display, "0.5");
}

#[test]
fn test_handle_percentage_with_decimal() {
    let mut calc = Calculator::new();
    calc.expression = "25.5".to_string();
    calc.display = "25.5".to_string();
    calc.handle_percentage_input();
    assert_eq!(calc.display, "0.255");
}

#[test]
fn test_handle_sign_toggle_basic() {
    let mut calc = Calculator::new();
    calc.expression = "5".to_string();
    calc.display = "5".to_string();
    calc.handle_sign_toggle_input();
    assert_eq!(calc.display, "-5");
}

#[test]
fn test_handle_sign_toggle_negative() {
    let mut calc = Calculator::new();
    calc.expression = "-3".to_string();
    calc.display = "-3".to_string();
    calc.handle_sign_toggle_input();
    assert_eq!(calc.display, "3");
}

#[test]
fn test_handle_sign_toggle_zero() {
    let mut calc = Calculator::new();
    calc.handle_sign_toggle_input();
    assert_eq!(calc.display, "0");
}

#[test]
fn test_handle_sign_toggle_with_invalid_display() {
    let mut calc = Calculator::new();
    calc.display = "Error".to_string();
    calc.handle_sign_toggle_input();
    // Should do nothing when display is not a valid number
    assert_eq!(calc.display, "Error");
}

#[test]
fn test_handle_sign_toggle_with_unparseable_display() {
    let mut calc = Calculator::new();
    calc.expression = "abc".to_string();
    calc.display = "abc".to_string();
    calc.handle_sign_toggle_input();
    // Should do nothing when display cannot be parsed as f64 (covers lines 381-382)
}

#[test]
fn test_handle_clear_input() {
    let mut calc = Calculator::new();
    calc.expression = "123+456".to_string();
    calc.display = "789".to_string();
    calc.new_input = true;
    calc.handle_clear_input();
    assert_eq!(calc.expression, "0");
    assert_eq!(calc.display, "0");
    assert!(!calc.new_input);
}

#[test]
fn test_handle_number_input_various_digits() {
    let mut calc = Calculator::new();
    for digit in 0..=9 {
        calc.handle_clear_input();
        calc.handle_number_input(digit);
        assert_eq!(calc.expression, digit.to_string());
        assert_eq!(calc.display, digit.to_string());
    }
}

#[test]
fn test_handle_operation_input_all_operations() {
    let mut calc = Calculator::new();
    calc.handle_number_input(5);

    calc.handle_operation_input(Operation::Add);
    assert_eq!(calc.expression, "5+");

    calc.handle_number_input(3);
    calc.handle_operation_input(Operation::Subtract);
    assert_eq!(calc.expression, "5+3-");

    calc.handle_number_input(2);
    calc.handle_operation_input(Operation::Multiply);
    assert_eq!(calc.expression, "5+3-2x");

    calc.handle_number_input(4);
    calc.handle_operation_input(Operation::Divide);
    assert_eq!(calc.expression, "5+3-2x4÷");
}

#[test]
fn test_handle_decimal_input_complex_cases() {
    let mut calc = Calculator::new();

    // Decimal after operation
    calc.handle_number_input(5);
    calc.handle_operation_input(Operation::Add);
    calc.handle_decimal_input();
    calc.handle_number_input(2);
    calc.handle_number_input(5);
    assert_eq!(calc.expression, "5+0.25");
    assert_eq!(calc.display, "0.25");
}

#[test]
fn test_handle_backspace_complex_expression() {
    let mut calc = Calculator::new();
    calc.handle_number_input(1);
    calc.handle_number_input(2);
    calc.handle_number_input(3);
    calc.handle_operation_input(Operation::Add);
    calc.handle_number_input(4);
    calc.handle_number_input(5);

    // Expression should be "123+45"
    assert_eq!(calc.expression, "123+45");
    assert_eq!(calc.display, "45");

    // Backspace should remove '5'
    calc.handle_backspace_input();
    assert_eq!(calc.expression, "123+4");
    assert_eq!(calc.display, "4");

    // Backspace should remove '4'
    calc.handle_backspace_input();
    assert_eq!(calc.expression, "123+");
    assert_eq!(calc.display, "0");

    // Backspace should remove '+'
    calc.handle_backspace_input();
    assert_eq!(calc.expression, "123");
    assert_eq!(calc.display, "123");
}

// Tests from lib_tests.rs for input-related functions
#[test]
fn test_extract_last_number() {
    let mut calc = Calculator::new();
    calc.expression = "123+456".to_string();
    assert_eq!(calc.extract_last_number(), "456");

    calc.expression = "123".to_string();
    assert_eq!(calc.extract_last_number(), "123");

    calc.expression = "123+456x789".to_string();
    assert_eq!(calc.extract_last_number(), "789");

    calc.expression = "123.45".to_string();
    assert_eq!(calc.extract_last_number(), "123.45");
}

#[test]
fn test_extract_current_number() {
    let mut calc = Calculator::new();
    calc.expression = "123+456".to_string();
    assert_eq!(calc.extract_current_number(), "456");

    calc.expression = "123".to_string();
    assert_eq!(calc.extract_current_number(), "123");

    calc.expression = "123+456x789".to_string();
    assert_eq!(calc.extract_current_number(), "789");

    calc.expression = "123.45".to_string();
    assert_eq!(calc.extract_current_number(), "123.45");
}
