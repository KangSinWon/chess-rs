pub mod chessButton {
    // For now, to implement a custom native widget you will need to add
    // `iced_native` and `iced_wgpu` to your dependencies.
    //
    // Then, you simply need to define your widget type and implement the
    // `iced_native::Widget` trait with the `iced_wgpu::Renderer`.
    //
    // Of course, you can choose to make the implementation renderer-agnostic,
    // if you wish to, by creating your own `Renderer` trait, which could be
    // implemented by `iced_wgpu` and other renderers.
    use iced::theme::Theme;
    use iced::widget::button::{Appearance, StyleSheet};
    use iced::widget::{canvas, Button};
    use iced::widget::{svg, Svg};
    use iced::{mouse, Background, BorderRadius, Color, Length, Point, Rectangle, Renderer, Size};

    use chess::Square;

    use crate::ui;

    pub fn get_button(_board_square: BoardSquare) -> Button<'static, ui::Message> {
        let btn: Button<'static, ui::Message>;
        match _board_square.square_type {
            ui::SquareType::Piece => {
                btn = Button::new(get_icon(_board_square.piece, _board_square.piece_color))
            }
            ui::SquareType::Moveable => btn = Button::new(canvas(MoveableCircle)),
            ui::SquareType::None => btn = Button::new(iced::widget::Space::new(0, 0)),
        }

        btn.width(Length::Fixed(ui::SQUARE_SIZE as f32))
            .height(Length::Fixed(ui::SQUARE_SIZE as f32))
            .style(iced::theme::Button::Custom(Box::new(_board_square)))
            .on_press(ui::Message::Square(_board_square.position))
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

    #[derive(Debug, Clone, Copy)]
    pub struct BoardSquare {
        square_type: ui::SquareType,
        position: Square,
        bg_color: Option<Background>,
        piece: Option<chess::Piece>,
        piece_color: Option<chess::Color>,
    }

    impl Default for BoardSquare {
        fn default() -> Self {
            BoardSquare {
                square_type: ui::SquareType::None,
                position: Square::A1,
                bg_color: Option::None,
                piece: Option::None,
                piece_color: Option::None,
            }
        }
    }

    impl BoardSquare {
        pub fn new(
            square_type: ui::SquareType,
            position: Square,
            piece: Option<chess::Piece>,
            piece_color: Option<chess::Color>,
        ) -> Self {
            let col = position.to_int() / 8;
            let row = position.to_int() % 8;
            let bg_color = if (row + col) % 2 == 0 {
                Some(Background::Color(Color::new(0.8, 0.718, 0.682, 1.0)))
            } else {
                Some(Background::Color(Color::new(0.439, 0.4, 0.467, 1.0)))
            };

            Self {
                square_type,
                position,
                bg_color,
                piece,
                piece_color,
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

    struct MoveableCircle;
    impl<Message> canvas::Program<Message> for MoveableCircle {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            _theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let cache = canvas::Cache::default();
            let square: canvas::Geometry = cache.draw(renderer, Size::new(50.0, 50.0), |frame| {
                frame.fill(
                    &canvas::Path::circle(Point::new(25.0 - 3.75, 25.0 - 3.75), 7.5),
                    Color::new(1.0, 1.0, 1.0, 0.5),
                );
            });

            vec![square]
        }
    }
}
