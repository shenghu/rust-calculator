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

impl Calculator {
    /// Creates a new calculator instance with default values.
    pub fn new() -> Self {
        Self {
            expression: "0".to_string(),
            display: "0".to_string(),
            new_input: false,
        }
    }

    /// Evaluates a mathematical expression with operator precedence.
    ///
    /// Multiplication and division are evaluated before addition and subtraction.
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
        if expr.is_empty() || expr == "0" {
            return Ok(0.0);
        }

        // First, handle multiplication and division (higher precedence)
        let mut expr = expr.to_string();
        loop {
            let mut found = false;
            if let Some(pos) = expr.find('x') {
                if let Some((n1, n2)) = self.extract_operands(&expr, pos) {
                    let result = n1 * n2;
                    self.replace_operation(&mut expr, pos, &result.to_string());
                    found = true;
                }
            } else if let Some(pos) = expr.find('÷') {
                if let Some((n1, n2)) = self.extract_operands(&expr, pos) {
                    if n2 == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    let result = n1 / n2;
                    self.replace_operation(&mut expr, pos, &result.to_string());
                    found = true;
                }
            } else if let Some(pos) = expr.find('/') {
                if let Some((n1, n2)) = self.extract_operands(&expr, pos) {
                    if n2 == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    let result = n1 / n2;
                    self.replace_operation(&mut expr, pos, &result.to_string());
                    found = true;
                }
            }
            if !found {
                break;
            }
        }

        // Now handle addition and subtraction
        self.evaluate_add_sub(&expr)
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
