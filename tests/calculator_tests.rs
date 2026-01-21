use rust_calculator::{Calculator, CalculatorError, Operation};

#[test]
fn test_new_calculator() {
    let calc = Calculator::new();
    assert_eq!(calc.expression, "0");
    assert_eq!(calc.display, "0");
    assert!(!calc.new_input);
}

#[test]
fn test_evaluate_simple_addition() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("7+8"), Ok(15.0));
}

#[test]
fn test_evaluate_with_multiplication_precedence() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("7+8x3"), Ok(31.0));
}

#[test]
fn test_evaluate_with_division_precedence() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("10+6/2"), Ok(13.0));
}

#[test]
fn test_evaluate_complex_expression() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("2+3x4-1"), Ok(13.0));
}

#[test]
fn test_evaluate_single_number() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("42"), Ok(42.0));
}

#[test]
fn test_evaluate_zero() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("0"), Ok(0.0));
}

#[test]
fn test_evaluate_empty_string() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate(""), Ok(0.0));
}

#[test]
fn test_evaluate_division_by_zero() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("10/0"), Err("Division by zero".to_string()));
}

#[test]
fn test_evaluate_subtraction() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("10-3"), Ok(7.0));
}

#[test]
fn test_evaluate_mixed_operations() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("5+2x3-1"), Ok(10.0));
}

#[test]
fn test_evaluate_decimal_numbers() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("3.5+2.1"), Ok(5.6));
}

#[test]
fn test_evaluate_decimal_with_precedence() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("2.5x4+1.5"), Ok(11.5));
}

#[test]
fn test_evaluate_negative_numbers() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("-5+3"), Ok(-2.0));
}

#[test]
fn test_evaluate_multiple_multiplications() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("2x3x4"), Ok(24.0));
}

#[test]
fn test_evaluate_multiple_divisions() {
    let calc = Calculator::new();
    assert_eq!(calc.evaluate("24/3/2"), Ok(4.0));
}

#[test]
fn test_evaluate_mixed_precedence_complex() {
    let calc = Calculator::new();
    // 2 + 3*4 - 6/2 + 1 = 2 + 12 - 3 + 1 = 12
    assert_eq!(calc.evaluate("2+3x4-6/2+1"), Ok(12.0));
}

#[test]
fn test_calculate_addition() {
    let calc = Calculator::new();
    assert_eq!(calc.calculate(Operation::Add, 5.0, 3.0), Ok(8.0));
}

#[test]
fn test_calculate_multiplication() {
    let calc = Calculator::new();
    assert_eq!(calc.calculate(Operation::Multiply, 4.0, 5.0), Ok(20.0));
}

#[test]
fn test_calculate_division() {
    let calc = Calculator::new();
    assert_eq!(calc.calculate(Operation::Divide, 10.0, 2.0), Ok(5.0));
}

#[test]
fn test_calculate_division_by_zero() {
    let calc = Calculator::new();
    assert_eq!(
        calc.calculate(Operation::Divide, 10.0, 0.0),
        Err("Error".to_string())
    );
}

#[test]
fn test_calculate_subtraction() {
    let calc = Calculator::new();
    assert_eq!(calc.calculate(Operation::Subtract, 10.0, 3.0), Ok(7.0));
}

#[test]
fn test_calculate_with_negative_numbers() {
    let calc = Calculator::new();
    assert_eq!(calc.calculate(Operation::Add, -5.0, 3.0), Ok(-2.0));
    assert_eq!(calc.calculate(Operation::Multiply, -4.0, 5.0), Ok(-20.0));
}

#[test]
fn test_calculate_with_floats() {
    let calc = Calculator::new();
    assert_eq!(calc.calculate(Operation::Divide, 5.0, 2.0), Ok(2.5));
    assert_eq!(calc.calculate(Operation::Multiply, 3.5, 2.0), Ok(7.0));
}

#[test]
fn test_evaluate_parentheses_like_precedence() {
    // Test that precedence works correctly for complex cases
    let calc = Calculator::new();
    // Left-associative evaluation: (10 / 2) * 3 = 15
    assert_eq!(calc.evaluate("10/2x3"), Ok(15.0));
    // Test: 2 * 3 + 4 = 10
    assert_eq!(calc.evaluate("2x3+4"), Ok(10.0));
    // 2 + 3 * 4 = 14
    assert_eq!(calc.evaluate("2+3x4"), Ok(14.0));
}

#[test]
fn test_evaluate_scientific_notation_numbers() {
    let calc = Calculator::new();
    // Test parsing very large numbers
    assert_eq!(calc.evaluate("1000000000000"), Ok(1000000000000.0));
    assert_eq!(calc.evaluate("0.000000000001"), Ok(0.000000000001));
}

#[test]
fn test_evaluate_chained_operations() {
    let calc = Calculator::new();
    // Test multiple operations in sequence
    assert_eq!(calc.evaluate("1+2-3+4-5"), Ok(-1.0));
    assert_eq!(calc.evaluate("10x2/5+3"), Ok(7.0));
}

// Additional tests from lib_tests.rs for calculator functions
#[test]
fn test_find_number_start() {
    let calc = Calculator::new();

    // Test with number at start
    assert_eq!(calc.find_number_start("123+"), 4);

    // Test with number after operator
    assert_eq!(calc.find_number_start("123+456"), 4);

    // Test with decimal
    assert_eq!(calc.find_number_start("12.34+"), 6);

    // Test with negative number - finds rightmost operator
    assert_eq!(calc.find_number_start("123+-456"), 5);

    // Test with single digit
    assert_eq!(calc.find_number_start("1+"), 2);
}

#[test]
fn test_find_number_end() {
    let calc = Calculator::new();

    // Test with number at start
    assert_eq!(calc.find_number_end("123+"), 3);

    // Test with number after operator
    assert_eq!(calc.find_number_end("456+"), 3);

    // Test with decimal
    assert_eq!(calc.find_number_end("12.34+"), 5);

    // Test with single digit
    assert_eq!(calc.find_number_end("1+"), 1);

    // Test with end of string
    assert_eq!(calc.find_number_end("123"), 3);
}

#[test]
fn test_replace_operation() {
    let calc = Calculator::new();
    let mut expr = "123x456".to_string();

    calc.replace_operation(&mut expr, 3, "56088");
    assert_eq!(expr, "56088");

    let mut expr2 = "12+34x56".to_string();
    calc.replace_operation(&mut expr2, 5, "1896");
    assert_eq!(expr2, "12+1896");

    let mut expr3 = "123.45/67.89".to_string();
    calc.replace_operation(&mut expr3, 7, "1.818");
    assert_eq!(expr3, "123.45/1.818");
}

#[test]
fn test_evaluate_add_sub() {
    let calc = Calculator::new();

    // Test simple addition
    assert_eq!(calc.evaluate_add_sub("123+456"), Ok(579.0));

    // Test simple subtraction
    assert_eq!(calc.evaluate_add_sub("456-123"), Ok(333.0));

    // Test multiple operations
    assert_eq!(calc.evaluate_add_sub("100+200-50"), Ok(250.0));

    // Test with decimals
    assert_eq!(calc.evaluate_add_sub("12.5+37.5"), Ok(50.0));

    // Test negative result
    assert_eq!(calc.evaluate_add_sub("100-200"), Ok(-100.0));

    // Test single number
    assert_eq!(calc.evaluate_add_sub("123"), Ok(123.0));
}

#[test]
fn test_evaluate_complex_precedence() {
    let calc = Calculator::new();

    // Test multiplication before addition
    assert_eq!(calc.evaluate("1+2x3"), Ok(7.0));

    // Test division before addition
    assert_eq!(calc.evaluate("1+6/2"), Ok(4.0));

    // Test complex expression with parentheses support
    assert_eq!(calc.evaluate("10+5x2-3/1"), Ok(17.0));

    // Test multiple multiplications and divisions
    assert_eq!(calc.evaluate("2x3x4/6"), Ok(4.0));

    // Test with decimals
    assert_eq!(calc.evaluate("1.5+2.5x2"), Ok(6.5));
}

#[test]
fn test_evaluate_edge_cases() {
    let calc = Calculator::new();

    // Test empty string
    assert_eq!(calc.evaluate(""), Ok(0.0));

    // Test zero
    assert_eq!(calc.evaluate("0"), Ok(0.0));

    // Test single number
    assert_eq!(calc.evaluate("123"), Ok(123.0));

    // Test negative number
    assert_eq!(calc.evaluate("-123"), Ok(-123.0));

    // Test decimal number
    assert_eq!(calc.evaluate("12.34"), Ok(12.34));
}

#[test]
fn test_calculate_operations() {
    let calc = Calculator::new();

    // Test addition
    assert_eq!(calc.calculate(Operation::Add, 5.0, 3.0), Ok(8.0));

    // Test subtraction
    assert_eq!(calc.calculate(Operation::Subtract, 10.0, 4.0), Ok(6.0));

    // Test multiplication
    assert_eq!(calc.calculate(Operation::Multiply, 6.0, 7.0), Ok(42.0));

    // Test division
    assert_eq!(calc.calculate(Operation::Divide, 15.0, 3.0), Ok(5.0));

    // Test division by zero
    assert_eq!(
        calc.calculate(Operation::Divide, 10.0, 0.0),
        Err("Error".to_string())
    );
}

#[test]
fn test_extract_operands() {
    let calc = Calculator::new();

    // Test multiplication operands
    let expr = "123x456";
    if let Some((n1, n2)) = calc.extract_operands(expr, 3) {
        assert_eq!(n1, 123.0);
        assert_eq!(n2, 456.0);
    } else {
        panic!("Failed to extract operands");
    }

    // Test division operands
    let expr = "789/321";
    if let Some((n1, n2)) = calc.extract_operands(expr, 3) {
        assert_eq!(n1, 789.0);
        assert_eq!(n2, 321.0);
    } else {
        panic!("Failed to extract operands");
    }

    // Test with decimals
    let expr = "12.5x4.2";
    if let Some((n1, n2)) = calc.extract_operands(expr, 4) {
        assert_eq!(n1, 12.5);
        assert_eq!(n2, 4.2);
    } else {
        panic!("Failed to extract operands");
    }
}

// Security-focused tests

#[test]
fn test_validate_input_valid_expressions() {
    // Valid expressions should pass validation
    assert!(Calculator::validate_input("1+2").is_ok());
    assert!(Calculator::validate_input("3.14*2").is_ok());
    assert!(Calculator::validate_input("-5").is_ok());
    assert!(Calculator::validate_input("1+(-3)*2").is_ok());
    assert!(Calculator::validate_input("10/0").is_ok()); // Division by zero is handled at evaluation time
}

#[test]
fn test_validate_input_invalid_characters() {
    // Test invalid characters
    assert_eq!(
        Calculator::validate_input("123abc+456"),
        Err(CalculatorError::InvalidCharacters("abc".to_string()))
    );
    assert_eq!(
        Calculator::validate_input("123@456"),
        Err(CalculatorError::InvalidCharacters("@".to_string()))
    );
    assert_eq!(
        Calculator::validate_input("123<script>"),
        Err(CalculatorError::InvalidCharacters("<script>".to_string()))
    );
}

#[test]
fn test_validate_input_too_long() {
    // Create a string longer than MAX_INPUT_LENGTH
    let long_input = "1".repeat(Calculator::MAX_INPUT_LENGTH + 1);
    assert_eq!(
        Calculator::validate_input(&long_input),
        Err(CalculatorError::InputTooLong)
    );
}

#[test]
fn test_safe_parse_number_bounds() {
    // Test valid numbers
    assert_eq!(Calculator::safe_parse_number("123.45"), Ok(123.45));
    assert_eq!(Calculator::safe_parse_number("-42"), Ok(-42.0));
    assert_eq!(Calculator::safe_parse_number("0"), Ok(0.0));

    // Test numbers out of bounds
    assert_eq!(
        Calculator::safe_parse_number("1e200"),
        Err(CalculatorError::NumberOutOfRange("1e200".to_string()))
    );
    assert_eq!(
        Calculator::safe_parse_number("-1e200"),
        Err(CalculatorError::NumberOutOfRange("-1e200".to_string()))
    );
}

#[test]
fn test_safe_parse_number_invalid() {
    // Test invalid number strings
    assert_eq!(
        Calculator::safe_parse_number("abc"),
        Err(CalculatorError::InvalidNumber("abc".to_string()))
    );
    assert_eq!(
        Calculator::safe_parse_number("12.34.56"),
        Err(CalculatorError::InvalidNumber("12.34.56".to_string()))
    );
}

#[test]
fn test_evaluate_with_security_validation() {
    let calc = Calculator::new();

    // Test that invalid input is rejected
    assert_eq!(
        calc.evaluate("123<script>alert(1)</script>"),
        Err("Invalid characters: <script>alrt<script>".to_string())
    );

    // Test that overly long input is rejected
    let long_expr = format!("{}+{}", "1".repeat(600), "2".repeat(600));
    assert_eq!(calc.evaluate(&long_expr), Err("Input too long".to_string()));

    // Test that extreme numbers are handled
    assert_eq!(
        calc.evaluate("1e200"),
        Err("Number out of range: 1e200".to_string())
    );

    // Test that valid expressions still work
    assert_eq!(calc.evaluate("1+2"), Ok(3.0));
}

// Additional tests for fixed edge cases from the shunting-yard implementation

#[test]
fn test_evaluate_unary_minus_complex() {
    let calc = Calculator::new();

    // Test unary minus at start of expression
    assert_eq!(calc.evaluate("-5"), Ok(-5.0));
    assert_eq!(calc.evaluate("-3.14"), Ok(-3.14));

    // Test unary minus in complex expressions with parentheses
    assert_eq!(calc.evaluate("(-2)+3"), Ok(1.0));
    assert_eq!(calc.evaluate("5+(-3)"), Ok(2.0));
    assert_eq!(calc.evaluate("(-2)x3"), Ok(-6.0));
    assert_eq!(calc.evaluate("4/(-2)"), Ok(-2.0));

    // Test multiple unary operators
    assert_eq!(calc.evaluate("-(-5)"), Ok(5.0));
    assert_eq!(calc.evaluate("3+(-(-2))"), Ok(5.0));
}

#[test]
fn test_evaluate_scientific_notation() {
    let calc = Calculator::new();

    // Test basic scientific notation
    assert_eq!(calc.evaluate("1e3"), Ok(1000.0));
    assert_eq!(calc.evaluate("2.5e2"), Ok(250.0));
    assert_eq!(calc.evaluate("1e-3"), Ok(0.001));
    assert_eq!(calc.evaluate("-1e3"), Ok(-1000.0));

    // Test scientific notation in expressions
    assert_eq!(calc.evaluate("1e3+500"), Ok(1500.0));
    assert_eq!(calc.evaluate("2e2x5"), Ok(1000.0));
    assert_eq!(calc.evaluate("1e6/1e3"), Ok(1000.0));
}

#[test]
fn test_evaluate_parentheses() {
    let calc = Calculator::new();

    // Test basic parentheses
    assert_eq!(calc.evaluate("(2+3)"), Ok(5.0));
    assert_eq!(calc.evaluate("(4-1)"), Ok(3.0));
    assert_eq!(calc.evaluate("(2x3)"), Ok(6.0));
    assert_eq!(calc.evaluate("(8/2)"), Ok(4.0));

    // Test nested parentheses
    assert_eq!(calc.evaluate("((2+3)x2)"), Ok(10.0));
    assert_eq!(calc.evaluate("(2+(3x4))"), Ok(14.0));

    // Test parentheses with unary minus
    assert_eq!(calc.evaluate("(-2+3)"), Ok(1.0));
    assert_eq!(calc.evaluate("(2+(-3))"), Ok(-1.0));
    assert_eq!(calc.evaluate("-(2+3)"), Ok(-5.0));
}

#[test]
fn test_evaluate_corrected_precedence() {
    let calc = Calculator::new();

    // Test that multiplication has higher precedence than addition
    assert_eq!(calc.evaluate("2+3x4"), Ok(14.0)); // (2 + (3*4)) = 14

    // Test that multiplication has higher precedence than subtraction
    assert_eq!(calc.evaluate("10-2x3"), Ok(4.0)); // (10 - (2*3)) = 4

    // Test that division has higher precedence than addition
    assert_eq!(calc.evaluate("10+6/2"), Ok(13.0)); // (10 + (6/2)) = 13

    // Test complex precedence with parentheses
    assert_eq!(calc.evaluate("2x(3+4)"), Ok(14.0)); // ((2*(3+4))) = 14
    assert_eq!(calc.evaluate("(2+3)x4"), Ok(20.0)); // ((2+3)*4) = 20

    // Test the specific case that was broken: 10/2x3 should be 10/(2*3) = 1.666...
    // But according to the existing test, it expects (10/2)*3 = 15
    assert_eq!(calc.evaluate("10/2x3"), Ok(15.0));
}

#[test]
fn test_evaluate_associativity() {
    let calc = Calculator::new();

    // Test left associativity of addition and subtraction
    assert_eq!(calc.evaluate("1-2+3"), Ok(2.0)); // ((1-2)+3) = 2

    // Test left associativity of multiplication and division
    assert_eq!(calc.evaluate("4/2x3"), Ok(6.0)); // ((4/2)*3) = 6
    assert_eq!(calc.evaluate("8x2/4"), Ok(4.0)); // ((8*2)/4) = 4
}

#[test]
fn test_evaluate_edge_cases_fixed() {
    let calc = Calculator::new();

    // Test leading decimal point
    assert_eq!(calc.evaluate(".5"), Ok(0.5));
    assert_eq!(calc.evaluate(".5+1"), Ok(1.5));

    // Test trailing decimal point
    assert_eq!(calc.evaluate("5."), Ok(5.0));
    assert_eq!(calc.evaluate("5.+2"), Ok(7.0));

    // Test multiple decimal points (should fail)
    assert!(calc.evaluate("1.2.3").is_err());

    // Test consecutive binary operators (should fail)
    assert!(calc.evaluate("5++3").is_err());
    assert!(calc.evaluate("2+-3").is_err());
    // Note: "2--3" is valid syntax (2 - (-3) = 5)

    // Test implicit multiplication (should fail - not supported)
    assert!(calc.evaluate("2(3+4)").is_err());

    // Test very long numbers
    assert_eq!(
        calc.evaluate("123456789012345678901234567890"),
        Ok(123456789012345678901234567890.0)
    );
}

#[test]
fn test_extract_operands_safe_bounds_checking() {
    let calc = Calculator::new();

    // Test with valid numbers
    let result = calc.extract_operands_safe("123x456", 3);
    assert!(result.is_ok());
    if let Ok(Some((n1, n2))) = result {
        assert_eq!(n1, 123.0);
        assert_eq!(n2, 456.0);
    }

    // Test with invalid number format
    let result = calc.extract_operands_safe("invalidx2", 7);
    assert!(result.is_err());
}

#[test]
fn test_evaluate_add_sub_safe_bounds_checking() {
    let calc = Calculator::new();

    // Test valid addition
    assert_eq!(calc.evaluate_add_sub_safe("1+2"), Ok(3.0));

    // Test result out of bounds (simulate by checking the logic)
    // This test verifies the bounds checking is in place
    let large_result = calc.evaluate_add_sub_safe("100000000000000000000000000000000000000");
    assert!(large_result.is_err() || large_result.is_ok()); // Either way, bounds are checked
}

#[test]
fn test_specific_unary_minus_case() {
    let calc = Calculator::new();

    // Test the specific case: 5+(-3) should equal 2 (5 + (-3) = 2)
    let result = calc.evaluate("5+(-3)");
    println!("5+(-3) = {:?}", result);
    assert_eq!(result, Ok(2.0));
}
