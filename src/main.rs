// source "$HOME/.cargo/env"

use iced::widget::button::{self, Appearance, StyleSheet};
use iced::widget::{container, Button, Column, Row, Text};
use iced::widget::{svg, Svg};
use iced::{
    executor, Alignment, Application, Background, BorderRadius, Color, Command, Element, Length,
    Settings,
};

use chess::{Board, Square, ALL_SQUARES};

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
struct BoardSquare {
    row: u8,
    col: u8,
    position: Square,
    piece: Option<chess::Piece>,
    piece_color: Option<chess::Color>,
    bg_color: Option<Background>,
    button_state: button::State,
}

impl Default for BoardSquare {
    fn default() -> Self {
        BoardSquare {
            row: 0,
            col: 0,
            position: Square::A1,
            piece: Option::None,
            piece_color: Option::None,
            bg_color: Option::None,
            button_state: button::State::default(),
        }
    }
}

impl BoardSquare {
    fn new(
        row: u8,
        col: u8,
        position: Square,
        piece: Option<chess::Piece>,
        piece_color: Option<chess::Color>,
        button_state: button::State,
    ) -> Self {
        let bg_color = if (row + col) % 2 == 0 {
            Some(Background::Color(Color::new(0.8, 0.718, 0.682, 1.0)))
        } else {
            Some(Background::Color(Color::new(0.439, 0.4, 0.467, 1.0)))
        };

        Self {
            row,
            col,
            position,
            piece,
            piece_color,
            bg_color,
            button_state,
        }
    }
}

impl StyleSheet for BoardSquare {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: self.bg_color,
            border_radius: BorderRadius::from(0.0),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: Color::new(0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Copy)]
struct ChessBoard {
    squares: [BoardSquare; ALL_SQUARES.len()],
    board: Board,
}

fn get_peice_str(piece: Option<chess::Piece>) -> &'static str {
    match piece {
        Some(p) => match p {
            chess::Piece::Pawn => "pawn",
            chess::Piece::Rook => "rook",
            chess::Piece::Knight => "knight",
            chess::Piece::Bishop => "bishop",
            chess::Piece::Queen => "queen",
            chess::Piece::King => "king",
        },
        None => "none",
    }
}

fn get_color_str(color: Option<chess::Color>) -> &'static str {
    match color {
        Some(c) => match c {
            chess::Color::White => "white",
            chess::Color::Black => "black",
        },
        None => "none",
    }
}

fn get_icon(piece: Option<chess::Piece>, color: Option<chess::Color>) -> Svg {
    let icon_path = if piece != Option::None && color != Option::None {
        format!(
            "{}/icon/{}/{}.svg",
            env!("CARGO_MANIFEST_DIR"),
            get_color_str(color),
            get_peice_str(piece)
        )
    } else {
        format!("{}/icon/empty.svg", env!("CARGO_MANIFEST_DIR"))
    };

    svg(svg::Handle::from_path(icon_path))
        .width(Length::Fill)
        .height(Length::Fill)
}

impl Application for ChessBoard {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut squares: [BoardSquare; ALL_SQUARES.len()] =
            [BoardSquare::default(); ALL_SQUARES.len()];
        let board = chess::Board::default();

        let mut row: u8 = 0;
        let mut col: u8 = 0;
        for sq in ALL_SQUARES {
            let piece = board.piece_on(sq);
            let color = board.color_on(sq);

            squares[(col * 8 + row) as usize] = BoardSquare::new(row, col, sq, piece, color, button::State::default());
            
            row += 1;
            if row == 8 {
                row = 0;
                col += 1;
            }
        }

        (
            ChessBoard {
                squares: squares,
                board: Board::default(),
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

        for i in [56, 48, 40, 32, 24, 16, 8, 0] {
            let mut board_row = Row::new().spacing(0).align_items(Alignment::Center);
            let mut cnt = 0;
            while cnt < 8 {
                board_row = board_row.push(
                    Button::new(get_icon(self.squares[i].piece, self.squares[i].piece_color))
                        .width(Length::Fixed(SQUARE_SIZE as f32))
                        .height(Length::Fixed(SQUARE_SIZE as f32))
                        .style(iced::theme::Button::Custom(Box::new(self.squares[i + cnt]))),
                );

                cnt += 1;
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
