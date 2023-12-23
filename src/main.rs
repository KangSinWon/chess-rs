// source "$HOME/.cargo/env"

use iced::widget::{container, Column, Row};
use iced::{executor, window, Alignment, Application, Command, Element, Length, Settings};

mod ui;

use chess::{BitBoard, Board, Square, ALL_SQUARES};

pub const NUM_PIECES: usize = 6;
pub const ROW_SIZE: usize = 8;
pub const COL_SIZE: usize = 8;

pub fn main() -> iced::Result {
    let window_setting: Settings<()> = iced::settings::Settings {
        window: window::Settings {
            size: (ui::SQUARE_SIZE as u32 * 8, ui::SQUARE_SIZE as u32 * 8),
            resizable: (false),
            decorations: (true),
            ..Default::default()
        },
        ..Default::default()
    };

    ChessBoard::run(window_setting)
}

#[derive(Clone, Copy)]
struct ChessBoard {
    squares: [ui::button::chessButton::BoardSquare; ALL_SQUARES.len()],
    board: Board,
}

impl Application for ChessBoard {
    type Executor = executor::Default;
    type Message = ui::Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut squares: [ui::button::chessButton::BoardSquare; ALL_SQUARES.len()] =
            [ui::button::chessButton::BoardSquare::default(); ALL_SQUARES.len()];
        let board = chess::Board::default();

        for pos in ALL_SQUARES {
            let piece = board.piece_on(pos);
            let color = board.color_on(pos);
            let mut square_type = ui::SquareType::None;

            if piece != Option::None && color != Option::None {
                square_type = ui::SquareType::Piece;
            }

            squares[pos.to_int() as usize] =
                ui::button::chessButton::BoardSquare::new(square_type, pos, piece, color);
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

    // TODO: update Squares (movable)
    fn update(&mut self, _message: ui::Message) -> Command<Self::Message> {
        match _message {
            ui::Message::Square(sq) => {
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

    fn view(&self) -> Element<ui::Message> {
        let mut board_col = Column::new().spacing(0).align_items(Alignment::Center);

        for i in [56, 48, 40, 32, 24, 16, 8, 0] {
            let mut board_row = Row::new().spacing(0).align_items(Alignment::Center);
            let mut cnt = 0;
            while cnt < 8 {
                let pos = i + cnt;
                board_row = board_row.push(ui::button::chessButton::get_button(self.squares[pos]));

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
