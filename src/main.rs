// source "$HOME/.cargo/env"

use iced::widget::button::{self, Appearance, StyleSheet};
use iced::widget::svg;
use iced::widget::{container, Button, Column, Row, Text};
use iced::{
    executor, Alignment, Application, Background, BorderRadius, Color, Command, Element, Length,
    Settings,
};

pub fn main() -> iced::Result {
    ChessBoard::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Pressed,
}

struct ChessBoard;

struct ChessSquare {
    col: u8,
    row: u8,
}

impl ChessSquare {
    fn new(col: u8, row: u8) -> Self {
        Self { col, row }
    }

    fn get_bg_color(&self) -> Option<Background> {
        if (self.row + self.col) % 2 == 0 {
            return Some(Background::Color(Color::new(0.8, 0.718, 0.682, 1.0)));
        }

        Some(Background::Color(Color::new(0.439, 0.4, 0.467, 1.0)))
    }
}

impl StyleSheet for ChessSquare {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            // background: Some(Background::Color(Color::new(0.439, 0.4, 0.467, 1.0))),
            background: self.get_bg_color(),
            border_radius: BorderRadius::from(0.0),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: Color::new(1.0, 0.0, 0.0, 1.0),
        }
    }
}

impl Application for ChessBoard {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (ChessBoard {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Styling 3")
    }

    fn update(&mut self, _message: Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let handle = svg::Handle::from_path(format!(
            "{}/icon/Chess_klt45.svg",
            env!("CARGO_MANIFEST_DIR")
        ));

        let mut col = Column::new().spacing(0).align_items(Alignment::Center);
        for i in 0..8 {
            let mut row = Row::new().spacing(0).align_items(Alignment::Center);
            for j in 0..8 {
                row = row.push(
                    Button::new(svg(handle.clone()).width(Length::Fill).height(Length::Fill))
                        .width(Length::Fixed(50.0))
                        .height(Length::Fixed(50.0))
                        .style(iced::theme::Button::Custom(Box::new(ChessSquare::new(
                            i, j,
                        )))),
                );
            }

            col = col.push(row);
        }

        container(col)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Self::Theme::default()
    }
}
