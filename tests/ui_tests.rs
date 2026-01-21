use iced::keyboard;
use rust_calculator::{CalculatorUIState, MessageResult, Operation, UIMessage};

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
