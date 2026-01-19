/// Represents a basic calculator with expression evaluation capabilities.
#[derive(Default, Debug, Clone)]
pub struct Calculator {
    /// The current expression being built
    pub expression: String,
    /// The current display value
    pub display: String,
    /// Whether the next input should start a new number
    pub new_input: bool,
}

/// Mathematical operations supported by the calculator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    /// Addition operation
    Add,
    /// Subtraction operation
    Subtract,
    /// Multiplication operation
    Multiply,
    /// Division operation
    Divide,
}

/// Custom error type for calculator operations.
#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorError {
    /// Division by zero error
    DivisionByZero,
    /// Invalid number format
    InvalidNumber(String),
    /// Invalid operation or syntax
    InvalidExpression(String),
    /// Input exceeds maximum allowed length
    InputTooLong,
    /// Input contains invalid characters
    InvalidCharacters(String),
    /// Numeric value out of allowed range
    NumberOutOfRange(String),
}

impl std::fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Division by zero"),
            CalculatorError::InvalidNumber(s) => write!(f, "Invalid number: {}", s),
            CalculatorError::InvalidExpression(s) => write!(f, "Invalid expression: {}", s),
            CalculatorError::InputTooLong => write!(f, "Input too long"),
            CalculatorError::InvalidCharacters(s) => write!(f, "Invalid characters: {}", s),
            CalculatorError::NumberOutOfRange(s) => write!(f, "Number out of range: {}", s),
        }
    }
}

impl std::error::Error for CalculatorError {}

impl Calculator {
    /// Maximum allowed input length for security (prevents resource exhaustion)
    pub const MAX_INPUT_LENGTH: usize = 1000;

    /// Validates input string for security constraints
    ///
    /// # Arguments
    /// * `input` - The input string to validate
    ///
    /// # Returns
    /// * `Ok(())` if input is valid
    /// * `Err(CalculatorError)` if input is invalid
    pub fn validate_input(input: &str) -> Result<(), CalculatorError> {
        // Check input length
        if input.len() > Self::MAX_INPUT_LENGTH {
            return Err(CalculatorError::InputTooLong);
        }

        // Check for valid characters only (digits, operators, decimal point, scientific notation, whitespace)
        let invalid_chars: Vec<char> = input
            .chars()
            .filter(|&c| {
                !matches!(
                    c,
                    '0'..='9' | '+' | '-' | 'x' | 'X' | '*' | '/' | '÷' | '.' | 'e' | 'E' | ' '
                )
            })
            .collect();

        if !invalid_chars.is_empty() {
            return Err(CalculatorError::InvalidCharacters(
                invalid_chars.into_iter().collect(),
            ));
        }

        Ok(())
    }

    /// Safely parses a number with bounds checking
    ///
    /// # Arguments
    /// * `s` - String slice to parse
    ///
    /// # Returns
    /// * `Ok(f64)` if parsing succeeds and number is in valid range
    /// * `Err(CalculatorError)` if parsing fails or number is out of range
    pub fn safe_parse_number(s: &str) -> Result<f64, CalculatorError> {
        let num = s
            .parse::<f64>()
            .map_err(|_| CalculatorError::InvalidNumber(s.to_string()))?;

        // Check for reasonable bounds to prevent extreme values
        if !num.is_finite() || num.abs() > 1e100 {
            return Err(CalculatorError::NumberOutOfRange(s.to_string()));
        }

        Ok(num)
    }

    /// Creates a new calculator instance with default values.
    pub fn new() -> Self {
        Self {
            expression: "0".to_string(),
            display: "0".to_string(),
            new_input: false,
        }
    }

    /// Evaluates a mathematical expression with operator precedence and security checks.
    ///
    /// Multiplication and division are evaluated before addition and subtraction.
    /// Input is validated for security constraints before evaluation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_calculator::Calculator;
    ///
    /// let calc = Calculator::new();
    /// assert_eq!(calc.evaluate("7+8x3"), Ok(31.0));
    /// assert_eq!(calc.evaluate("10/0"), Err("Division by zero".to_string()));
    /// ```
    pub fn evaluate(&self, expr: &str) -> Result<f64, String> {
        // Security: Validate input first
        if let Err(e) = Self::validate_input(expr) {
            return Err(e.to_string());
        }

        if expr.is_empty() || expr == "0" {
            return Ok(0.0);
        }

        // For single numbers, validate the number directly
        if !expr.contains(&['+', '-', 'x', 'X', '*', '/', '÷'][..]) {
            return Self::safe_parse_number(expr.trim()).map_err(|e| e.to_string());
        }

        // First, handle multiplication and division (higher precedence)
        let mut expr = expr.to_string();
        loop {
            let mut found = false;
            if let Some(pos) = expr.find('x') {
                match self.extract_operands_safe(&expr, pos) {
                    Ok(Some((n1, n2))) => {
                        let result = n1 * n2;
                        self.replace_operation(&mut expr, pos, &result.to_string());
                        found = true;
                    }
                    Ok(None) => {}
                    Err(e) => return Err(e.to_string()),
                }
            } else if let Some(pos) = expr.find('÷') {
                match self.extract_operands_safe(&expr, pos) {
                    Ok(Some((n1, n2))) => {
                        if n2 == 0.0 {
                            return Err(CalculatorError::DivisionByZero.to_string());
                        }
                        let result = n1 / n2;
                        self.replace_operation(&mut expr, pos, &result.to_string());
                        found = true;
                    }
                    Ok(None) => {}
                    Err(e) => return Err(e.to_string()),
                }
            } else if let Some(pos) = expr.find('/') {
                match self.extract_operands_safe(&expr, pos) {
                    Ok(Some((n1, n2))) => {
                        if n2 == 0.0 {
                            return Err(CalculatorError::DivisionByZero.to_string());
                        }
                        let result = n1 / n2;
                        self.replace_operation(&mut expr, pos, &result.to_string());
                        found = true;
                    }
                    Ok(None) => {}
                    Err(e) => return Err(e.to_string()),
                }
            }
            if !found {
                break;
            }
        }

        // Now handle addition and subtraction
        self.evaluate_add_sub_safe(&expr)
    }

    /// Extracts the operands around an operator position with bounds checking.
    pub fn extract_operands_safe(
        &self,
        expr: &str,
        op_pos: usize,
    ) -> Result<Option<(f64, f64)>, CalculatorError> {
        // Find the operator character at this position
        let op_char = expr
            .chars()
            .nth(expr.char_indices().position(|(i, _)| i == op_pos).unwrap())
            .unwrap();
        let op_len = op_char.len_utf8();

        let before = &expr[..op_pos];
        let after = &expr[op_pos + op_len..];

        // Find the number before the operator
        let num1_start = self.find_number_start(before);
        let num1 = &before[num1_start..];

        // Find the number after the operator
        let num2_end = self.find_number_end(after);
        let num2 = &after[..num2_end];

        let n1 = Self::safe_parse_number(num1)?;
        let n2 = Self::safe_parse_number(num2)?;
        Ok(Some((n1, n2)))
    }

    /// Extracts the operands around an operator position.
    pub fn extract_operands(&self, expr: &str, op_pos: usize) -> Option<(f64, f64)> {
        // Find the operator character at this position
        let op_char = expr
            .chars()
            .nth(expr.char_indices().position(|(i, _)| i == op_pos).unwrap())
            .unwrap();
        let op_len = op_char.len_utf8();

        let before = &expr[..op_pos];
        let after = &expr[op_pos + op_len..];

        // Find the number before the operator
        let num1_start = self.find_number_start(before);
        let num1 = &before[num1_start..];

        // Find the number after the operator
        let num2_end = self.find_number_end(after);
        let num2 = &after[..num2_end];

        let n1 = num1.parse().ok()?;
        let n2 = num2.parse().ok()?;
        Some((n1, n2))
    }

    /// Finds the start position of the number before an operator.
    pub fn find_number_start(&self, s: &str) -> usize {
        for (i, c) in s.chars().rev().enumerate() {
            if !c.is_digit(10) && c != '.' {
                return s.len() - i;
            }
        }
        0
    }

    /// Finds the end position of the number after an operator.
    pub fn find_number_end(&self, s: &str) -> usize {
        for (i, c) in s.chars().enumerate() {
            if !c.is_digit(10) && c != '.' {
                return i;
            }
        }
        s.len()
    }

    /// Replaces an operation with its result in the expression.
    pub fn replace_operation(&self, expr: &mut String, op_pos: usize, result: &str) {
        // Find the operator character at this position
        let op_char = expr
            .chars()
            .nth(expr.char_indices().position(|(i, _)| i == op_pos).unwrap())
            .unwrap();
        let op_len = op_char.len_utf8();

        let before = &expr[..op_pos];
        let after = &expr[op_pos + op_len..];

        let num1_start = self.find_number_start(before);
        let num2_end = self.find_number_end(after);

        let start = num1_start;
        let end = op_pos + op_len + num2_end;

        expr.replace_range(start..end, result);
    }

    /// Evaluates addition and subtraction operations with bounds checking.
    pub fn evaluate_add_sub_safe(&self, expr: &str) -> Result<f64, String> {
        let mut result = 0.0;
        let mut current_op = '+';
        let mut current_num = String::new();

        for c in expr.chars() {
            if c.is_digit(10) || c == '.' {
                current_num.push(c);
            } else if c == '+' || c == '-' {
                if !current_num.is_empty() {
                    let num = Self::safe_parse_number(&current_num).map_err(|e| e.to_string())?;
                    match current_op {
                        '+' => result += num,
                        '-' => result -= num,
                        _ => {}
                    }
                    current_num.clear();
                }
                current_op = c;
            }
        }

        // Handle the last number
        if !current_num.is_empty() {
            let num = Self::safe_parse_number(&current_num).map_err(|e| e.to_string())?;
            match current_op {
                '+' => result += num,
                '-' => result -= num,
                _ => {}
            }
        }

        // Check final result bounds
        if !result.is_finite() || result.abs() > 1e100 {
            return Err(CalculatorError::NumberOutOfRange(result.to_string()).to_string());
        }

        Ok(result)
    }

    /// Evaluates addition and subtraction operations.
    pub fn evaluate_add_sub(&self, expr: &str) -> Result<f64, String> {
        let mut result = 0.0;
        let mut current_op = '+';
        let mut current_num = String::new();

        for c in expr.chars() {
            if c.is_digit(10) || c == '.' {
                current_num.push(c);
            } else if c == '+' || c == '-' {
                if !current_num.is_empty() {
                    let num: f64 = current_num
                        .parse()
                        .map_err(|_| "Invalid number".to_string())?;
                    match current_op {
                        '+' => result += num,
                        '-' => result -= num,
                        _ => {}
                    }
                    current_num.clear();
                }
                current_op = c;
            }
        }

        // Handle the last number
        if !current_num.is_empty() {
            let num: f64 = current_num
                .parse()
                .map_err(|_| "Invalid number".to_string())?;
            match current_op {
                '+' => result += num,
                '-' => result -= num,
                _ => {}
            }
        }

        Ok(result)
    }

    /// Performs a basic calculation between two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_calculator::{Calculator, Operation};
    ///
    /// let calc = Calculator::new();
    /// assert_eq!(calc.calculate(Operation::Add, 5.0, 3.0), Ok(8.0));
    /// ```
    pub fn calculate(&self, operation: Operation, a: f64, b: f64) -> Result<f64, String> {
        match operation {
            Operation::Add => Ok(a + b),
            Operation::Subtract => Ok(a - b),
            Operation::Multiply => Ok(a * b),
            Operation::Divide => {
                if b != 0.0 {
                    Ok(a / b)
                } else {
                    Err("Error".to_string())
                }
            }
        }
    }
}
