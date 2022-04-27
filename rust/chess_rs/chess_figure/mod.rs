use crate::{color::Color, chess_board::ChessBoard};

use self::{figure_type::ChessFigureType, figure_move::{generate_move, Point}};
pub mod metadata;
pub mod figure_type;
pub mod figure_move;
pub mod aware_array;
use figure_move::Move;

#[derive(Debug, Clone)]
pub struct ChessFigure{
    figure: ChessFigureType,
    color: Color,
    already_moved: bool,
    pos: Point
}

impl ChessFigure {
    pub fn figure(&self)-> ChessFigureType{
        self.figure
    }

    pub fn pos(&self) -> Point{
        self.pos
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

    pub fn color(&self) -> Color{
        self.color
    }

    pub fn from_fen(fen: char, pos: Point) -> Option<Self> {
        if let Some(figure) = ChessFigureType::from_fen(fen.to_ascii_lowercase()){
            return Some(Self{color: Color::from_char(fen), figure, already_moved: false,pos})
        }
        None        
    }

}