#![allow(clippy::unnecessary_wraps)]

use ggez::{event, glam::*, graphics::{self, Color, DrawParam}, Context, GameResult, graphics::Image, GameError};
use ggez::event::MouseButton;

#[derive(Clone)]
enum Figure {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

enum movement_type {
    Unlimited,
    Limited
}

impl Figure{
    fn get_moves(&self, ) -> Color {
        match self {
            Figure::King => [[[1, 1], [0, 1], [1, 0], [-1, 0], [-1, 1], [-1, -1], [1, -1], [0, -1]], [movement_type::Limited]],
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
            data: vec![vec![Some(PlayerFigure::Black(Figure::Rook)), Some(PlayerFigure::Black(Figure::Knight)), Some(PlayerFigure::Black(Figure::Bishop)), Some(PlayerFigure::Black(Figure::King)), Some(PlayerFigure::Black(Figure::Queen)), Some(PlayerFigure::Black(Figure::Bishop)), Some(PlayerFigure::Black(Figure::Knight)), Some(PlayerFigure::Black(Figure::Rook))],
                vec![Some(PlayerFigure::Black(Figure::Pawn)); 8], 
                vec![None; 8], 
                vec![None; 8], 
                vec![None; 8], 
                vec![None; 8], 
                vec![Some(PlayerFigure::White(Figure::Pawn)); 8], 
                vec![Some(PlayerFigure::White(Figure::Rook)), Some(PlayerFigure::White(Figure::Knight)), Some(PlayerFigure::White(Figure::Bishop)), Some(PlayerFigure::White(Figure::Queen)), Some(PlayerFigure::White(Figure::King)), Some(PlayerFigure::White(Figure::Bishop)), Some(PlayerFigure::White(Figure::Knight)), Some(PlayerFigure::White(Figure::Rook))]],
            is_whites_turn: true,
        }
    }
}

struct App {
    board: Board,
    sel_piece_x: Option<i32>,
    sel_piece_y: Option<i32>,
}

impl App {
    fn new(ctx: &mut Context) -> GameResult<App> {
        Ok(App { board: Board::new(),
            sel_piece_x: None,
            sel_piece_y: None,
        })
    }
}


impl event::EventHandler<ggez::GameError> for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([186. / 255., 140. / 255., 99. / 255., 1.0]));

        
        canvas.finish(ctx)?;

        Ok(())
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult {
        
        
        println!("Current selected coords: X: {}, Y: {}, Is it whites turn? {}", self.sel_piece_x.unwrap(), self.sel_piece_y.unwrap(), self.board.is_whites_turn);
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::context::ContextBuilder::new("Chess", "malanak");
    let (mut ctx, event_loop) = cb.build()?;
    let app = App::new(&mut ctx)?;
    event::run(ctx, event_loop, app)
}

//CREDITS: Chess pieces + chess board looks: JohnPablok's improved Cburnett chess set. (https://creativecommons.org/licenses/by-sa/3.0/)