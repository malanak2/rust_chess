#![allow(clippy::unnecessary_wraps)]

use std::time::Instant;

use ggez::{event, glam::*, graphics::{self, Color, DrawParam, GraphicsContext}, Context, GameResult, graphics::Image, GameError};
use ggez::event::MouseButton;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

#[derive(Copy, Clone)]
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
    fn get_moves(&self, ) -> [[[i32; 2]; 8]; 1] { // movement_type
        match self {
            Figure::King => [[[1, 1], [0, 1], [1, 0], [-1, 0], [-1, 1], [-1, -1], [1, -1], [0, -1]]],
            Figure::Queen => [[[1, 1], [0, 1], [1, 0], [-1, 0], [-1, 1], [-1, -1], [1, -1], [0, -1]]],
            Figure::Bishop => [[[1, 1], [0, 1], [1, 0], [-1, 0], [-1, 1], [-1, -1], [1, -1], [0, -1]]],
            Figure::Knight => [[[1, 1], [0, 1], [1, 0], [-1, 0], [-1, 1], [-1, -1], [1, -1], [0, -1]]],
            Figure::Rook => [[[1, 1], [0, 1], [1, 0], [-1, 0], [-1, 1], [-1, -1], [1, -1], [0, -1]]],
            Figure::Pawn => [[[1, 1], [0, 1], [1, 0], [-1, 0], [-1, 1], [-1, -1], [1, -1], [0, -1]]],
        }
    }
    fn get_texture(&self) -> graphics::Text {
        match &self {
            Figure::King => graphics::Text::new("K"),
            Figure::Queen => graphics::Text::new("Q"),
            Figure::Bishop => graphics::Text::new("B"),
            Figure::Knight => graphics::Text::new("N"),
            Figure::Rook => graphics::Text::new("R"),
            Figure::Pawn => graphics::Text::new("P"),
        }
    }
}
#[derive(Copy, Clone)]
enum PlayerFigure {
    White(Figure),
    Black(Figure),
}
#[derive(Clone)]
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
    time_since_start: Instant,
    picked_up_piece: Option<PlayerFigure>,
    curr_turn: bool,
    curr_piece_col: Option<bool>,
}

impl App {
    fn new(ctx: &mut Context) -> GameResult<App> {
        Ok(App { board: Board::new(),
            sel_piece_x: None,
            sel_piece_y: None,
            time_since_start: Instant::now(),
            picked_up_piece: None,
            curr_turn: true,
            curr_piece_col: None,
        })
    }
}


impl event::EventHandler<ggez::GameError> for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
        let h: f32 = f32::sin(self.time_since_start.elapsed().as_secs_f32())*height;
        let m_coords = ctx.mouse.position();
        for y in 0..8 {
            for x in 0..8 {
                    canvas.draw(&graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new((width / 8.0) * x as f32, (height / 8.0) * y as f32, width / 8.0, height / 8.0),
                        if (x + y) % 2 == 0 {Color::RED} else {Color::BLACK},
                    )?, Vec2::new(0.0, 0.0));
                    //println!("{} {} WHITE", (width / 8.0) * x as f32, (height / 8.0) * i as f32);
            }
        }
        for y in 0..8 {
            for x in 0..8 {
                let figure = &self.board.data[x as usize][y as usize];
                if let Some(Figure) = figure {
                    let Some(player_figure) = &self.board.data[x as usize][y as usize] else {
                        continue;
                    };
                    let color;
                    let figure = match player_figure {
                        PlayerFigure::Black(figure) => {
                            color = Color::BLACK;
                            figure
                        }
                        PlayerFigure::White(figure) => {
                            color = Color::WHITE;
                            figure
                        }
                    };

                    canvas.draw(figure.get_texture().add(if color == Color::BLACK {"b"} else {"w"}).set_scale(50.0), Vec2::new((width / 8.0) * y as f32, (height / 8.0) * x as f32));
                }
                
                
            }
        }
        let figure = &self.picked_up_piece;
        if figure.is_none() {

        } else {
            let color;
            let u_figure = match figure {
                Some(PlayerFigure::Black(figure)) => {
                    color = Color::BLACK;
                    figure
                }
                Some(PlayerFigure::White(figure)) => {
                    color = Color::WHITE;
                    figure
                }
                None => {
                    color = Color::WHITE;
                    &Figure::Bishop
                }
            };
            canvas.draw(u_figure.get_texture().add(if color == Color::BLACK {"b"} else {"w"}).set_scale(50.0), m_coords);
        }
        canvas.draw(&graphics::Mesh::new_rectangle(
            ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, 20.0, h),
                Color::RED,
            )?, Vec2::new(0.0, 0.0));
        canvas.finish(ctx)?;

        Ok(())
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult {
        let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
        let (mut g_x,mut g_y) = (width / 8.0, height / 8.0);
        let (mut grid_x,mut grid_y) = (_y as i32 / g_y as i32, _x as i32 / g_x as i32);
        if grid_x == 8 {
            grid_x = 7;
        }
        if grid_y == 8 {
            grid_y = 7;
        }
        let figure = self.board.data[grid_x as usize][grid_y as usize];
        if self.picked_up_piece.is_none() {
            if figure.is_none() {
                println!("None at {} {}", grid_x, grid_y)
            } else {
                if self.curr_turn == true { // and is white figure
                    self.sel_piece_x = Some(grid_x);
                    self.sel_piece_y = Some(grid_y);
                    self.picked_up_piece = figure;
                } else if self.curr_turn != true { // and is black figure
                    self.sel_piece_x = Some(grid_x);
                    self.sel_piece_y = Some(grid_y);
                    self.picked_up_piece = figure;
                }
            }
        }
        else if figure.is_none() {
            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
            self.board.data[grid_x as usize][grid_y as usize] = figure;
            self.sel_piece_x = None;
            self.sel_piece_y = None;
            self.picked_up_piece = None;
            self.curr_turn = !self.curr_turn;
            println!("None at {} {}", grid_x, grid_y);
        } else {
            self.sel_piece_x = Some(grid_x);
            self.sel_piece_y = Some(grid_y);
            self.picked_up_piece = figure;
            println!("Picked up piece at {} {}", grid_x, grid_y)
        }
        Ok(())
        //println!("Current selected coords: X: {}, Y: {}, Is it whites turn? {}", self.sel_piece_x.unwrap(), self.sel_piece_y.unwrap(), self.board.is_whites_turn);
        //Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::context::ContextBuilder::new("Chess", "malanak")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (mut ctx, event_loop) = cb.build()?;
    let app = App::new(&mut ctx)?;
    event::run(ctx, event_loop, app)
}

//CREDITS: Chess pieces + chess board looks: JohnPablok's improved Cburnett chess set. (https://creativecommons.org/licenses/by-sa/3.0/)