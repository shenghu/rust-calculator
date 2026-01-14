use crate::calculator::Calculator;

impl Calculator {
    /// Formats large numbers in a string to scientific notation.
    pub fn format_large_numbers(&self, expr: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = expr.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if c.is_digit(10)
                || c == '.'
                || (c == '-' && i + 1 < chars.len() && chars[i + 1].is_digit(10))
            {
                let start = i;
                i += 1;
                while i < chars.len() && {
                    let nc = chars[i];
                    nc.is_digit(10)
                        || nc == '.'
                        || (nc == 'e' && i + 1 < chars.len() && {
                            let next = chars[i + 1];
                            next.is_digit(10) || next == '+' || next == '-'
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
    pub fn display_string(&self) -> String {
        // Apply scientific notation formatting to the expression
        self.format_large_numbers(&self.expression)
    }
}
