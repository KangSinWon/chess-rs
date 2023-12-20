// source "$HOME/.cargo/env"

use core::fmt;

use iced::widget::button::{self, Appearance, StyleSheet};
use iced::widget::{container, Button, Column, Row, Text};
use iced::widget::{svg, Svg};
use iced::{
    executor, Alignment, Application, Background, BorderRadius, Color, Command, Element, Length,
    Settings,
};

pub const SQUARE_SIZE: u16 = 50;
pub const NUM_PIECES: usize = 6;
pub const ROW_SIZE: usize = 8;
pub const COL_SIZE: usize = 8;

pub fn main() -> iced::Result {
    ChessBoard::run(Settings::default())
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    None,
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

impl Piece {
    fn as_str(&self) -> &'static str {
        match self {
            Piece::Pawn => "pawn",
            Piece::Rook => "rook",
            Piece::Knight => "knight",
            Piece::Bishop => "bishop",
            Piece::Queen => "queen",
            Piece::King => "king",
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ChessSquare {
    col: u8,
    row: u8,
    piece: Option<Piece>,
    color: Option<PieceColor>,
    button_state: button::State,
}

impl Default for ChessSquare {
    fn default() -> Self {
        ChessSquare {
            col: 0,
            row: 0,
            piece: Option::None,
            color: Option::None,
            button_state: button::State::default(),
        }
    }
}

impl ChessSquare {
    fn new(
        col: u8,
        row: u8,
        piece: Option<Piece>,
        color: Option<PieceColor>,
        button_state: button::State,
    ) -> Self {
        Self {
            col,
            row,
            piece,
            color,
            button_state,
        }
    }

    fn get_bg_color(&self) -> Option<Background> {
        if (self.row + self.col) % 2 == 0 {
            return Some(Background::Color(Color::new(0.8, 0.718, 0.682, 1.0)));
        }

        Some(Background::Color(Color::new(0.439, 0.4, 0.467, 1.0)))
    }

    fn get_icon(&self) -> Svg {
        let icon_path = match self.piece {
            Some(piece) => match self.color {
                Some(color) => match color {
                    PieceColor::White => format!(
                        "{}/icon/white/{}.svg",
                        env!("CARGO_MANIFEST_DIR"),
                        piece.as_str()
                    ),
                    PieceColor::Black => format!(
                        "{}/icon/black/{}.svg",
                        env!("CARGO_MANIFEST_DIR"),
                        piece.as_str()
                    ),
                },
                None => format!("{}/icon/empty.svg", env!("CARGO_MANIFEST_DIR")),
            },
            None => format!("{}/icon/empty.svg", env!("CARGO_MANIFEST_DIR")),
        };

        svg(svg::Handle::from_path(icon_path))
            .width(Length::Fill)
            .height(Length::Fill)
    }
}

impl StyleSheet for ChessSquare {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: self.get_bg_color(),
            border_radius: BorderRadius::from(0.0),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: Color::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    fn pressed (&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: Some(Background::Color(Color::new(0.0, 0.0, 0.0, 1.0))),
            border_radius: BorderRadius::from(0.0),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: Color::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Copy)]
struct ChessBoard {
    chess_squares: [[ChessSquare; 8]; 8],
}

impl Application for ChessBoard {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut squares: [[ChessSquare; 8]; 8] = [[ChessSquare::default(); 8]; 8];
        for col in 0..COL_SIZE {
            for row in 0..ROW_SIZE {
                let mut p: Option<Piece> = Option::None;
                let mut c: Option<PieceColor> = Option::None;

                if col == 0 || col == COL_SIZE - 1 {
                    if row == 0 || row == ROW_SIZE - 1 {
                        p = Some(Piece::Rook);
                    } else if row == 1 || row == ROW_SIZE - 2 {
                        p = Some(Piece::Knight);
                    } else if row == 2 || row == ROW_SIZE - 3 {
                        p = Some(Piece::Bishop);
                    } else if row == 3 {
                        p = Some(Piece::Queen);
                    } else {
                        p = Some(Piece::King)
                    }

                    if col == 0 {
                        c = Some(PieceColor::Black);
                    } else {
                        c = Some(PieceColor::White);
                    }
                }
                if col == 1 || col == COL_SIZE - 2 {
                    p = Some(Piece::Pawn);

                    if col == 1 {
                        c = Some(PieceColor::Black);
                    } else {
                        c = Some(PieceColor::White);
                    }
                }

                squares[col][row] =
                    ChessSquare::new(col as u8, row as u8, p, c, button::State::default());
            }
        }

        (
            ChessBoard {
                chess_squares: squares,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("chess")
    }

    fn update(&mut self, _message: Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut board_col = Column::new().spacing(0).align_items(Alignment::Center);
        for col in 0..COL_SIZE {
            let mut board_row = Row::new().spacing(0).align_items(Alignment::Center);
            for row in 0..ROW_SIZE {
                board_row = board_row.push(
                    Button::new(self.chess_squares[col][row].get_icon())
                        .width(Length::Fixed(SQUARE_SIZE as f32))
                        .height(Length::Fixed(SQUARE_SIZE as f32))
                        .style(iced::theme::Button::Custom(Box::new(
                            self.chess_squares[col][row],
                        ))),
                );
            }

            board_col = board_col.push(board_row);
        }

        container(board_col)
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
