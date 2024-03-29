#![allow(clippy::unnecessary_wraps)]

use std::time::Instant;
use std::env;

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

enum MovementType {
    Unlimited,
    Limited,
    Pawn
}

impl Figure{
    fn get_moves(&self, ) -> (Vec<Vec<i32>>, MovementType) { // movement_type
        match self {
            Figure::King => (vec![vec![1, 1], vec![0, 1], vec![1, 0], vec![-1, 0], vec![-1, 1], vec![-1, -1], vec![1, -1], vec![0, -1]], MovementType::Limited),
            Figure::Queen => (vec![vec![1, 1], vec![0, 1], vec![1, 0], vec![-1, 0], vec![-1, 1], vec![-1, -1], vec![1, -1], vec![0, -1]], MovementType::Unlimited),
            Figure::Bishop => (vec![vec![1, 1], vec![-1, 1], vec![-1, -1], vec![1, -1]], MovementType::Unlimited),
            Figure::Knight => (vec![vec![2, 1], vec![-2, 1], vec![2, -1], vec![-2, -1], vec![1, 2], vec![-1, 2], vec![1, -2], vec![-1, -2]], MovementType::Limited),
            Figure::Rook => (vec![vec![0, 1], vec![1, 0], vec![-1, 0], vec![0, -1]], MovementType::Unlimited),
            Figure::Pawn => (vec![vec![1, 0], vec![1, 1], vec![1, -1]], MovementType::Pawn),
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
                vec![None, None, None, None, None, None, None, Some(PlayerFigure::Black(Figure::Pawn))], 
                vec![Some(PlayerFigure::White(Figure::Pawn)); 8], 
                vec![Some(PlayerFigure::White(Figure::Rook)), Some(PlayerFigure::White(Figure::Knight)), Some(PlayerFigure::White(Figure::Bishop)), Some(PlayerFigure::White(Figure::Queen)), Some(PlayerFigure::White(Figure::King)), Some(PlayerFigure::White(Figure::Bishop)), Some(PlayerFigure::White(Figure::Knight)), Some(PlayerFigure::White(Figure::Rook))]],
            is_whites_turn: true,
        }
    }
}

struct App {
    board: Board,
    sel_piece_data: (Option<i32>, Option<i32>, Option<PlayerFigure>),
    time_since_start: Instant,
    curr_turn: bool,
}

impl App {
    fn new(ctx: &mut Context) -> GameResult<App> {
        Ok(App { board: Board::new(),
            sel_piece_data: (None, None, None),
            time_since_start: Instant::now(),
            curr_turn: true,
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
                        if (x + y) % 2 == 0 {Color {r: 0.502, g: 0.502, b: 0.502, a: 1.0} } else {Color::BLACK},
                    )?, Vec2::new(0.0, 0.0));
                    //println!("{} {} WHITE", (width / 8.0) * x as f32, (height / 8.0) * i as f32);
            }
        }
        for y in 0..8 {
            for x in 0..8 {
                let figure = &self.board.data[x as usize][y as usize];
                if let Some(_Figure) = figure {
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
        let figure = &self.sel_piece_data.2;
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
        /*canvas.draw(&graphics::Mesh::new_rectangle(
            ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(0.0, 0.0, 20.0, h),
                Color::RED,
            )?, Vec2::new(0.0, 0.0));*/
        canvas.finish(ctx)?;

        Ok(())
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult {
        let (width, height) = (SCREEN_WIDTH, SCREEN_HEIGHT);
        let (g_x, g_y) = (width / 8.0, height / 8.0);
        let (mut grid_x,mut grid_y) = (_y as i32 / g_y as i32, _x as i32 / g_x as i32);
        if grid_x == 8 {
            grid_x = 7;
        }
        if grid_y == 8 {
            grid_y = 7;
        }
        let figure = self.board.data[grid_x as usize][grid_y as usize];
        if self.sel_piece_data.2.is_none() {
            if figure.is_none() {
                println!("None at {} {}", grid_x, grid_y)
            } else {
                let color = match figure {
                    Some(PlayerFigure::Black(_figure)) => {
                        Color::BLACK
                    }
                    Some(PlayerFigure::White(_figure)) => {
                        Color::WHITE
                    }
                    None => {
                        Color::RED
                    }
                };
                if self.curr_turn == true && color == Color::WHITE { // and is white figure
                    if figure.is_none() {println!("NONE DETECTED white init");}
                    self.sel_piece_data = (Some(grid_x), Some(grid_y), figure);
                } else if self.curr_turn != true && color == Color::BLACK { // and is black figure
                    if figure.is_none() {println!("NONE DETECTED at initial");}
                    self.sel_piece_data = (Some(grid_x), Some(grid_y), figure);
                } else if !self.sel_piece_data.0.is_none()
                {
                    let fig = self.sel_piece_data.2.unwrap();
                    let moves;
                        match fig {
                            PlayerFigure::Black(figure) => {
                                moves = figure.get_moves();
                            },
                            PlayerFigure::White(figure) => {
                                moves = figure.get_moves();
                            }
                        }
                    if self.curr_turn != true && color == Color::WHITE
                    {
                        match moves.1 {
                            MovementType::Limited => {
                                
                            }
                            MovementType::Unlimited => {
                                
                            }
                            MovementType::Pawn => {
                                
                            }
                        }

                    } else if self.curr_turn == true && color == Color::BLACK {

                    }
                }
                
                
                 else if self.sel_piece_data.2.is_none() 
                {} else {
                    
                    println!("How did we get here?");
                }
            }
        }
        else if figure.is_none() {
            if self.sel_piece_data.2.is_none() {println!("NONE DETECTED movement check");}
            if self.sel_piece_data.1.is_none() {println!("NONE DETECTED movement 1");}
            if self.sel_piece_data.0.is_none() {println!("NONE DETECTED movement 0");}
            // Chechks above do not detect a null
            let piece = self.sel_piece_data;
            let color;
            let u_figure = match piece.2 {
                Some(PlayerFigure::Black(piece)) => {
                    color = Color::BLACK;
                    piece
                }
                Some(PlayerFigure::White(piece)) => {
                    color = Color::WHITE;
                    piece
                }
                None => {
                    color = Color::WHITE;
                    Figure::Bishop
                }
            };
            let moves = u_figure.get_moves();
            for fig_move in moves.0 {
                match moves.1 {
                    MovementType::Limited => {
                        // Checks below do detect a null if the piece is black
                        if piece.2.is_none() {println!("NONE DETECTED movement");}
                        if piece.1.is_none() {println!("NONE DETECTED movement 1");}
                        if piece.0.is_none() {println!("NONE DETECTED movement 0");}
                        if grid_x - &piece.0.unwrap() == fig_move[0] && grid_y - &piece.1.unwrap() == fig_move[1] {
                            self.board.data[piece.0.unwrap() as usize][piece.1.unwrap() as usize] = None;
                            self.board.data[grid_x as usize][grid_y as usize] = piece.2;
                            self.sel_piece_data = (None, None, None);
                            self.curr_turn = !self.curr_turn;
                            println!("Shot at {} {}", grid_x, grid_y);
                        }
                    }
                    MovementType::Unlimited => {

                    }
                    MovementType::Pawn => {

                    }
                }
                
            }
            
        } else {
            let color = match figure {
                Some(PlayerFigure::Black(figure)) => {
                    Color::BLACK
                }
                Some(PlayerFigure::White(figure)) => {
                    Color::WHITE
                }
                None => {
                    Color::RED
                }
            };
            if self.curr_turn == true && color == Color::WHITE { // and is white figure
                if self.sel_piece_data.2.is_none() {println!("NONE DETECTED white sec");}
                self.sel_piece_data = (Some(grid_x), Some(grid_y), figure);
            } else if self.curr_turn != true && color == Color::BLACK { // and is black figure
                if figure.is_none() {println!("NONE DETECTED black sec");}
                println!("{} {}", grid_x, grid_y);
                self.sel_piece_data = (Some(grid_x), Some(grid_y), figure);
            } else {
                if self.sel_piece_data.2.is_none() {println!("NONE DETECTED movement check");}
                if self.sel_piece_data.1.is_none() {println!("NONE DETECTED movement 1");}
                if self.sel_piece_data.0.is_none() {println!("NONE DETECTED movement 0");}
                // Chechks above do not detect a null
                let piece = self.sel_piece_data;
                let color;
                let u_figure = match piece.2 {
                    Some(PlayerFigure::Black(piece)) => {
                        color = Color::BLACK;
                        piece
                    }
                    Some(PlayerFigure::White(piece)) => {
                        color = Color::WHITE;
                        piece
                    }
                    None => {
                        color = Color::WHITE;
                        Figure::Bishop
                    }
                };
                let moves = u_figure.get_moves();
                for fig_move in moves.0 {
                    match moves.1 {
                        MovementType::Limited => {
                            // Checks below do detect a null if the piece is black
                            if piece.2.is_none() {println!("NONE DETECTED movement");}
                            if piece.1.is_none() {println!("NONE DETECTED movement 1");}
                            if piece.0.is_none() {println!("NONE DETECTED movement 0");}
                            if grid_x - &piece.0.unwrap() == fig_move[0] && grid_y - &piece.1.unwrap() == fig_move[1] {
                                self.board.data[piece.0.unwrap() as usize][piece.1.unwrap() as usize] = None;
                                self.board.data[grid_x as usize][grid_y as usize] = piece.2;
                                self.sel_piece_data = (None, None, None);
                                self.curr_turn = !self.curr_turn;
                                println!("Shot at {} {}", grid_x, grid_y);
                            }
                        }
                        MovementType::Unlimited => {

                        }
                        MovementType::Pawn => {

                        }
                    }
                    
                }
            }
        }
        Ok(())
        //println!("Current selected coords: X: {}, Y: {}, Is it whites turn? {}", self.sel_piece_x.unwrap(), self.sel_piece_y.unwrap(), self.board.is_whites_turn);
        //Ok(())
    }
}

pub fn main() -> GameResult {
    env::set_var("RUST_BACKTRACE", "0");
    let cb = ggez::context::ContextBuilder::new("Chess", "malanak")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT));
    let (mut ctx, event_loop) = cb.build()?;
    let app = App::new(&mut ctx)?;
    event::run(ctx, event_loop, app)
}

//CREDITS: Chess pieces + chess board looks: JohnPablok's improved Cburnett chess set. (https://creativecommons.org/licenses/by-sa/3.0/)
