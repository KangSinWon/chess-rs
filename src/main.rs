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

    // new...
    selected: Option<chess::Square>,
    selected_piece: Option<chess::Piece>,
    selccted_color: Option<chess::Color>,
    movable_squares: BitBoard,
}

impl ChessBoard {
    fn get_movable_squares(
        &self,
        square: chess::Square,
        piece: chess::Piece,
        color: chess::Color,
    ) -> BitBoard {
        let moves: BitBoard;
        match piece {
            chess::Piece::Pawn => {
                moves = chess::get_pawn_moves(square, color, *self.board.combined())
                    & !self.board.color_combined(color);
            }
            chess::Piece::Rook => {
                moves = chess::get_rook_moves(square, *self.board.combined())
                    & !self.board.color_combined(color);
            }
            chess::Piece::Knight => {
                moves = chess::get_knight_moves(square) & !self.board.color_combined(color);
            }
            chess::Piece::Bishop => {
                moves = chess::get_bishop_moves(square, *self.board.combined())
                    & !self.board.color_combined(color);
            }
            chess::Piece::Queen => {
                let rook_moves = chess::get_rook_moves(square, *self.board.combined())
                    & !self.board.color_combined(color);
                let bishop_moves = chess::get_bishop_moves(square, *self.board.combined())
                    & !self.board.color_combined(color);
                moves = rook_moves | bishop_moves;
            }
            chess::Piece::King => {
                moves = chess::get_king_moves(square) & !self.board.color_combined(color);
            }
        }

        moves
    }

    fn update_ui_squares(&mut self, movable_squares: BitBoard) {
        for square in chess::ALL_SQUARES {
            let p = self.board.piece_on(square);
            let c = self.board.color_on(square);

            let square_int = square.to_int();
            let square_bitboard = chess::BitBoard::from_square(square);
            let mut square_type = ui::SquareType::None;

            if p.is_some() {
                square_type = ui::SquareType::Piece;
            } else if square_bitboard & movable_squares == square_bitboard {
                square_type = ui::SquareType::Moveable;
            }
            self.squares[square_int as usize].update(square_type, p, c);
        }
    }
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
                selected: Option::None,
                selected_piece: Option::None,
                selccted_color: Option::None,
                movable_squares: BitBoard(0),
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
            ui::Message::Square(square) => {
                let turn = self.board.side_to_move();
                println!("{:?} -> Pressed: {}", turn, square);

                let p = self.board.piece_on(square);
                let c = self.board.color_on(square);

                let mut new_selected = Option::None;
                let mut new_selected_piece = Option::None;
                let mut new_selected_color = Option::None;
                let mut movable_squares: chess::BitBoard = chess::BitBoard(0);

                if (c.is_some() && turn == c.unwrap())
                    || (self.selccted_color.is_some() && turn == self.selccted_color.unwrap())
                {
                    if self.selected.is_none() {
                        if p.is_some() {
                            new_selected = Some(square);
                            new_selected_piece = p;
                            new_selected_color = c;
                            movable_squares =
                                self.get_movable_squares(square, p.unwrap(), c.unwrap());
                        }
                    } else {
                        if self.selected_piece.unwrap() == chess::Piece::Pawn {
                            let attack_sqaures = chess::get_pawn_attacks(
                                self.selected.unwrap(),
                                self.board.side_to_move(),
                                *self.board.color_combined(self.board.side_to_move()),
                            );
                            self.movable_squares = self.movable_squares | attack_sqaures;
                        }

                        let square_bitboard = chess::BitBoard::from_square(square);
                        if square_bitboard & self.movable_squares == square_bitboard {
                            let m = chess::ChessMove::new(self.selected.unwrap(), square, None);
                            self.board = self.board.make_move_new(m);
                        } else {
                            if p.is_some() && (turn == c.unwrap()) {
                                new_selected = Some(square);
                                new_selected_piece = p;
                                new_selected_color = c;
                                movable_squares =
                                    self.get_movable_squares(square, p.unwrap(), c.unwrap());
                            }
                        }
                    }
                }

                self.selected = new_selected;
                self.selected_piece = new_selected_piece;
                self.selccted_color = new_selected_color;
                self.movable_squares = movable_squares;

                // update squares (board, movable_squares)
                self.update_ui_squares(self.movable_squares);
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
