use crate::calculator::{Calculator, Operation};

/// GUI state management for the calculator application.
/// This struct manages UI-specific state that can be unit tested.
#[derive(Debug, Clone)]
pub struct CalculatorUIState {
    /// The calculator logic instance
    pub calculator: Calculator,
    /// Previous display text length for scroll management
    pub previous_display_len: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UIMessage {
    NumberPressed(u8),
    DecimalPressed,
    OperationPressed(Operation),
    EqualsPressed,
    ClearPressed,
    BackspacePressed,
    PercentagePressed,
    SignTogglePressed,
}

/// Result of processing a UI message, indicating if scrolling should occur.
#[derive(Debug, PartialEq)]
pub enum MessageResult {
    /// No scrolling needed
    NoScroll,
    /// Scrolling to end is needed
    ScrollToEnd,
}

impl CalculatorUIState {
    /// Creates a new UI state with default values.
    pub fn new() -> Self {
        Self {
            calculator: Calculator::new(),
            previous_display_len: 1,
        }
    }

    /// Processes a UI message and returns whether scrolling should occur.
    /// This is the extracted logic from main.rs that can be unit tested.
    pub fn process_message(&mut self, message: UIMessage) -> MessageResult {
        let old_len = self.calculator.expression.len();

        match message {
            UIMessage::NumberPressed(digit) => {
                self.calculator.handle_number_input(digit);
            }
            UIMessage::DecimalPressed => {
                self.calculator.handle_decimal_input();
            }
            UIMessage::OperationPressed(operation) => {
                self.calculator.handle_operation_input(operation);
            }
            UIMessage::EqualsPressed => {
                self.calculator.handle_equals_input();
            }
            UIMessage::ClearPressed => {
                self.calculator.handle_clear_input();
            }
            UIMessage::BackspacePressed => {
                self.calculator.handle_backspace_input();
            }
            UIMessage::PercentagePressed => {
                self.calculator.handle_percentage_input();
            }
            UIMessage::SignTogglePressed => {
                self.calculator.handle_sign_toggle_input();
            }
        }

        let new_len = self.calculator.expression.len();

        // Auto-scroll only when content grows (most natural UX)
        if new_len > old_len {
            MessageResult::ScrollToEnd
        } else {
            MessageResult::NoScroll
        }
    }

    /// Determines if scrolling should occur based on expression length changes.
    /// This logic is extracted and can be unit tested.
    pub fn should_scroll(&self, old_expression_len: usize, new_expression_len: usize) -> bool {
        new_expression_len > old_expression_len
    }
}

impl Default for CalculatorUIState {
    fn default() -> Self {
        Self::new()
    }
}
