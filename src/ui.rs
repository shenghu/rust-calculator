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

#[cfg(test)]
mod ui_tests {
    use super::*;

    #[test]
    fn test_ui_state_creation() {
        let ui_state = CalculatorUIState::new();
        assert_eq!(ui_state.calculator.expression, "0");
        assert_eq!(ui_state.calculator.display, "0");
        assert_eq!(ui_state.previous_display_len, 1);
    }

    #[test]
    fn test_process_message_number_pressed_scroll() {
        let mut ui_state = CalculatorUIState::new();
        ui_state.calculator.expression = "123".to_string();
        ui_state.calculator.display = "123".to_string();

        let result = ui_state.process_message(UIMessage::NumberPressed(4));
        assert_eq!(result, MessageResult::ScrollToEnd);
        assert_eq!(ui_state.calculator.expression, "1234");
        assert_eq!(ui_state.calculator.display, "1234");
    }

    #[test]
    fn test_process_message_operation_pressed_scroll() {
        let mut ui_state = CalculatorUIState::new();
        ui_state.calculator.expression = "123".to_string();
        ui_state.calculator.display = "123".to_string();

        let result = ui_state.process_message(UIMessage::OperationPressed(Operation::Add));
        assert_eq!(result, MessageResult::ScrollToEnd);
        assert_eq!(ui_state.calculator.expression, "123+");
        assert_eq!(ui_state.calculator.display, "123");
    }

    #[test]
    fn test_process_message_backspace_no_scroll() {
        let mut ui_state = CalculatorUIState::new();
        ui_state.calculator.expression = "123".to_string();
        ui_state.calculator.display = "123".to_string();

        let result = ui_state.process_message(UIMessage::BackspacePressed);
        assert_eq!(result, MessageResult::NoScroll);
        assert_eq!(ui_state.calculator.expression, "12");
        assert_eq!(ui_state.calculator.display, "12");
    }

    #[test]
    fn test_process_message_equals_no_scroll() {
        let mut ui_state = CalculatorUIState::new();
        ui_state.calculator.expression = "2+3".to_string();
        ui_state.calculator.display = "3".to_string();

        let result = ui_state.process_message(UIMessage::EqualsPressed);
        assert_eq!(result, MessageResult::NoScroll);
        assert_eq!(ui_state.calculator.display, "5");
        assert_eq!(ui_state.calculator.expression, "5");
    }

    #[test]
    fn test_should_scroll_logic() {
        let ui_state = CalculatorUIState::new();

        // Should scroll when expression grows
        assert!(ui_state.should_scroll(3, 5));
        assert!(ui_state.should_scroll(0, 1));

        // Should not scroll when expression stays same or shrinks
        assert!(!ui_state.should_scroll(5, 5));
        assert!(!ui_state.should_scroll(5, 3));
        assert!(!ui_state.should_scroll(1, 0));
    }

    #[test]
    fn test_all_message_types_handled() {
        let mut ui_state = CalculatorUIState::new();

        // Test all message types are handled without panicking
        let messages = vec![
            UIMessage::NumberPressed(5),
            UIMessage::DecimalPressed,
            UIMessage::OperationPressed(Operation::Add),
            UIMessage::EqualsPressed,
            UIMessage::ClearPressed,
            UIMessage::BackspacePressed,
            UIMessage::PercentagePressed,
            UIMessage::SignTogglePressed,
        ];

        for message in messages {
            let _result = ui_state.process_message(message);
            // Reset for next test
            ui_state.calculator.handle_clear_input();
        }
    }

    #[test]
    fn test_scroll_behavior_complex_expression() {
        let mut ui_state = CalculatorUIState::new();

        // Build a complex expression and verify scroll behavior
        ui_state.process_message(UIMessage::NumberPressed(1)); // "1" - should scroll
        assert_eq!(
            ui_state.process_message(UIMessage::NumberPressed(2)),
            MessageResult::ScrollToEnd
        ); // "12" - should scroll

        ui_state.process_message(UIMessage::OperationPressed(Operation::Add)); // "12+" - should scroll
        assert_eq!(
            ui_state.process_message(UIMessage::NumberPressed(3)),
            MessageResult::ScrollToEnd
        ); // "12+3" - should scroll

        ui_state.process_message(UIMessage::EqualsPressed); // "15" - no scroll (length same)
        assert_eq!(
            ui_state.process_message(UIMessage::BackspacePressed),
            MessageResult::NoScroll
        ); // "1" - no scroll (shorter)
    }
}
