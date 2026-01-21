use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Task, Theme, application, event, keyboard};
use rust_calculator::{CalculatorUIState, MessageResult, Operation, UIMessage};
use std::sync::LazyLock;

// Static ID for the display scrollable widget - must be reused for scroll_to to work
static DISPLAY_SCROLL_ID: LazyLock<scrollable::Id> =
    LazyLock::new(|| scrollable::Id::new("display_scroll"));

#[derive(Default)]
struct Calculator {
    ui_state: CalculatorUIState,
    pressed_keys: std::collections::HashSet<iced::keyboard::Key>,
    key_mapping: std::collections::HashMap<iced::keyboard::Key, iced::keyboard::Key>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NumberPressed(u8),
    DecimalPressed,
    OperationPressed(Operation),
    EqualsPressed,
    ClearPressed,
    BackspacePressed,
    PercentagePressed,
    SignTogglePressed,
    KeyboardEvent(iced::keyboard::Key),
    KeyCombinationPressed {
        original: iced::keyboard::Key,
        resolved: iced::keyboard::Key,
    },
    KeyReleased(iced::keyboard::Key),
}

pub fn main() -> iced::Result {
    application("Rust Calculator", Calculator::update, Calculator::view)
        .subscription(Calculator::subscription)
        .window(iced::window::Settings {
            size: iced::Size::new(348.0, 542.0),
            resizable: false,
            decorations: true,
            ..Default::default()
        })
        .theme(|_| iced::Theme::Dark)
        .run()
}

impl Calculator {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::KeyboardEvent(key) => {
                // Add key to pressed set for visual feedback
                self.pressed_keys.insert(key.clone());

                // Handle keyboard input by converting to appropriate messages
                if let Some(calc_message) = Self::keyboard_to_message(key) {
                    // Recursively call update with the converted message
                    self.update(calc_message)
                } else {
                    Task::none()
                }
            }
            Message::KeyCombinationPressed { original, resolved } => {
                // Store the mapping for proper release handling
                self.key_mapping.insert(original, resolved.clone());

                // Add the resolved key to pressed set for visual feedback
                self.pressed_keys.insert(resolved.clone());

                // Handle keyboard input by converting to appropriate messages
                if let Some(calc_message) = Self::keyboard_to_message(resolved) {
                    // Recursively call update with the converted message
                    self.update(calc_message)
                } else {
                    Task::none()
                }
            }
            Message::KeyReleased(key) => {
                // Check if this key was part of a key combination
                if let Some(resolved_key) = self.key_mapping.get(&key) {
                    // This was part of a combination, remove the resolved key
                    self.pressed_keys.remove(resolved_key);
                    // Also remove the mapping since the combination is released
                    self.key_mapping.remove(&key);
                } else {
                    // Regular key release
                    self.pressed_keys.remove(&key);
                }
                Task::none()
            }
            // Handle all other messages normally
            _ => {
                // Convert GUI message to UI state message
                let ui_message = match message {
                    Message::NumberPressed(digit) => UIMessage::NumberPressed(digit),
                    Message::DecimalPressed => UIMessage::DecimalPressed,
                    Message::OperationPressed(operation) => UIMessage::OperationPressed(operation),
                    Message::EqualsPressed => UIMessage::EqualsPressed,
                    Message::ClearPressed => UIMessage::ClearPressed,
                    Message::BackspacePressed => UIMessage::BackspacePressed,
                    Message::PercentagePressed => UIMessage::PercentagePressed,
                    Message::SignTogglePressed => UIMessage::SignTogglePressed,
                    Message::KeyboardEvent(_)
                    | Message::KeyCombinationPressed { .. }
                    | Message::KeyReleased(_) => {
                        unreachable!("Keyboard events handled above")
                    }
                };

                // Process the message using the extracted UI state logic
                let result = self.ui_state.process_message(ui_message);

                match result {
                    MessageResult::ScrollToEnd => scrollable::scroll_to(
                        DISPLAY_SCROLL_ID.clone(),
                        scrollable::AbsoluteOffset {
                            x: f32::INFINITY, // Scroll to the rightmost position (end/latest input)
                            y: 0.0,
                        },
                    ),
                    MessageResult::NoScroll => Task::none(),
                }
            }
        }
    }

    /// Converts keyboard input to calculator messages
    fn keyboard_to_message(key: iced::keyboard::Key) -> Option<Message> {
        match key {
            // Number keys and operators from character input
            keyboard::Key::Character(ch) => match ch.as_str() {
                "0" => Some(Message::NumberPressed(0)),
                "1" => Some(Message::NumberPressed(1)),
                "2" => Some(Message::NumberPressed(2)),
                "3" => Some(Message::NumberPressed(3)),
                "4" => Some(Message::NumberPressed(4)),
                "5" => Some(Message::NumberPressed(5)),
                "6" => Some(Message::NumberPressed(6)),
                "7" => Some(Message::NumberPressed(7)),
                "8" => Some(Message::NumberPressed(8)),
                "9" => Some(Message::NumberPressed(9)),
                "+" => Some(Message::OperationPressed(Operation::Add)),
                "-" => Some(Message::OperationPressed(Operation::Subtract)),
                "*" | "x" | "X" => Some(Message::OperationPressed(Operation::Multiply)),
                "/" | "÷" => Some(Message::OperationPressed(Operation::Divide)),
                "." => Some(Message::DecimalPressed),
                "%" => Some(Message::PercentagePressed),
                "±" => Some(Message::SignTogglePressed), // Special marker for sign toggle (option + -)
                _ => None,
            },
            // Named keys
            keyboard::Key::Named(named_key) => match named_key {
                keyboard::key::Named::Enter => Some(Message::EqualsPressed),
                keyboard::key::Named::Backspace => Some(Message::BackspacePressed),
                keyboard::key::Named::Escape => Some(Message::ClearPressed),
                _ => None,
            },
            _ => None,
        }
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        event::listen_with(|event, _status, _window| match event {
            iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                // Handle key combinations based on modifiers
                let effective_key = Self::resolve_key_combination(key.clone(), modifiers);

                // Store the mapping from original key to resolved key for proper release handling
                if effective_key != key {
                    // This was a key combination, store the mapping
                    Some(Message::KeyCombinationPressed {
                        original: key,
                        resolved: effective_key,
                    })
                } else {
                    // Regular key press
                    Some(Message::KeyboardEvent(effective_key))
                }
            }
            iced::Event::Keyboard(keyboard::Event::KeyReleased { key, .. }) => {
                // Send KeyReleased to reset visual feedback
                Some(Message::KeyReleased(key))
            }
            _ => None,
        })
    }

    /// Resolve key combinations based on modifiers according to user story specifications
    fn resolve_key_combination(
        key: iced::keyboard::Key,
        modifiers: keyboard::Modifiers,
    ) -> iced::keyboard::Key {
        // Handle specific key combinations as defined in the user story
        match (key, modifiers.shift(), modifiers.alt()) {
            // % is mapped to shift + 5
            (keyboard::Key::Character(ch), true, _) if ch == "5" => {
                keyboard::Key::Character("%".into())
            }
            // * is mapped to shift + 8
            (keyboard::Key::Character(ch), true, _) if ch == "8" => {
                keyboard::Key::Character("*".into())
            }
            // + is mapped to shift + =
            (keyboard::Key::Character(ch), true, _) if ch == "=" => {
                keyboard::Key::Character("+".into())
            }
            // +/- is mapped to option + -
            (keyboard::Key::Character(ch), _, true) if ch == "-" => {
                // For +/-, we need to handle this differently since it's a special button
                // We'll return a special marker that keyboard_to_message can handle
                keyboard::Key::Character("±".into()) // Using ± as a marker for sign toggle
            }
            // Return the key as-is for all other combinations
            (key, _, _) => key,
        }
    }

    /// Check if a specific key is currently pressed
    fn is_key_pressed(&self, key: &iced::keyboard::Key) -> bool {
        self.pressed_keys.contains(key)
    }

    fn view(&self) -> Element<'_, Message> {
        // Shared width value for display and keyboard - matches button row width: 4*70 + 3*12 = 316
        let content_width = 316.0;

        let display_text = self.ui_state.calculator.display_string();

        // Result display – horizontally scrollable with invisible scrollbar
        let display_content = scrollable(
            text(display_text)
                .size(52.0)
                .align_x(iced::alignment::Horizontal::Right),
        )
        .id(DISPLAY_SCROLL_ID.clone())
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::new().width(0).scroller_width(0),
        ))
        .width(content_width)
        .height(80.0);

        let display = container(display_content)
            .width(content_width)
            .height(80.0)
            .center_x(iced::Length::Shrink);

        // Button grid – exactly same width
        let keyboard = column![
            // Row 1: ⌫ AC % ÷
            row![
                function_button(
                    "⌫",
                    Message::BackspacePressed,
                    self.is_key_pressed(&keyboard::Key::Named(keyboard::key::Named::Backspace))
                ),
                function_button(
                    "AC",
                    Message::ClearPressed,
                    self.is_key_pressed(&keyboard::Key::Named(keyboard::key::Named::Escape))
                ),
                function_button(
                    "%",
                    Message::PercentagePressed,
                    self.is_key_pressed(&keyboard::Key::Character("%".into()))
                ),
                operator_button(
                    "÷",
                    Message::OperationPressed(Operation::Divide),
                    self.is_key_pressed(&keyboard::Key::Character("/".into()))
                ),
            ]
            .spacing(12.0),
            // Row 2: 7 8 9 x
            row![
                number_button(
                    "7",
                    Message::NumberPressed(7),
                    self.is_key_pressed(&keyboard::Key::Character("7".into()))
                ),
                number_button(
                    "8",
                    Message::NumberPressed(8),
                    self.is_key_pressed(&keyboard::Key::Character("8".into()))
                ),
                number_button(
                    "9",
                    Message::NumberPressed(9),
                    self.is_key_pressed(&keyboard::Key::Character("9".into()))
                ),
                operator_button(
                    "x",
                    Message::OperationPressed(Operation::Multiply),
                    self.is_key_pressed(&keyboard::Key::Character("*".into()))
                        || self.is_key_pressed(&keyboard::Key::Character("x".into()))
                ),
            ]
            .spacing(12.0),
            // Row 3: 4 5 6 −
            row![
                number_button(
                    "4",
                    Message::NumberPressed(4),
                    self.is_key_pressed(&keyboard::Key::Character("4".into()))
                ),
                number_button(
                    "5",
                    Message::NumberPressed(5),
                    self.is_key_pressed(&keyboard::Key::Character("5".into()))
                ),
                number_button(
                    "6",
                    Message::NumberPressed(6),
                    self.is_key_pressed(&keyboard::Key::Character("6".into()))
                ),
                operator_button(
                    "−",
                    Message::OperationPressed(Operation::Subtract),
                    self.is_key_pressed(&keyboard::Key::Character("-".into()))
                ),
            ]
            .spacing(12.0),
            // Row 4: 1 2 3 +
            row![
                number_button(
                    "1",
                    Message::NumberPressed(1),
                    self.is_key_pressed(&keyboard::Key::Character("1".into()))
                ),
                number_button(
                    "2",
                    Message::NumberPressed(2),
                    self.is_key_pressed(&keyboard::Key::Character("2".into()))
                ),
                number_button(
                    "3",
                    Message::NumberPressed(3),
                    self.is_key_pressed(&keyboard::Key::Character("3".into()))
                ),
                operator_button(
                    "+",
                    Message::OperationPressed(Operation::Add),
                    self.is_key_pressed(&keyboard::Key::Character("+".into()))
                ),
            ]
            .spacing(12.0),
            // Row 5: +/- 0 . =
            row![
                function_button(
                    "+/-",
                    Message::SignTogglePressed,
                    self.is_key_pressed(&keyboard::Key::Character("±".into()))
                ),
                number_button(
                    "0",
                    Message::NumberPressed(0),
                    self.is_key_pressed(&keyboard::Key::Character("0".into()))
                ),
                number_button(
                    ".",
                    Message::DecimalPressed,
                    self.is_key_pressed(&keyboard::Key::Character(".".into()))
                ),
                operator_button(
                    "=",
                    Message::EqualsPressed,
                    self.is_key_pressed(&keyboard::Key::Named(keyboard::key::Named::Enter))
                ),
            ]
            .spacing(12.0),
        ]
        .spacing(12.0)
        .align_x(iced::Alignment::Center)
        .width(content_width);

        // Combine both and center the whole group horizontally
        let main_content = column![display, keyboard]
            .spacing(32.0)
            .align_x(iced::Alignment::Center);

        container(main_content)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .padding(16)
            .into()
    }
}

/// Convenience functions for different button types following the example pattern
/// All buttons now use the same size: 70x70 with padding 16
fn number_button(label: &str, on_press: Message, pressed: bool) -> Element<'_, Message> {
    let (background_color, border_width) = if pressed {
        (iced::Color::from_rgb8(100, 100, 102), 2.0) // Lighter color and thicker border when pressed
    } else {
        (iced::Color::from_rgb8(44, 44, 46), 0.0) // Normal color and no border
    };

    button(
        text(label)
            .size(24.0)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .on_press(on_press)
    .padding(16.0)
    .width(70.0)
    .height(70.0)
    .style(move |theme: &Theme, _status| button::Style {
        background: Some(iced::Background::Color(background_color)),
        text_color: theme.palette().text,
        border: iced::Border {
            color: iced::Color::from_rgb8(255, 255, 255),
            width: border_width,
            radius: 30.0.into(),
        },
        ..Default::default()
    })
    .into()
}

fn operator_button(label: &str, on_press: Message, pressed: bool) -> Element<'_, Message> {
    let (background_color, border_width) = if pressed {
        (iced::Color::from_rgb8(255, 180, 50), 2.0) // Lighter orange color and thicker border when pressed
    } else {
        (iced::Color::from_rgb8(255, 149, 0), 0.0) // Normal orange color and no border
    };

    button(
        text(label)
            .size(24.0)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .on_press(on_press)
    .padding(16.0)
    .width(70.0)
    .height(70.0)
    .style(move |theme: &Theme, _status| button::Style {
        background: Some(iced::Background::Color(background_color)),
        text_color: theme.palette().text,
        border: iced::Border {
            color: iced::Color::from_rgb8(255, 255, 255),
            width: border_width,
            radius: 30.0.into(),
        },
        ..Default::default()
    })
    .into()
}

fn function_button(label: &str, on_press: Message, pressed: bool) -> Element<'_, Message> {
    let (background_color, border_width) = if pressed {
        (iced::Color::from_rgb8(100, 100, 102), 2.0) // Lighter gray color and thicker border when pressed
    } else {
        (iced::Color::from_rgb8(58, 58, 60), 0.0) // Normal gray color and no border
    };

    button(
        text(label)
            .size(20.0)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center),
    )
    .on_press(on_press)
    .padding(16.0)
    .width(70.0)
    .height(70.0)
    .style(move |theme: &Theme, _status| button::Style {
        background: Some(iced::Background::Color(background_color)),
        text_color: theme.palette().text,
        border: iced::Border {
            color: iced::Color::from_rgb8(255, 255, 255),
            width: border_width,
            radius: 30.0.into(),
        },
        ..Default::default()
    })
    .into()
}
