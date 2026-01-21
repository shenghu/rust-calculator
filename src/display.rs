use crate::calculator::Calculator;

impl Calculator {
    /// Formats large numbers in a string to scientific notation.
    pub fn format_large_numbers(&self, expr: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if c.is_ascii_digit()
                || c == '.'
                || (c == '-' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
            {
                let start = i;
                i += 1;
                while i < chars.len() && {
                    let nc = chars[i];
                    nc.is_ascii_digit()
                        || nc == '.'
                        || (nc == 'e' && i + 1 < chars.len() && {
                            let next = chars[i + 1];
                            next.is_ascii_digit() || next == '+' || next == '-'
                        })
                } {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                if let Ok(value) = num_str.parse::<f64>() {
                    if value.abs() >= 1e9
                        || (value.abs() < 1.0 && value.abs() > 0.0)
                        || (num_str.len() > 10 && !num_str.contains('.') && !num_str.contains('e'))
                    {
                        result.push_str(&format!("{:.1e}", value));
                    } else {
                        result.push_str(&num_str);
                    }
                } else {
                    result.push_str(&num_str);
                }
            } else {
                result.push(c);
                i += 1;
            }
        }
        result
    }

    /// Returns the current expression for display purposes.
    /// For GUI display, show the full expression as typed.
    /// Long numeric strings are formatted as scientific notation.
    /// Scientific notation is also used for results after equals.
    /// Negative operands in expressions are shown with parentheses for clarity.
    pub fn display_string(&self) -> String {
        // Apply scientific notation formatting to the expression
        let formatted = self.format_large_numbers(&self.expression);

        // Add parentheses around negative operands in expressions
        self.add_parentheses_to_negative_operands(&formatted)
    }

    /// Adds parentheses around negative operands in expressions for display clarity.
    /// For example: "7+-9" becomes "7+(-9)"
    pub fn add_parentheses_to_negative_operands(&self, expr: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            // Check if this is an operator followed by a negative number
            if "+-x÷".contains(c) && i + 1 < chars.len() {
                let next_char = chars[i + 1];
                if next_char == '-' {
                    // Found operator followed by negative sign
                    result.push(c);
                    result.push('(');
                    result.push(next_char);
                    i += 2;

                    // Collect the rest of the negative number
                    while i < chars.len() {
                        let nc = chars[i];
                        if nc.is_ascii_digit() || nc == '.' || nc == 'e' || nc == '+' || nc == '-' {
                            result.push(nc);
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    result.push(')');
                    continue;
                }
            }

            result.push(c);
            i += 1;
        }

        result
    }
}
