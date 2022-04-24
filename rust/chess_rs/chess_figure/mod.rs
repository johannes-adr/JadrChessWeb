use crate::{color::Color, chess_board::ChessBoard};

use self::{figure_type::ChessFigureType, figure_move::{generate_move, Point}};
pub mod metadata;
pub mod figure_type;
pub mod figure_move;
use figure_move::Move;

#[derive(Debug, Clone, Copy)]
pub struct ChessFigure{
    figure: ChessFigureType,
    color: Color,
    pos: Point,
    already_moved: bool
}

impl ChessFigure {
    pub fn figure(&self)-> ChessFigureType{
        self.figure
    }

    pub fn flag_moved(&mut self){
        self.already_moved = true;
    }

    pub fn fen(&self) -> char{
        let s = self.figure().stats();
        match self.color{
            Color::Black => s.fen_char,
            Color::White => s.fen_char.to_ascii_uppercase(),
        }
    }

    pub fn set_pos(&mut self,x: usize, y: usize){
        self.pos = Point::new(x, y)
    }

    pub fn get_pos(&self) -> Point{
        self.pos
    }

    pub fn color(&self) -> Color{
        self.color
    }

    pub fn from_fen(fen: char, x: usize, y: usize) -> Option<Self> {
        if let Some(figure) = ChessFigureType::from_fen(fen.to_ascii_lowercase()){
            return Some(Self{color: Color::from_char(fen), figure, pos: Point::new(x,y), already_moved: false})
        }
        None        
    }

    pub fn get_avaible_moves(&self,board: &ChessBoard) -> Vec<Move>{
        generate_move(self, board)
    }
}