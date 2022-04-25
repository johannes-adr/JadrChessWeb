use std::fmt::Write;

use crate::chess_figure::{ChessFigure, figure_move::Move};

use super::color::Color;

#[derive(Debug)]
struct Castleable {
    white_short: bool,
    white_long: bool,
    black_short: bool,
    black_long: bool,
}

impl Castleable {
    pub fn from_fen(s: &str) -> Self {
        Self {
            white_short: s.contains("K"),
            white_long: s.contains("Q"),
            black_short: s.contains("k"),
            black_long: s.contains("q"),
        }
    }

    pub fn to_fen(&self) -> String{
        let mut s = String::with_capacity(4);
        if self.white_short{
            _=s.write_char('K');
        }
        if self.white_long{
            _=s.write_char('Q');
        }
        if self.black_short{
            _=s.write_char('k');
        }
        if self.black_long{
            _=s.write_char('q');
        }
        return s;
    }

    pub fn default() -> Self {
        Self::from_fen("")
    }
}

pub struct FenBuilder{
    board: ChessBoard
}

impl FenBuilder{
    pub fn new() -> FenBuilder{
        Self{board: ChessBoard::empty()}
    }

    pub fn board(&self) -> &ChessBoard {
        &self.board
    }

    pub fn set(&mut self, f: ChessFigure) -> &Self{
        self.board.board[f.get_pos().y()][f.get_pos().x()] = Some(f);
        self
    }
    pub fn set_char(&mut self, fen: char, x: usize, y: usize) -> &Self{
        self.board.board[y][x] = ChessFigure::from_fen(fen, x, y);
        self
    }

    pub fn build(&self) -> String{
        self.board.to_fen()
    }
}

#[derive(Debug)]
pub struct ChessBoard {
    turns: u32,
    on_turn: Color,
    half_turns: u32,
    castle: Castleable,
    board: [[Option<ChessFigure>; 8]; 8],
}
struct TeamFigures{
    vec: [ChessFigure;16]
}

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
impl ChessBoard {
    pub fn empty() -> Self {
        Self {
            turns: 0,
            on_turn: Color::White,
            board: [[Option::None; 8]; 8],
            castle: Castleable::default(),
            half_turns: 0,
        }
    }

    pub fn default() -> Self {
        let mut b = Self::empty();
        b.load_fen(DEFAULT_FEN).expect("ERROR LOADING DEFAULT FEN");
        return b;
    }

    pub fn from_fen(fen: &str) -> Result<ChessBoard, String>{
        let mut b = ChessBoard::empty();
        b.load_fen(fen)?;
        return Ok(b)
    }

    pub fn clear(&self) {}

    pub fn load_fen(&mut self, fen: &str) -> core::result::Result<(), String> {
        self.clear();
        let fen_parts: Vec<&str> = fen.split(" ").collect();
        if fen_parts.len() != 6{
            return Err("Fen not correct format (Needs to have 6 parts after split)".to_owned())
        }
        let fen_board = fen_parts[0];
        self.on_turn = Color::from_char(
            fen_parts[1]
                .chars()
                .next()
                .ok_or("No fen char for on_turn color provided")?,
        );

        self.castle = Castleable::from_fen(fen_parts[2]);
        self.half_turns = fen_parts[4].parse().unwrap();
        self.turns = fen_parts[5].parse().unwrap();

        let rows = fen_board.split("/");

        for (y, row) in rows.enumerate() {
            let mut x = 0;
            while x < row.len() {
                let c = row.chars().nth(x).unwrap();
                if c.is_numeric() {
                    x += c.to_digit(10).unwrap() as usize - 1
                } else {
                    let f = ChessFigure::from_fen(c,x,y)
                        .ok_or(format!("Error parsing fenchar '{}'", c).to_string())?;
                    self.board[y][x] = Some(f);
                }
                x += 1;
            }
        }
        Ok(())
    }

    pub fn get_figure(&self,x: usize, y: usize) -> Option<ChessFigure>{
        self.board[y][x]
    }

    pub fn get_field(&mut self, x: usize, y: usize) ->&mut Option<ChessFigure>{
        &mut self.board[y][x]
    }

    // pub fn move_figure(&mut self,x: usize, y: usize, f: &mut ChessFigure){
    //     self.delete(f.get_pos().x(), f.get_pos().y());
    //     f.flag_moved();
    //     f.set_pos(x, y);
    //     self.board[y][x] = Some(*f);
    // }

    fn set_field(&mut self,x: usize, y: usize, f: Option<ChessFigure>){
        self.board[y][x] = f;
    }

    /**
     * Returns true if field is none
     */
    pub fn empty_at(&self, x: usize, y: usize) -> bool{
        if let Option::None = self.get_figure(x, y) {
            true
        }else {
            false
        }
    }

    /**
     * Returns 0 if target field is with ally figure
     * 1 if field with enemy figure
     * 2 if field is empty
     */
    pub fn moveable(&self, x: usize,y:usize,fig: &ChessFigure) -> u8{
        if let Some(fig2) = self.get_figure(x, y){
            if fig2.color().equals(fig.color()){
                0 //false
            }else{
                1 //true
            }
        }else{
            2 //true
        }
    }

    pub fn move_figure(&self,figure: &mut ChessFigure, x: usize, y: usize){
        let start = figure.get_pos();
        self.set_field(start.x(), start.y(), None);
        figure.set_pos(x, y);
        self.set_field(x, y, figure);

    }

    pub fn get_moves_for_side(&self) -> Vec<Move>{
        let mut moves = Vec::with_capacity(100);
        for row in self.board{
            for field in row{
                if let Some(fig) = field{
                    let mut fig_moves = fig.get_avaible_moves(self);
                    moves.append(&mut fig_moves)
                }
            }
        }
        moves
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(80);
        for row in self.board {
            let mut start = 0;
            for x in 0..8 {
                let figure = row[x];
                if figure.is_none() {
                    start += 1;
                } else {
                    if start > 0 {
                        _ = fen.write_char(char::from_digit(start, 10).unwrap());
                        start = 0
                    }
                    _ = fen.write_char(figure.unwrap().fen());
                }
            }

            if start > 0 {
                _ = fen.write_char(char::from_digit(start, 10).unwrap());
            }
            _=fen.write_char('/');
        }
        let mut fen_chars = fen.chars();
        fen_chars.next_back();
        fen = fen_chars.as_str().to_string();
        _=fen.write_str(format!(" {} ",self.on_turn.to_fen()).as_str());
        _=fen.write_str(self.castle.to_fen().as_str());
        _=fen.write_str(format!(" - 0 {}",self.turns).as_str());
        fen
    }

    pub fn as_str(&self) -> String{
        let mut res = String::new();
        for row in self.board{
            for cell in row{
                if let Some(f) = cell{
                    _=res.write_char(f.fen());
                }else{
                    _=res.write_char('°');
                }
            }
            _=res.write_char('\n')
        }
        return res;
    }


}



#[test]
fn test_fen() {
    let b = ChessBoard::from_fen("");
    assert!(b.is_err());
}