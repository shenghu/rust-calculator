use crate::calculator::{Calculator, Operation};

impl Calculator {
    /// Handles number input for the calculator.
    pub fn handle_number_input(&mut self, digit: u8) {
        if self.display == "Error" {
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
            self.display = digit.to_string();
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
        if let Some(last_char) = self.expression.chars().last() {
            if "+-x÷".contains(last_char) {
                self.expression.pop();
            }
        }
        self.expression.push_str(op_char);
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
            self.display = "0.".to_string();
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
                // Removed an operator, show the previous number
                self.display = self.extract_last_number();
                self.new_input = true;
            } else {
                // Removed a digit/decimal, update the current number display
                self.display = self.extract_current_number();
                if self.display.is_empty() {
                    self.display = "0".to_string();
                    self.new_input = false;
                } else {
                    self.new_input = true;
                }
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
        if let Ok(value) = self.display.parse::<f64>() {
            let toggled = -value;
            // Handle -0 case
            self.display = if toggled == 0.0 {
                "0".to_string()
            } else {
                toggled.to_string()
            };
            // Update the last part of expression
            if let Some(last_space) = self.expression.rfind(' ') {
                self.expression.truncate(last_space + 1);
                self.expression.push_str(&self.display);
            } else {
                self.expression = self.display.clone();
            }
        }
    }

    /// Handles clear input for the calculator.
    pub fn handle_clear_input(&mut self) {
        self.expression = "0".to_string();
        self.display = "0".to_string();
        self.new_input = false;
    }
}
