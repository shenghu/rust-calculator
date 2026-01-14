use rust_calculator::{Calculator, Operation};

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
    // Left to right evaluation: 10 / (2 * 3) = 10/6 ≈ 1.666
    assert_eq!(calc.evaluate("10/2x3"), Ok(10.0 / 6.0));
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

    // Test with negative number
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

    // Test complex expression
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
