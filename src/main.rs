use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Task, Theme, application};
use rust_calculator::{CalculatorUIState, MessageResult, Operation, UIMessage};
use std::sync::LazyLock;

// Static ID for the display scrollable widget - must be reused for scroll_to to work
static DISPLAY_SCROLL_ID: LazyLock<scrollable::Id> =
    LazyLock::new(|| scrollable::Id::new("display_scroll"));

#[derive(Default)]
struct Calculator {
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

pub fn main() -> iced::Result {
    application("Rust Calculator", Calculator::update, Calculator::view)
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
                function_button("⌫", Message::BackspacePressed),
                function_button("AC", Message::ClearPressed),
                function_button("%", Message::PercentagePressed),
                operator_button("÷", Message::OperationPressed(Operation::Divide)),
            ]
            .spacing(12.0),
            // Row 2: 7 8 9 x
            row![
                number_button("7", Message::NumberPressed(7)),
                number_button("8", Message::NumberPressed(8)),
                number_button("9", Message::NumberPressed(9)),
                operator_button("x", Message::OperationPressed(Operation::Multiply)),
            ]
            .spacing(12.0),
            // Row 3: 4 5 6 −
            row![
                number_button("4", Message::NumberPressed(4)),
                number_button("5", Message::NumberPressed(5)),
                number_button("6", Message::NumberPressed(6)),
                operator_button("−", Message::OperationPressed(Operation::Subtract)),
            ]
            .spacing(12.0),
            // Row 4: 1 2 3 +
            row![
                number_button("1", Message::NumberPressed(1)),
                number_button("2", Message::NumberPressed(2)),
                number_button("3", Message::NumberPressed(3)),
                operator_button("+", Message::OperationPressed(Operation::Add)),
            ]
            .spacing(12.0),
            // Row 5: +/- 0 . =
            row![
                function_button("+/-", Message::SignTogglePressed),
                number_button("0", Message::NumberPressed(0)),
                number_button(".", Message::DecimalPressed),
                operator_button("=", Message::EqualsPressed),
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
fn number_button(label: &str, on_press: Message) -> Element<'_, Message> {
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
    .style(|theme: &Theme, _status| button::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb8(44, 44, 46))),
        text_color: theme.palette().text,
        border: iced::Border {
            radius: 30.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn operator_button(label: &str, on_press: Message) -> Element<'_, Message> {
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
    .style(|theme: &Theme, _status| button::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb8(255, 149, 0))),
        text_color: theme.palette().text,
        border: iced::Border {
            radius: 30.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}

fn function_button(label: &str, on_press: Message) -> Element<'_, Message> {
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
    .style(|theme: &Theme, _status| button::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb8(58, 58, 60))),
        text_color: theme.palette().text,
        border: iced::Border {
            radius: 30.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .into()
}
