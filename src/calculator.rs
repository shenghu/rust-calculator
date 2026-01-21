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

/// Tokens used in expression parsing for the shunting-yard algorithm.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    /// Numeric value
    Number(f64),
    /// Addition operator
    Plus,
    /// Binary subtraction operator
    Minus,
    /// Unary negation operator
    UnaryMinus,
    /// Multiplication operator
    Multiply,
    /// Division operator
    Divide,
    /// Left parenthesis
    LeftParen,
    /// Right parenthesis
    RightParen,
}

/// Represents operator precedence and associativity.
#[derive(Debug, Clone, Copy, PartialEq)]
struct OperatorInfo {
    precedence: u8,
    left_associative: bool,
}

impl Token {
    /// Returns operator information for tokens that are operators.
    fn operator_info(&self) -> Option<OperatorInfo> {
        match self {
            Token::Plus | Token::Minus => Some(OperatorInfo {
                precedence: 1,
                left_associative: true,
            }),
            Token::Multiply | Token::Divide => Some(OperatorInfo {
                precedence: 2, // Same precedence, left-associative
                left_associative: true,
            }),
            Token::UnaryMinus => Some(OperatorInfo {
                precedence: 3,           // Highest precedence for unary operators
                left_associative: false, // Right-associative
            }),
            _ => None,
        }
    }

    /// Checks if this token is a left parenthesis.
    fn is_left_paren(&self) -> bool {
        matches!(self, Token::LeftParen)
    }
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
    /// Tokenizes an input expression into tokens for the shunting-yard algorithm.
    ///
    /// Handles numbers, operators, and parentheses. Detects unary minus operations.
    ///
    /// # Arguments
    /// * `input` - The input expression string
    ///
    /// # Returns
    /// * `Ok(Vec<Token>)` - Successfully tokenized expression
    /// * `Err(String)` - Tokenization error with description
    fn tokenize(input: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        let mut expect_operand = true; // Track if we expect an operand (number/paren) or operator
        let mut prev_was_binary_op = false; // Track if previous token was a binary operator

        while let Some(&ch) = chars.peek() {
            match ch {
                '0'..='9' | '.' => {
                    // Parse number (including scientific notation)
                    let mut num_str = String::new();
                    let mut has_dot = false;
                    let mut has_e = false;

                    while let Some(&c) = chars.peek() {
                        match c {
                            '0'..='9' => {
                                num_str.push(c);
                                chars.next();
                            }
                            '.' => {
                                if has_dot {
                                    return Err(format!(
                                        "Invalid number format: multiple decimal points in '{}'",
                                        num_str + "."
                                    ));
                                }
                                has_dot = true;
                                num_str.push(c);
                                chars.next();
                            }
                            'e' | 'E' => {
                                if has_e {
                                    return Err(format!(
                                        "Invalid number format: multiple 'e' in '{}'",
                                        num_str + "e"
                                    ));
                                }
                                has_e = true;
                                num_str.push(c);
                                chars.next();

                                // Handle optional sign after 'e'
                                if let Some(&next) = chars.peek()
                                    && (next == '+' || next == '-')
                                {
                                    num_str.push(next);
                                    chars.next();
                                }
                            }
                            _ => break,
                        }
                    }

                    // Parse the number
                    match Self::safe_parse_number(&num_str) {
                        Ok(num) => tokens.push(Token::Number(num)),
                        Err(e) => return Err(e.to_string()),
                    }
                    expect_operand = false;
                    prev_was_binary_op = false; // Numbers are not operators
                }
                '+' => {
                    if expect_operand {
                        return Err("Unexpected '+' operator".to_string());
                    }
                    // Check for consecutive operators
                    if prev_was_binary_op {
                        return Err("Consecutive operators".to_string());
                    }
                    tokens.push(Token::Plus);
                    chars.next();
                    expect_operand = true;
                    prev_was_binary_op = true;
                }
                '-' => {
                    chars.next();
                    if expect_operand && !prev_was_binary_op {
                        // Unary minus only allowed at the start or after parentheses
                        tokens.push(Token::UnaryMinus);
                        expect_operand = true;
                        prev_was_binary_op = false; // Unary minus doesn't count as binary operator
                    } else if expect_operand && prev_was_binary_op {
                        // After a binary operator, - followed by nothing is invalid
                        return Err("Consecutive operators".to_string());
                    } else {
                        // When not expecting an operand, - is a binary operator
                        // Check for consecutive binary operators
                        if prev_was_binary_op {
                            return Err("Consecutive operators".to_string());
                        }
                        tokens.push(Token::Minus);
                        expect_operand = true;
                        prev_was_binary_op = true;
                    }
                }
                'x' | 'X' | '*' => {
                    if expect_operand {
                        return Err("Unexpected multiplication operator".to_string());
                    }
                    // Check for consecutive operators
                    if prev_was_binary_op {
                        return Err("Consecutive operators".to_string());
                    }
                    tokens.push(Token::Multiply);
                    chars.next();
                    expect_operand = true;
                    prev_was_binary_op = true;
                }
                '/' | '÷' => {
                    if expect_operand {
                        return Err("Unexpected division operator".to_string());
                    }
                    // Check for consecutive operators
                    if prev_was_binary_op {
                        return Err("Consecutive operators".to_string());
                    }
                    tokens.push(Token::Divide);
                    chars.next();
                    expect_operand = true;
                    prev_was_binary_op = true;
                }
                '(' => {
                    // Check for consecutive operators (parentheses can follow operators)
                    tokens.push(Token::LeftParen);
                    chars.next();
                    expect_operand = true;
                    prev_was_binary_op = false; // Parentheses are not operators
                }
                ')' => {
                    if expect_operand {
                        return Err("Unexpected ')' - missing operand".to_string());
                    }
                    tokens.push(Token::RightParen);
                    chars.next();
                    expect_operand = false;
                    prev_was_binary_op = false; // Parentheses are not operators
                }
                ' ' => {
                    // Skip whitespace
                    chars.next();
                }
                _ => {
                    return Err(format!("Invalid character: {}", ch));
                }
            }
        }

        Ok(tokens)
    }

    /// Converts infix tokens to postfix notation using the shunting-yard algorithm.
    ///
    /// # Arguments
    /// * `tokens` - Vector of infix tokens
    ///
    /// # Returns
    /// * `Ok(Vec<Token>)` - Postfix tokens ready for evaluation
    /// * `Err(String)` - Conversion error with description
    fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
        let mut output: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        for token in tokens {
            match token {
                Token::Number(_) => {
                    output.push(token);
                }
                Token::UnaryMinus => {
                    operator_stack.push(token);
                }
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    while let Some(top) = operator_stack.last() {
                        if top.is_left_paren() {
                            break;
                        }

                        if let (Some(current_info), Some(top_info)) =
                            (token.operator_info(), top.operator_info())
                        {
                            if top_info.precedence > current_info.precedence
                                || (top_info.precedence == current_info.precedence
                                    && current_info.left_associative)
                            {
                                output.push(operator_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(token);
                }
                Token::LeftParen => {
                    operator_stack.push(token);
                }
                Token::RightParen => {
                    let mut found_left_paren = false;
                    while let Some(op) = operator_stack.pop() {
                        if op.is_left_paren() {
                            found_left_paren = true;
                            break;
                        }
                        output.push(op);
                    }
                    if !found_left_paren {
                        return Err("Mismatched parentheses".to_string());
                    }
                }
            }
        }

        // Pop remaining operators
        while let Some(op) = operator_stack.pop() {
            if op.is_left_paren() {
                return Err("Mismatched parentheses".to_string());
            }
            output.push(op);
        }

        Ok(output)
    }

    /// Evaluates postfix notation tokens.
    ///
    /// # Arguments
    /// * `tokens` - Vector of postfix tokens
    ///
    /// # Returns
    /// * `Ok(f64)` - Result of the evaluation
    /// * `Err(String)` - Evaluation error with description
    fn evaluate_postfix(tokens: Vec<Token>) -> Result<f64, String> {
        let mut stack = Vec::new();

        for token in tokens {
            match token {
                Token::Number(num) => {
                    stack.push(num);
                }
                Token::UnaryMinus => {
                    let a = stack.pop().ok_or("Invalid expression: missing operand")?;
                    stack.push(-a);
                }
                Token::Plus => {
                    let b = stack.pop().ok_or("Invalid expression: missing operand")?;
                    let a = stack.pop().ok_or("Invalid expression: missing operand")?;
                    stack.push(a + b);
                }
                Token::Minus => {
                    let b = stack.pop().ok_or("Invalid expression: missing operand")?;
                    let a = stack.pop().ok_or("Invalid expression: missing operand")?;
                    stack.push(a - b);
                }
                Token::Multiply => {
                    let b = stack.pop().ok_or("Invalid expression: missing operand")?;
                    let a = stack.pop().ok_or("Invalid expression: missing operand")?;
                    stack.push(a * b);
                }
                Token::Divide => {
                    let b = stack.pop().ok_or("Invalid expression: missing operand")?;
                    let a = stack.pop().ok_or("Invalid expression: missing operand")?;
                    if b == 0.0 {
                        return Err(CalculatorError::DivisionByZero.to_string());
                    }
                    stack.push(a / b);
                }
                _ => {
                    return Err(format!(
                        "Unexpected token in postfix evaluation: {:?}",
                        token
                    ));
                }
            }
        }

        if stack.len() != 1 {
            return Err("Invalid expression: too many operands".to_string());
        }

        let result = stack[0];
        // Check final result bounds
        if !result.is_finite() || result.abs() > 1e100 {
            return Err(CalculatorError::NumberOutOfRange(result.to_string()).to_string());
        }

        Ok(result)
    }

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

        // Check for valid characters only (digits, operators, decimal point, scientific notation, whitespace, parentheses)
        // Parentheses are allowed for display purposes but not evaluated
        let invalid_chars: Vec<char> = input
            .chars()
            .filter(|&c| {
                !matches!(
                    c,
                    '0'..='9'
                        | '+'
                        | '-'
                        | 'x'
                        | 'X'
                        | '*'
                        | '/'
                        | '÷'
                        | '.'
                        | 'e'
                        | 'E'
                        | '('
                        | ')'
                        | ' '
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
    /// Uses the shunting-yard algorithm to handle proper operator precedence and associativity.
    /// Parentheses have the highest precedence, followed by multiplication and division,
    /// then addition and subtraction. Supports unary minus operations.
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
    /// assert_eq!(calc.evaluate("2x(3+4)"), Ok(14.0));
    /// assert_eq!(calc.evaluate("-5+3"), Ok(-2.0));
    /// ```
    pub fn evaluate(&self, expr: &str) -> Result<f64, String> {
        // Security: Validate input first
        if let Err(e) = Self::validate_input(expr) {
            return Err(e.to_string());
        }

        let trimmed = expr.trim();
        if trimmed.is_empty() || trimmed == "0" {
            return Ok(0.0);
        }

        // For single numbers, validate the number directly
        if !trimmed.contains(&['+', '-', 'x', 'X', '*', '/', '÷', '(', ')'][..]) {
            return Self::safe_parse_number(trimmed).map_err(|e| e.to_string());
        }

        // Tokenize the input
        let tokens = Self::tokenize(trimmed)?;

        // Convert to postfix notation
        let postfix = Self::shunting_yard(tokens)?;

        // Evaluate the postfix expression
        Self::evaluate_postfix(postfix)
    }

    /// Extracts the operands around an operator position with bounds checking.
    pub fn extract_operands_safe(
        &self,
        expr: &str,
        op_pos: usize,
    ) -> Result<Option<(f64, f64)>, CalculatorError> {
        // Find the operator character at this position
        let op_char = expr.chars().nth(op_pos).unwrap();
        let op_len = op_char.len_utf8();

        // Find left number start by scanning backwards for the start of the number
        let mut left_start = 0;
        for i in (0..op_pos).rev() {
            let c = expr.chars().nth(i).unwrap();
            if "+-x*÷/".contains(c) {
                left_start = i + 1;
                break;
            }
        }

        // Find right number end
        let mut right_end = op_pos + op_len;
        let mut found_digit = false;
        for i in (op_pos + op_len)..expr.len() {
            let c = expr.chars().nth(i).unwrap();
            if c.is_ascii_digit() || c == '.' {
                found_digit = true;
                right_end = i + 1;
            } else if c == '-' && !found_digit {
                // Leading negative sign
                right_end = i + 1;
            } else if "+-x*÷/".contains(c) && found_digit {
                // Hit an operator after finding digits
                break;
            } else if !c.is_ascii_digit() && c != '.' && c != '-' {
                // Hit some other character
                break;
            }
        }

        let num1 = &expr[left_start..op_pos];
        let num2 = &expr[op_pos + op_len..right_end];

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
        // Find the rightmost operator in the string
        for (i, c) in s.chars().rev().enumerate() {
            if "+-x÷/".contains(c) {
                let op_pos = s.len() - i - 1; // position of the operator from the left
                let result = op_pos + 1; // position after the operator
                return result;
            }
        }
        0
    }

    /// Finds the end position of the number after an operator.
    pub fn find_number_end(&self, s: &str) -> usize {
        if s.is_empty() {
            return 0;
        }

        let mut chars = s.chars();
        let mut i = 0;

        // Handle optional leading negative sign
        if let Some(c) = chars.next() {
            if c == '-' || c.is_ascii_digit() || c == '.' {
                i = 1;
            } else {
                return 0; // Non-numeric character at start
            }
        }

        // Continue with digits and decimal points
        for c in chars {
            if !c.is_ascii_digit() && c != '.' {
                break;
            }
            i += 1;
        }
        i
    }

    /// Replaces an operation with its result in the expression.
    pub fn replace_operation(&self, expr: &mut String, op_pos: usize, result: &str) {
        // Find the operator character at this position
        let op_char = expr.chars().nth(op_pos).unwrap();
        let op_len = op_char.len_utf8();

        // Find left number start by scanning backwards for the start of the number
        let mut left_start = 0;
        for i in (0..op_pos).rev() {
            let c = expr.chars().nth(i).unwrap();
            if "+-x*÷/".contains(c) {
                left_start = i + 1;
                break;
            }
        }

        // Find right number end
        let mut right_end = op_pos + op_len;
        let mut found_digit = false;
        for i in (op_pos + op_len)..expr.len() {
            let c = expr.chars().nth(i).unwrap();
            if c.is_ascii_digit() || c == '.' {
                found_digit = true;
                right_end = i + 1;
            } else if c == '-' && !found_digit {
                // Leading negative sign
                right_end = i + 1;
            } else if "+-x*÷/".contains(c) && found_digit {
                // Hit an operator after finding digits
                break;
            } else if !c.is_ascii_digit() && c != '.' && c != '-' {
                // Hit some other character
                break;
            }
        }

        let start = left_start;
        let end = right_end;

        expr.replace_range(start..end, result);
    }

    /// Evaluates addition and subtraction operations with bounds checking.
    pub fn evaluate_add_sub_safe(&self, expr: &str) -> Result<f64, String> {
        // If the expression contains no operators, just parse the number directly
        if !expr.contains(&['+', '-'][..]) {
            return Self::safe_parse_number(expr.trim()).map_err(|e| e.to_string());
        }

        let mut result = 0.0;
        let mut current_op = '+';
        let mut current_num = String::new();

        for c in expr.chars() {
            if c.is_ascii_digit() || c == '.' || (c == '-' && current_num.is_empty()) {
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
            } else {
                // If we encounter any other character, it's an error
                return Err(format!("Invalid character '{}' in expression", c));
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
            if c.is_ascii_digit() || c == '.' {
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
