#![allow(clippy::unnecessary_wraps)]

use ggez::{event, glam::*, graphics::{self, Color}, Context, GameResult, graphics::Image, GameError};
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
    white_board_image: Image,
    dark_board_image: Image,
    w_knight: Image,
    w_rook: Image,
    w_bishop: Image,
    w_king: Image,
    w_queen: Image,
    w_pawn: Image,
    b_knight: Image,
    b_rook: Image,
    b_bishop: Image,
    b_king: Image,
    b_queen: Image,
    b_pawn: Image,
    sel_piece_x: Option<i32>,
    sel_piece_y: Option<i32>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let w_knight = Image::from_path(ctx, "/img/w_knight.png")?;
        let w_bishop = Image::from_path(ctx, "/img/w_bishop.png")?;
        let w_king = Image::from_path(ctx, "/img/w_king.png")?;
        let w_queen = Image::from_path(ctx, "/img/w_queen.png")?;
        let w_rook = Image::from_path(ctx, "/img/w_rook.png")?;
        let w_pawn = Image::from_path(ctx, "/img/w_pawn.png")?;
        let b_knight = Image::from_path(ctx, "/img/b_knight.png")?;
        let b_bishop = Image::from_path(ctx, "/img/b_bishop.png")?;
        let b_king = Image::from_path(ctx, "/img/b_king.png")?;
        let b_queen = Image::from_path(ctx, "/img/b_queen.png")?;
        let b_rook = Image::from_path(ctx, "/img/b_rook.png")?;
        let b_pawn = Image::from_path(ctx, "/img/b_pawn.png")?;
        Ok(MainState { board: Board::new(),
            white_board_image: Image::from_path(ctx, "/img/square_brown_light.png")?,
            dark_board_image: Image::from_path(ctx, "/img/square_brown_dark.png")?,
            w_knight,
            w_bishop,
            w_king,
            w_queen,
            w_rook,
            w_pawn,
            b_knight,
            b_bishop,
            b_king,
            b_queen,
            b_rook,
            b_pawn,
            sel_piece_x: None,
            sel_piece_y: None,
        })
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
                let curr_coords = &self.board.data[i][x];
                /*
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
                    canvas.draw(&self.white_board_image, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    /*
                    canvas.draw(&graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, width / 8.0, height / 8.0),
                        Color::WHITE,
                    )?, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    println!("{} {} WHITE", (width / 8.0) * x as f32, (height / 8.0) * i as f32);*/
                    is_white = false;
                } else {
                    canvas.draw(&self.dark_board_image, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    /*
                    canvas.draw(&graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(0.0, 0.0, width / 8.0, height / 8.0),
                        Color::BLACK,
                    )?, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    println!("{} {} BLACK", (width / 8.0) * x as f32, (height / 8.0) * i as f32);*/
                    is_white = true;
                }
                match curr_coords {
                    Some(PlayerFigure::Black(Figure::Rook)) => {
                        canvas.draw(&self.b_rook, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::White(Figure::Rook)) => {
                        canvas.draw(&self.w_rook, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::Black(Figure::Bishop)) => {
                        canvas.draw(&self.b_bishop, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::White(Figure::Bishop)) => {
                        canvas.draw(&self.w_bishop, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::Black(Figure::Knight)) => {
                        canvas.draw(&self.b_knight, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::White(Figure::Knight)) => {
                        canvas.draw(&self.w_knight, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::Black(Figure::Pawn)) => {
                        canvas.draw(&self.b_pawn, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::White(Figure::Pawn)) => {
                        canvas.draw(&self.w_pawn, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::Black(Figure::Queen)) => {
                        canvas.draw(&self.b_queen, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::White(Figure::Queen)) => {
                        canvas.draw(&self.w_queen, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::Black(Figure::King)) => {
                        canvas.draw(&self.b_king, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    Some(PlayerFigure::White(Figure::King)) => {
                        canvas.draw(&self.w_king, Vec2::new((width / 8.0) * x as f32, (height / 8.0) * i as f32));
                    }
                    None => {}
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
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) -> GameResult {
        let (width, height) = graphics::GraphicsContext::size(&_ctx.gfx/* &GraphicsContext */);
        let grid_x = width / 8.0;
        let grid_y = height / 8.0;
        let mod_x = &_x % 8.0;
        let mod_y = &_y % 8.0;
        let fin_x = &_x - mod_x;
        let fin_y = &_y - mod_y;
        let coord_x = fin_x / grid_x;
        let coord_y = fin_y / grid_y;
        let last_sel = &self.board.data[coord_x as usize][coord_y as usize];
        if self.sel_piece_x.is_none() && self.sel_piece_y.is_none() {
            self.sel_piece_x = Some(coord_x as i32);
            self.sel_piece_y = Some(coord_y as i32);
        } else {
            match last_sel {
                Some(PlayerFigure::Black(Figure::Pawn)) => {
                    if self.board.is_whites_turn {
                        if self.sel_piece_x.unwrap() + 1 == coord_x as i32 {
                            if self.sel_piece_y.unwrap() != coord_y as i32 {
                                if self.sel_piece_y.unwrap() + 1 == coord_y as i32 || self.sel_piece_y.unwrap() - 1 == coord_y as i32 {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Pawn));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = false;
                                }
                                else {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Pawn));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = false;
                                }
                            }
                        }
                        self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Pawn));
                        self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                        self.sel_piece_x = None;
                        self.sel_piece_y = None;
                    } else {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                }
                Some(PlayerFigure::Black(Figure::Bishop)) => {
                    if self.board.is_whites_turn {
                        for x in -7..7 {
                            if self.sel_piece_x.unwrap() + x == coord_x as i32 && self.sel_piece_y.unwrap() + x == coord_y as i32 {
                                self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Bishop));
                                self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                self.sel_piece_x = None;
                                self.sel_piece_y = None;
                                self.board.is_whites_turn = false;
                            } else if self.sel_piece_x.unwrap() + (x * -1) == coord_x as i32 && self.sel_piece_y.unwrap() + x == coord_y as i32 {
                                self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Bishop));
                                self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                self.sel_piece_x = None;
                                self.sel_piece_y = None;
                                self.board.is_whites_turn = false;
                            } else if self.sel_piece_x.unwrap() + x == coord_x as i32 && self.sel_piece_y.unwrap() + (x * -1) == coord_y as i32 {
                                self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Bishop));
                                self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                self.sel_piece_x = None;
                                self.sel_piece_y = None;
                                self.board.is_whites_turn = false;
                            }
                        }
                    } else {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                }
                Some(PlayerFigure::Black(Figure::Knight)) => {
                    if self.board.is_whites_turn {
                        let mov_main = 3;
                        let mov_side = 1;
                        if self.sel_piece_x.unwrap() + mov_main == coord_x as i32 && self.sel_piece_y.unwrap() + mov_side == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() + mov_main == coord_x as i32 && self.sel_piece_y.unwrap() - mov_side == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() - mov_main == coord_x as i32 && self.sel_piece_y.unwrap() + mov_side == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() - mov_main == coord_x as i32 && self.sel_piece_y.unwrap() - mov_side == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() + mov_side == coord_x as i32 && self.sel_piece_y.unwrap() + mov_main == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() + mov_side == coord_x as i32 && self.sel_piece_y.unwrap() - mov_main == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() - mov_side == coord_x as i32 && self.sel_piece_y.unwrap() + mov_main == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() - mov_side == coord_x as i32 && self.sel_piece_y.unwrap() - mov_main == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        }
                    } else {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                }
                Some(PlayerFigure::Black(Figure::King)) => {
                    if self.board.is_whites_turn {
                        if self.sel_piece_x.unwrap() - 1 == coord_x as i32 && self.sel_piece_y.unwrap() - 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() + 1 == coord_x as i32 && self.sel_piece_y.unwrap() - 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() - 1 == coord_x as i32 && self.sel_piece_y.unwrap() + 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() + 1 == coord_x as i32 && self.sel_piece_y.unwrap() + 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() - 1 == coord_x as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_x.unwrap() + 1 == coord_x as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_y.unwrap() - 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else if self.sel_piece_y.unwrap() + 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else {
                            self.sel_piece_x = Some(coord_x as i32);
                            self.sel_piece_y = Some(coord_y as i32);
                        }
                    } else {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                }
                Some(PlayerFigure::Black(Figure::Queen)) => {
                    if self.board.is_whites_turn {
                        if self.sel_piece_x.unwrap() == coord_x as i32 || self.sel_piece_y.unwrap() == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Queen));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else {
                            for x in -7..7 {
                                if self.sel_piece_x.unwrap() + x == coord_x as i32 && self.sel_piece_y.unwrap() + x == coord_y as i32 {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Bishop));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = false;
                                } else if self.sel_piece_x.unwrap() + (x * -1) == coord_x as i32 && self.sel_piece_y.unwrap() + x == coord_y as i32 {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Bishop));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = false;
                                } else if self.sel_piece_x.unwrap() + x == coord_x as i32 && self.sel_piece_y.unwrap() + (x * -1) == coord_y as i32 {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Bishop));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = false;
                                }
                            }
                            self.sel_piece_x = Some(coord_x as i32);
                            self.sel_piece_y = Some(coord_y as i32);
                        }
                    } else {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                }
                Some(PlayerFigure::Black(Figure::Rook)) => {
                    if self.board.is_whites_turn {
                        if self.sel_piece_x.unwrap() == coord_x as i32 || self.sel_piece_y.unwrap() == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::Black(Figure::Rook));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = false;
                        } else {
                            self.sel_piece_x = Some(coord_x as i32);
                            self.sel_piece_y = Some(coord_y as i32);
                        }
                    } else {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                }
                Some(PlayerFigure::White(Figure::Pawn)) => {
                    if self.board.is_whites_turn {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                    else {
                        if self.sel_piece_x.unwrap() + 1 == coord_x as i32 {
                            if self.sel_piece_y.unwrap() != coord_y as i32 {
                                if self.sel_piece_y.unwrap() + 1 == coord_y as i32 || self.sel_piece_y.unwrap() - 1 == coord_y as i32 {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Pawn));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = true;
                                }
                                else {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Pawn));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = true;
                                }
                            }
                        } else {
                            self.sel_piece_x = Some(coord_x as i32);
                            self.sel_piece_y = Some(coord_y as i32);
                        }
                    }
                }
                Some(PlayerFigure::White(Figure::Bishop)) => {
                    if self.board.is_whites_turn {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                    else {
                        for x in -7..7 {
                            if self.sel_piece_x.unwrap() + x == coord_x as i32 && self.sel_piece_y.unwrap() + x == coord_y as i32 {
                                self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Bishop));
                                self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                self.sel_piece_x = None;
                                self.sel_piece_y = None;
                                self.board.is_whites_turn = true;
                            } else if self.sel_piece_x.unwrap() + (x * -1) == coord_x as i32 && self.sel_piece_y.unwrap() + x == coord_y as i32 {
                                self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Bishop));
                                self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                self.sel_piece_x = None;
                                self.sel_piece_y = None;
                                self.board.is_whites_turn = true;
                            } else if self.sel_piece_x.unwrap() + x == coord_x as i32 && self.sel_piece_y.unwrap() + (x * -1) == coord_y as i32 {
                                self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Bishop));
                                self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                self.sel_piece_x = None;
                                self.sel_piece_y = None;
                                self.board.is_whites_turn = true;
                            } else {
                                self.sel_piece_x = Some(coord_x as i32);
                                self.sel_piece_y = Some(coord_y as i32);
                            }
                        }
                    }
                }
                Some(PlayerFigure::White(Figure::Knight)) => {
                    if self.board.is_whites_turn {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                    else {
                        let mov_main = 3;
                        let mov_side = 1;
                        if self.sel_piece_x.unwrap() + mov_main == coord_x as i32 && self.sel_piece_y.unwrap() + mov_side == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() + mov_main == coord_x as i32 && self.sel_piece_y.unwrap() - mov_side == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() - mov_main == coord_x as i32 && self.sel_piece_y.unwrap() + mov_side == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() - mov_main == coord_x as i32 && self.sel_piece_y.unwrap() - mov_side == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() + mov_side == coord_x as i32 && self.sel_piece_y.unwrap() + mov_main == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() + mov_side == coord_x as i32 && self.sel_piece_y.unwrap() - mov_main == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() - mov_side == coord_x as i32 && self.sel_piece_y.unwrap() + mov_main == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() - mov_side == coord_x as i32 && self.sel_piece_y.unwrap() - mov_main == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Knight));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        }
                        else {
                            self.sel_piece_x = Some(coord_x as i32);
                            self.sel_piece_y = Some(coord_y as i32);
                        }
                    }
                }
                Some(PlayerFigure::White(Figure::King)) => {
                    if self.board.is_whites_turn {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                    else {
                        if self.sel_piece_x.unwrap() - 1 == coord_x as i32 && self.sel_piece_y.unwrap() - 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() + 1 == coord_x as i32 && self.sel_piece_y.unwrap() - 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() - 1 == coord_x as i32 && self.sel_piece_y.unwrap() + 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() + 1 == coord_x as i32 && self.sel_piece_y.unwrap() + 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() - 1 == coord_x as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_x.unwrap() + 1 == coord_x as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_y.unwrap() - 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else if self.sel_piece_y.unwrap() + 1 == coord_y as i32{
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::King));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else {
                            self.sel_piece_x = Some(coord_x as i32);
                            self.sel_piece_y = Some(coord_y as i32);
                        }
                    }
                }
                Some(PlayerFigure::White(Figure::Queen)) => {
                    if self.board.is_whites_turn {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                    else {
                        if self.sel_piece_x.unwrap() == coord_x as i32 || self.sel_piece_y.unwrap() == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Queen));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else {
                            for x in -7..7 {
                                if self.sel_piece_x.unwrap() + x == coord_x as i32 && self.sel_piece_y.unwrap() + x == coord_y as i32 {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Bishop));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = true;
                                } else if self.sel_piece_x.unwrap() + (x * -1) == coord_x as i32 && self.sel_piece_y.unwrap() + x == coord_y as i32 {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Bishop));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = true;
                                } else if self.sel_piece_x.unwrap() + x == coord_x as i32 && self.sel_piece_y.unwrap() + (x * -1) == coord_y as i32 {
                                    self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Bishop));
                                    self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                                    self.sel_piece_x = None;
                                    self.sel_piece_y = None;
                                    self.board.is_whites_turn = true;
                                }
                            }
                            self.sel_piece_x = Some(coord_x as i32);
                            self.sel_piece_y = Some(coord_y as i32);
                        }
                    }
                }
                Some(PlayerFigure::White(Figure::Rook)) => {
                    if self.board.is_whites_turn {
                        self.sel_piece_x = Some(coord_x as i32);
                        self.sel_piece_y = Some(coord_y as i32);
                    }
                    else {
                        if self.sel_piece_x.unwrap() == coord_x as i32 || self.sel_piece_y.unwrap() == coord_y as i32 {
                            self.board.data[coord_x as usize][coord_y as usize] = Some(PlayerFigure::White(Figure::Rook));
                            self.board.data[self.sel_piece_x.unwrap() as usize][self.sel_piece_y.unwrap() as usize] = None;
                            self.sel_piece_x = None;
                            self.sel_piece_y = None;
                            self.board.is_whites_turn = true;
                        } else {
                            self.sel_piece_x = Some(coord_x as i32);
                            self.sel_piece_y = Some(coord_y as i32);
                        }
                    }
                }
                None => {

                }
            }
        }
        if self.sel_piece_x.is_none() && self.sel_piece_y.is_none() {}
        else {
            println!("Current selected coords: X: {}, Y: {}, Is it whites turn? {}", self.sel_piece_x.unwrap(), self.sel_piece_y.unwrap(), self.board.is_whites_turn);
        }Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::context::ContextBuilder::new("Chess", "malanak");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

//CREDITS: Chess pieces + chess board looks: JohnPablok's improved Cburnett chess set. (https://creativecommons.org/licenses/by-sa/3.0/)