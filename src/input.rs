use crate::calculator::{Calculator, Operation};

impl Calculator {
    /// Handles number input for the calculator.
    pub fn handle_number_input(&mut self, digit: u8) {
        if self.display == "Error"
            || self.display.starts_with("Invalid")
            || self.display.starts_with("Division")
            || self.display.starts_with("Number out of range")
        {
            // Clear any error state and start fresh
            self.expression = digit.to_string();
            self.display = digit.to_string();
            self.new_input = false;
        } else if self.new_input && !self.expression.contains(|c| "+-x÷".contains(c)) {
            // If expression is just a number (result), replace it
            self.expression = digit.to_string();
            self.display = digit.to_string();
            self.new_input = false;
        } else if self.new_input {
            self.expression.push_str(&digit.to_string());
            self.display = self.display_string(); // Update display to show full expression
            self.new_input = false;
        } else if self.display == "0" {
            self.expression = digit.to_string();
            self.display = digit.to_string();
        } else {
            self.expression.push_str(&digit.to_string());
            self.display.push_str(&digit.to_string());
        }
    }

    /// Handles operation input for the calculator.
    pub fn handle_operation_input(&mut self, operation: Operation) {
        if self.display == "Error" {
            return;
        }
        let op_char = match operation {
            Operation::Add => "+",
            Operation::Subtract => "-",
            Operation::Multiply => "x",
            Operation::Divide => "÷",
        };
        // If the expression ends with an operator, replace it instead of appending
        if let Some(last_char) = self.expression.chars().last()
            && "+-x÷".contains(last_char)
        {
            self.expression.pop();
        }
        self.expression.push_str(op_char);
        self.display = self.display_string(); // Update display to show full expression
        self.new_input = true;
    }

    /// Handles equals input for the calculator.
    pub fn handle_equals_input(&mut self) {
        if self.display == "Error" {
            return;
        }
        match self.evaluate(&self.expression) {
            Ok(result) => {
                // Format nice result for display
                self.display = if result.abs() >= 1e6 || (result.abs() < 1e-4 && result != 0.0) {
                    format!("{:.4e}", result)
                } else {
                    // Remove unnecessary trailing zeros and decimal point
                    let formatted = format!("{:.8}", result);
                    formatted
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .to_string()
                };
                self.expression = result.to_string(); // keep full precision
                self.new_input = true;
            }
            Err(error) => {
                self.display = error.clone();
                self.expression = "0".to_string();
            }
        }
    }

    /// Handles decimal point input for the calculator.
    pub fn handle_decimal_input(&mut self) {
        if self.display == "Error" {
            self.expression = "0.".to_string();
            self.display = "0.".to_string();
            self.new_input = false;
        } else if self.new_input && !self.expression.contains(|c| "+-x÷".contains(c)) {
            // If expression is just a number (result), replace it
            self.expression = "0.".to_string();
            self.display = "0.".to_string();
            self.new_input = false;
        } else if self.new_input {
            self.expression.push_str("0.");
            self.display = self.display_string(); // Update display to show full expression
            self.new_input = false;
        } else if self.display == "0" {
            self.expression = "0.".to_string();
            self.display = "0.".to_string();
        } else if !self.display.contains('.') {
            // Only add decimal if there isn't one already in current number
            self.expression.push('.');
            self.display.push('.');
        }
        // If already has decimal, do nothing
    }

    /// Handles backspace input for the calculator.
    pub fn handle_backspace_input(&mut self) {
        if self.display == "Error" {
            self.expression = "0".to_string();
            self.display = "0".to_string();
            self.new_input = false;
        } else if self.expression.len() > 1 {
            // Remove last character
            let last_char = self.expression.pop().unwrap();

            // Update display based on what was removed
            if "+-x÷".contains(last_char) {
                // Removed an operator, show the full expression
                self.display = self.display_string();
                self.new_input = true;
            } else {
                // Removed a digit/decimal, show the full expression
                self.display = self.display_string();
                self.new_input = !(self.expression.is_empty() || self.expression == "0");
            }
        } else if self.expression == "0" {
            // Already at minimum
        } else {
            self.expression = "0".to_string();
            self.display = "0".to_string();
            self.new_input = false;
        }
    }

    /// Extracts the last number from the expression (before the last operator)
    pub fn extract_last_number(&self) -> String {
        if let Some(last_op_pos) = self.expression.rfind(|c: char| "+-x÷".contains(c)) {
            self.expression[last_op_pos + 1..].to_string()
        } else {
            self.expression.clone()
        }
    }

    /// Extracts the current number being entered (after the last operator)
    pub fn extract_current_number(&self) -> String {
        if let Some(last_op_pos) = self.expression.rfind(|c: char| "+-x÷".contains(c)) {
            self.expression[last_op_pos + 1..].to_string()
        } else {
            self.expression.clone()
        }
    }

    /// Handles percentage input for the calculator.
    pub fn handle_percentage_input(&mut self) {
        if let Ok(value) = self.display.parse::<f64>() {
            let percentage = value / 100.0;
            self.display = percentage.to_string();
            // Update the last part of expression
            if let Some(last_space) = self.expression.rfind(' ') {
                self.expression.truncate(last_space + 1);
                self.expression.push_str(&percentage.to_string());
            } else {
                self.expression = percentage.to_string();
            }
        }
    }

    /// Handles sign toggle input for the calculator.
    pub fn handle_sign_toggle_input(&mut self) {
        // Check if we're in the middle of entering an expression with an operator
        // Exclude parentheses and only check for actual mathematical operators
        let has_operators = self.expression.contains(|c: char| "+x÷".contains(c))
            || (self.expression.contains('-')
                && self.find_last_operator_position(&self.expression).is_some());

        if has_operators {
            // There's an operator in the expression, so we're toggling the current operand
            // Find the last operator position
            if let Some(last_op_pos) = self.find_last_operator_position(&self.expression) {
                // Get the current number being entered (could be parenthesized)
                let current_part = &self.expression[last_op_pos + 1..];

                // Try to parse as a regular number first
                let num_value = if let Ok(value) = current_part.parse::<f64>() {
                    Some(value)
                } else if current_part.starts_with("(-") && current_part.ends_with(')') {
                    // Try parsing as parenthesized negative number like "(-3)"
                    current_part[1..current_part.len() - 1].parse::<f64>().ok()
                } else if current_part.starts_with('-')
                    && current_part.len() > 1
                    && current_part[1..].starts_with("(-")
                    && current_part.ends_with(')')
                {
                    // Try parsing as negative parenthesized number like "-(-3)"
                    let inner = &current_part[2..current_part.len() - 1];
                    inner.parse::<f64>().ok().map(|v| -v)
                } else {
                    None
                };

                if let Some(num_value) = num_value {
                    // Replace the current number part with the negated version
                    self.expression.truncate(last_op_pos + 1);

                    if num_value > 0.0 {
                        // Positive number: add parentheses with minus sign
                        self.expression.push_str(&format!("(-{})", num_value));
                    } else {
                        // Negative number: remove parentheses
                        self.expression.push_str(&num_value.abs().to_string());
                    }

                    self.display = self.display_string(); // Update display to show full expression
                    self.new_input = false;
                }
            }
        } else {
            // No operator, just toggle the sign of the entire expression
            // Handle the display format which may include parentheses
            let (display_value, _is_from_parentheses) =
                if self.display.starts_with("(-") && self.display.ends_with(')') {
                    // Format like "(-3)" - this represents a negative number, so extract and keep as negative
                    if let Ok(num_str) = self.display[1..self.display.len() - 1].parse::<f64>() {
                        (num_str, true)
                    } else {
                        return; // Invalid format, do nothing
                    }
                } else if let Ok(value) = self.display.parse::<f64>() {
                    // Regular number format
                    (value, false)
                } else {
                    return; // Cannot parse, do nothing
                };

            if display_value > 0.0 {
                self.expression = format!("-{}", display_value);
                self.display = format!("(-{})", display_value);
            } else {
                self.expression = display_value.abs().to_string();
                self.display = display_value.abs().to_string();
            }
        }
    }

    /// Finds the position of the last operator that separates operands at the top level.
    /// This handles parentheses properly - operators inside parentheses are ignored.
    /// It skips '-' when it's a sign for negative numbers (at start or after another operator).
    pub fn find_last_operator_position(&self, expr: &str) -> Option<usize> {
        let chars: Vec<char> = expr.chars().collect();
        let mut i = chars.len() as i32 - 1;
        let mut paren_depth = 0;

        while i >= 0 {
            let c = chars[i as usize];
            if c == ')' {
                paren_depth += 1;
            } else if c == '(' {
                paren_depth -= 1;
            } else if paren_depth == 0 && "+-x÷".contains(c) {
                // We're at the top level and found an operator
                // Check if this is a '-' that is a sign for a negative number
                // A '-' is a negative sign if:
                // 1. It's at the beginning of the expression (i == 0), OR
                // 2. It's immediately after another operator
                let is_negative_sign =
                    c == '-' && (i == 0 || "+-x÷".contains(chars[(i - 1) as usize]));
                if !is_negative_sign {
                    // This is a separating operator
                    return Some(i as usize);
                }
                // Skip this '-' as it's a sign
            }
            i -= 1;
        }
        None
    }

    /// Handles clear input for the calculator.
    pub fn handle_clear_input(&mut self) {
        self.expression = "0".to_string();
        self.display = "0".to_string();
        self.new_input = false;
    }
}
