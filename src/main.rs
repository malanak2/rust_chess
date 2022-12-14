#![allow(clippy::unnecessary_wraps)]

use ggez::{
    event,
    glam::*,
    graphics::{self, Color},
    Context, GameResult,
};
#[derive(Clone)]
enum Figure {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Color {
    fn from(color: [f32; 4]) -> Color {
        Color {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3]
        }
    }
}

impl Figure{
    fn color(&self) -> Color {
        match self {
            Figure::King => Color::from([0.0, 0.0, 0.0, 1.0]),
            Figure::Queen => Color::from([0.0, 0.0, 0.0, 1.0]),
            Figure::Bishop => Color::from([0.0, 0.0, 0.0, 1.0]),
            Figure::Knight => Color::from([0.0, 0.0, 0.0, 1.0]),
            Figure::Rook => Color::from([0.0, 0.0, 0.0, 1.0]),
            Figure::Pawn => Color::from([0.0, 0.0, 0.0, 1.0]),
        }
    }
}
#[derive(Clone)]
enum PlayerFigure {
    White(Figure),
    Black(Figure),
}

struct Board {
    data: Vec<Vec<Option<PlayerFigure>>>,
    is_whites_turn: bool,
}
impl Board {
    fn new() -> Board {
        Board {
            data: vec![vec![Some(PlayerFigure::Black(Figure::Rook)), Some(PlayerFigure::Black(Figure::King)), Some(PlayerFigure::Black(Figure::Bishop)), Some(PlayerFigure::Black(Figure::King)), Some(PlayerFigure::Black(Figure::Queen)), Some(PlayerFigure::Black(Figure::Bishop)), Some(PlayerFigure::Black(Figure::King)), Some(PlayerFigure::Black(Figure::Rook))],
                vec![Some(PlayerFigure::Black(Figure::Pawn)); 8], 
                vec![None; 8], 
                vec![None; 8], 
                vec![None; 8], 
                vec![None; 8], 
                vec![Some(PlayerFigure::White(Figure::Pawn)); 8], 
                vec![Some(PlayerFigure::White(Figure::Rook)), Some(PlayerFigure::White(Figure::King)), Some(PlayerFigure::White(Figure::Bishop)), Some(PlayerFigure::White(Figure::Queen)), Some(PlayerFigure::White(Figure::King)), Some(PlayerFigure::White(Figure::Bishop)), Some(PlayerFigure::White(Figure::King)), Some(PlayerFigure::White(Figure::Rook))]],
            is_whites_turn: true,
        }
    }
}

struct MainState {
    board: Board,
}

impl MainState {
    fn new() -> GameResult<MainState> {

        Ok(MainState { board: Board::new() })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([186. / 255., 140. / 255., 99. / 255., 1.0]));

        for i in 0..8 {
            for x in 0..8 {
                canvas.draw(graphics::Rect, Vec2::new())
            }
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}