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

struct MainState {
    board: Board,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {

        Ok(MainState { board: Board::new() })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([186. / 255., 140. / 255., 99. / 255., 1.0]));

        let (width, height) = graphics::GraphicsContext::size(&ctx.gfx/* &GraphicsContext */);
        let mut is_white: bool = true;
        for i in 0..8 {
            for x in 0..8 {
                /*let curr_coords = &self.board.data[i][x];
                match curr_coords {
                    Some(PlayerFigure::White(_)) => {
                        canvas.draw(&graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect::new(0.0, 0.0, width / 8.0, height / 8.0),
                            Color::WHITE,
                        )?, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                        println!("{} {} WHITE", (width / 8.0) * x as f32, (height / 8.0) * i as f32);
                    }
                    Some(PlayerFigure::Black(_)) => {
                        canvas.draw(&graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect::new(0.0, 0.0, width / 8.0, height / 8.0),
                            Color::BLACK,
                        )?, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                        println!("{} {} BLACK", (width / 8.0) * x as f32, (height / 8.0) * i as f32);
                    }
                    None => {
                        canvas.draw(&graphics::Mesh::new_rectangle(
                            ctx,
                            graphics::DrawMode::fill(),
                            graphics::Rect::new(0.0, 0.0, width / 8.0, height / 8.0),
                            Color::RED,
                        )?, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                        println!("{} {} NONE", (width / 8.0) * x as f32, (height / 8.0) * i as f32);
                    }*/
                if is_white {
                    canvas.draw(&graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, width / 8.0, height / 8.0),
                        Color::WHITE,
                    )?, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    println!("{} {} WHITE", (width / 8.0) * x as f32, (height / 8.0) * i as f32);
                    is_white = false;
                } else {
                    canvas.draw(&graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, width / 8.0, height / 8.0),
                        Color::BLACK,
                    )?, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    println!("{} {} BLACK", (width / 8.0) * x as f32, (height / 8.0) * i as f32);
                    is_white = true;
                }
            }
            if is_white {
                is_white = false;
            } else {
                is_white = true;
            }
        }
        canvas.finish(ctx)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::context::ContextBuilder::new("Chess", "malanak");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}