//! # Calculator GUI Application
//!
//! A graphical calculator application built with Iced.

use iced::alignment::{Horizontal, Vertical};
use iced::widget::container::Appearance as ContainerAppearance;
use iced::widget::scrollable::{Direction, Properties};
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Application, Background, Color, Element, Length, Settings, Theme};
use rust_calculator::{CalculatorUIState, MessageResult, Operation, UIMessage};

/// The main GUI application struct.
pub struct CalculatorApp {
    /// The UI state that manages calculator logic and scroll behavior
    ui_state: CalculatorUIState,
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
}

/// Custom button style structs with 30pt border radius
struct CircularButtonStyle {
    background_color: Color,
}

impl iced::widget::button::StyleSheet for CircularButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(self.background_color)),
            border: iced::Border {
                radius: 30.0.into(), // 30pt border radius as requested
                ..iced::Border::default()
            },
            text_color: Color::WHITE,
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let active = self.active(style);
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(Color {
                r: (self.background_color.r + 0.1).min(1.0),
                g: (self.background_color.g + 0.1).min(1.0),
                b: (self.background_color.b + 0.1).min(1.0),
                a: self.background_color.a,
            })),
            ..active
        }
    }
}

/// Custom scrollable style with hidden scrollbar
struct HiddenScrollbarStyle;

impl iced::widget::scrollable::StyleSheet for HiddenScrollbarStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::scrollable::Appearance {
        iced::widget::scrollable::Appearance {
            container: ContainerAppearance {
                background: None,
                text_color: None,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            scrollbar: iced::widget::scrollable::Scrollbar {
                background: Some(Background::Color(Color::TRANSPARENT)),
                border: iced::Border::default(),
                scroller: iced::widget::scrollable::Scroller {
                    color: Color::TRANSPARENT,
                    border: iced::Border::default(),
                },
            },
            gap: None,
        }
    }

    fn hovered(
        &self,
        _style: &Self::Style,
        _is_mouse_over_scrollbar: bool,
    ) -> iced::widget::scrollable::Appearance {
        self.active(_style)
    }
}

/// Convenience functions for different button types following the example pattern
/// All buttons now use the same size: 70x70 with padding 16
fn number_button(label: &str, on_press: Message) -> Element<'_, Message> {
    button(
        text(label)
            .size(24) // Smaller text to fit better in 70x70 buttons
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .on_press(on_press)
    .padding(16)
    .width(Length::Fixed(70.0))
    .height(Length::Fixed(70.0))
    .style(iced::theme::Button::custom(CircularButtonStyle {
        background_color: Color::from_rgb8(44, 44, 46), // #2C2C2E
    }))
    .into()
}

fn operator_button(label: &str, on_press: Message) -> Element<'_, Message> {
    button(
        text(label)
            .size(24) // Smaller text to fit better in 70x70 buttons
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .on_press(on_press)
    .padding(16)
    .width(Length::Fixed(70.0))
    .height(Length::Fixed(70.0))
    .style(iced::theme::Button::custom(CircularButtonStyle {
        background_color: Color::from_rgb8(255, 149, 0), // #FF9500
    }))
    .into()
}

fn function_button(label: &str, on_press: Message) -> Element<'_, Message> {
    button(
        text(label)
            .size(20) // Even smaller for function buttons with symbols like ⌫
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Center),
    )
    .on_press(on_press)
    .padding(16)
    .width(Length::Fixed(70.0))
    .height(Length::Fixed(70.0))
    .style(iced::theme::Button::custom(CircularButtonStyle {
        background_color: Color::from_rgb8(58, 58, 60), // #3A3A3C
    }))
    .into()
}

impl iced::Application for CalculatorApp {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, iced::Command<Message>) {
        (
            Self {
                ui_state: CalculatorUIState::new(),
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Rust Calculator")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
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
        };

        // Process the message using the extracted UI state logic
        let result = self.ui_state.process_message(ui_message);

        match result {
            MessageResult::ScrollToEnd => self.scroll_to_end(),
            MessageResult::NoScroll => iced::Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Shared width value for display and keyboard - matches button row width: 4*70 + 3*12 = 316
        let content_width = Length::Fixed(316.0);

        let display_text = self.ui_state.calculator.display_string();

        // Result display – scrollable with hidden scrollbar
        let display_content = scrollable(
            text(display_text)
                .size(52)
                .width(Length::Shrink)
                .horizontal_alignment(iced::alignment::Horizontal::Right),
        )
        .id(iced::widget::scrollable::Id::new("display_scroll"))
        .direction(Direction::Horizontal(
            Properties::new()
                .width(0.0) // scrollbar track width = 0
                .scroller_width(0.0), // scroller thumb width = 0
        ))
        .width(content_width)
        .height(Length::Fixed(80.0))
        .style(iced::theme::Scrollable::Custom(Box::new(
            HiddenScrollbarStyle,
        )));

        let display = container(display_content)
            .width(content_width)
            .height(Length::Fixed(80.0))
            .center_x();

        // Button grid – exactly same width
        let keyboard = column![
            // Row 1: ⌫ AC % ÷
            row![
                function_button("⌫", Message::BackspacePressed),
                function_button("AC", Message::ClearPressed),
                function_button("%", Message::PercentagePressed),
                operator_button("÷", Message::OperationPressed(Operation::Divide)),
            ]
            .spacing(12),
            // Row 2: 7 8 9 x
            row![
                number_button("7", Message::NumberPressed(7)),
                number_button("8", Message::NumberPressed(8)),
                number_button("9", Message::NumberPressed(9)),
                operator_button("x", Message::OperationPressed(Operation::Multiply)),
            ]
            .spacing(12),
            // Row 3: 4 5 6 −
            row![
                number_button("4", Message::NumberPressed(4)),
                number_button("5", Message::NumberPressed(5)),
                number_button("6", Message::NumberPressed(6)),
                operator_button("−", Message::OperationPressed(Operation::Subtract)),
            ]
            .spacing(12),
            // Row 4: 1 2 3 +
            row![
                number_button("1", Message::NumberPressed(1)),
                number_button("2", Message::NumberPressed(2)),
                number_button("3", Message::NumberPressed(3)),
                operator_button("+", Message::OperationPressed(Operation::Add)),
            ]
            .spacing(12),
            // Row 5: +/- 0 . =
            row![
                function_button("+/-", Message::SignTogglePressed),
                number_button("0", Message::NumberPressed(0)),
                number_button(".", Message::DecimalPressed),
                operator_button("=", Message::EqualsPressed),
            ]
            .spacing(12),
        ]
        .spacing(12)
        .align_items(Alignment::Center)
        .width(content_width);

        // Combine both and center the whole group horizontally
        let main_content = column![display, keyboard]
            .spacing(32)
            .align_items(Alignment::Center);

        container(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x() // centers everything horizontally
            .center_y() // optional: vertical centering too
            .into()
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::Dark
    }
}

impl CalculatorApp {
    /// Scrolls the display to the end (right side) when content grows
    fn scroll_to_end(&self) -> iced::Command<Message> {
        iced::widget::scrollable::scroll_to(
            iced::widget::scrollable::Id::new("display_scroll"),
            iced::widget::scrollable::AbsoluteOffset {
                x: f32::INFINITY,
                y: 0.0,
            },
        )
    }
}

fn main() -> iced::Result {
    CalculatorApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(380.0, 550.0),
            resizable: false, // Disable window maximization and resizing
            ..Default::default()
        },
        ..Default::default()
    })
}
