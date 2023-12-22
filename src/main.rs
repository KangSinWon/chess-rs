// source "$HOME/.cargo/env"

mod circle {
    // For now, to implement a custom native widget you will need to add
    // `iced_native` and `iced_wgpu` to your dependencies.
    //
    // Then, you simply need to define your widget type and implement the
    // `iced_native::Widget` trait with the `iced_wgpu::Renderer`.
    //
    // Of course, you can choose to make the implementation renderer-agnostic,
    // if you wish to, by creating your own `Renderer` trait, which could be
    // implemented by `iced_wgpu` and other renderers.
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::mouse;
    use iced::{Color, Element, Length, Rectangle, Size};

    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Self {
            Self { radius }
        }
    }

    pub fn circle(radius: f32) -> Circle {
        Circle::new(radius)
    }

    impl<Message, Renderer> Widget<Message, Renderer> for Circle
    where
        Renderer: renderer::Renderer,
    {
        fn width(&self) -> Length {
            Length::Fill
        }

        fn height(&self) -> Length {
            Length::Fill
        }

        fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
            layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
        }

        fn draw(
            &self,
            _state: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Renderer::Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border_radius: self.radius.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                },
                Color::new(1.0, 1.0, 1.0, 0.5),
            );
        }
    }

    impl<'a, Message, Renderer> From<Circle> for Element<'a, Message, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(circle: Circle) -> Self {
            Self::new(circle)
        }
    }
}

use circle::circle;

use iced::widget::button::{self, Appearance, StyleSheet};
use iced::widget::{container, Button, Column, Row, Text};
use iced::widget::{svg, Svg};
use iced::{
    executor, window, Alignment, Application, Background, BorderRadius, Color, Command, Element,
    Length, Settings,
};

use chess::{BitBoard, Board, Square, ALL_SQUARES};

pub const SQUARE_SIZE: u16 = 50;
pub const NUM_PIECES: usize = 6;
pub const ROW_SIZE: usize = 8;
pub const COL_SIZE: usize = 8;

pub fn main() -> iced::Result {
    let window_setting: Settings<()> = iced::settings::Settings {
        window: window::Settings {
            size: (SQUARE_SIZE as u32 * 8, SQUARE_SIZE as u32 * 8),
            resizable: (false),
            decorations: (true),
            ..Default::default()
        },
        ..Default::default()
    };

    ChessBoard::run(window_setting)
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Square(chess::Square),
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

    fn pressed(&self, style: &Self::Style) -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: Some(Background::Color(Color::new(0.0, 0.0, 0.0, 0.5))),
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

            squares[(col * 8 + row) as usize] =
                BoardSquare::new(row, col, sq, piece, color, button::State::default());

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
        match _message {
            Message::Square(sq) => {
                let p = self.board.piece_on(sq);
                let c = self.board.color_on(sq);

                if p == Option::None || c == Option::None {
                    return Command::none();
                }

                println!("Pressed: {}", p.unwrap());

                let moves: BitBoard;
                match p.unwrap() {
                    chess::Piece::Pawn => {
                        moves = chess::get_pawn_moves(sq, c.unwrap(), *self.board.combined())
                            & !self.board.color_combined(c.unwrap());
                    }
                    chess::Piece::Rook => {
                        moves = chess::get_rook_moves(sq, *self.board.combined())
                            & !self.board.color_combined(c.unwrap());
                    }
                    chess::Piece::Knight => {
                        moves =
                            chess::get_knight_moves(sq) & !self.board.color_combined(c.unwrap());
                    }
                    chess::Piece::Bishop => {
                        moves = chess::get_bishop_moves(sq, *self.board.combined())
                            & !self.board.color_combined(c.unwrap());
                    }
                    chess::Piece::Queen => {
                        let rook_moves = chess::get_rook_moves(sq, *self.board.combined())
                            & !self.board.color_combined(c.unwrap());
                        let bishop_moves = chess::get_bishop_moves(sq, *self.board.combined())
                            & !self.board.color_combined(c.unwrap());
                        moves = rook_moves | bishop_moves;
                    }
                    chess::Piece::King => {
                        moves = chess::get_king_moves(sq) & !self.board.color_combined(c.unwrap());
                    }
                }
                println!("{}", moves.reverse_colors());
                for s in moves {
                    println!("{}", s);
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let mut board_col = Column::new().spacing(0).align_items(Alignment::Center);

        for i in [56, 48, 40, 32, 24, 16, 8, 0] {
            let mut board_row = Row::new().spacing(0).align_items(Alignment::Center);
            let mut cnt = 0;
            while cnt < 8 {
                let pos = i + cnt;
                board_row = board_row.push(
                    Button::new(iced::widget::Space::new(0, 0))
                        .width(Length::Fixed(SQUARE_SIZE as f32))
                        .height(Length::Fixed(SQUARE_SIZE as f32))
                        .style(iced::theme::Button::Custom(Box::new(self.squares[pos])))
                        .on_press(Message::Square(self.squares[pos].position)),
                    // Button::new(
                    //     container(circle(7.))
                    //         .width(Length::Fixed(SQUARE_SIZE as f32))
                    //         .height(Length::Fixed(SQUARE_SIZE as f32))
                    //         .align_x(iced::alignment::Horizontal::Center)
                    //         .align_y(iced::alignment::Vertical::Center),
                    // )
                    // .width(Length::Fixed(SQUARE_SIZE as f32))
                    // .height(Length::Fixed(SQUARE_SIZE as f32))
                    // .style(iced::theme::Button::Custom(Box::new(self.squares[pos])))
                    // .on_press(Message::Square(self.squares[pos].position)),

                    // Button::new(get_icon(
                    //     self.squares[pos].piece,
                    //     self.squares[pos].piece_color,
                    // ))
                    // .width(Length::Fixed(SQUARE_SIZE as f32))
                    // .height(Length::Fixed(SQUARE_SIZE as f32))
                    // .style(iced::theme::Button::Custom(Box::new(self.squares[pos])))
                    // .on_press(Message::Square(self.squares[pos].position)),
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
