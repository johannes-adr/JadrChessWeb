use std::fmt::Write;

use arrayvec::ArrayVec;

use crate::{color::Color, chess_figure::{ChessFigure, figure_move::{Point, Move, generate_move}}};

// use crate::chess_figure::{
//     aware_array::{AwareArray2D, AwareElement},
//     figure_move::{Move, Point},
//     ChessFigure, Figure,
// };

// use super::color::Color;

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

    pub fn to_fen(&self) -> String {
        let mut s = String::with_capacity(4);
        if self.white_short {
            _ = s.write_char('K');
        }
        if self.white_long {
            _ = s.write_char('Q');
        }
        if self.black_short {
            _ = s.write_char('k');
        }
        if self.black_long {
            _ = s.write_char('q');
        }
        return s;
    }

    pub fn default() -> Self {
        Self::from_fen("")
    }
}
type AwareBoard = [[Option<ChessFigure>;8];8];
#[derive(Debug)]
pub struct ChessBoard {
    turns: u32,
    on_turn: Color,
    half_turns: u32,
    castle: Castleable,
    board: AwareBoard,
}

const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
impl ChessBoard {
    pub fn empty() -> Self {
        Self {
            turns: 0,
            on_turn: Color::White,
            board: Default::default(),
            castle: Castleable::default(),
            half_turns: 0,
        }
    }

    pub fn board(&mut self,)->&mut AwareBoard{
        &mut self.board
    }

    pub fn default() -> Self {
        let mut b = Self::empty();
        b.load_fen(DEFAULT_FEN).expect("ERROR LOADING DEFAULT FEN");
        return b;
    }

    pub fn from_fen(fen: &str) -> Result<ChessBoard, String> {
        let mut b = ChessBoard::empty();
        b.load_fen(fen)?;
        return Ok(b);
    }

    pub fn clear(&self) {}

    pub fn load_fen(&mut self, fen: &str) -> core::result::Result<(), String> {
        self.clear();
        let fen_parts: Vec<&str> = fen.split(" ").collect();
        if fen_parts.len() != 6 {
            return Err("Fen not correct format (Needs to have 6 parts after split)".to_owned());
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
                    let f = ChessFigure::from_fen(c, Point::new(x, y))
                        .ok_or(format!("Error parsing fenchar '{}'", c).to_string())?;
                    self.board[y][x] = Some(f)
                }
                x += 1;
            }
        }
        Ok(())
    }

    pub fn get_figure(&self, x: usize, y: usize) -> &Option<ChessFigure> {
        &self.board[y][x]
    }

    // pub fn move_figure(&mut self,x: usize, y: usize, f: &mut ChessFigure){
    //     self.delete(f.get_pos().x(), f.get_pos().y());
    //     f.flag_moved();
    //     f.set_pos(x, y);
    //     self.board[y][x] = Some(*f);
    // }
    /**
     * Returns true if field is none
     */
    pub fn empty_at(&self, x: usize, y: usize) -> bool {
        if let Option::None = self.get_figure(x, y) {
            true
        } else {
            false
        }
    }

    /**
     * Returns 0 if target field is with ally figure
     * 1 if field with enemy figure
     * 2 if field is empty
     */
    pub fn moveable(&self, x: usize, y: usize, fig: &ChessFigure) -> u8 {
        if let Some(fig2) = self.get_figure(x, y) {
            if fig2.color().equals(fig.color()) {
                0 //false
            } else {
                1 //true
            }
        } else {
            2 //true
        }
    }

    pub fn get_moves_for_side(&self, c: Color) -> Vec<Move> {
        let mut moves = Vec::with_capacity(100);
        for row in &self.board{
            for field in row{
                if let Some(f) = field{
                    if f.color().equals(c){
                        moves.append(&mut generate_move(f, self))
                    }
                }
            }
        }
        moves
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(80);
        for row in &self.board {
            let mut start = 0;
            for x in 0..8 {
                let figure = &row[x];
                if let Some(figure) = figure {
                    if start > 0 {
                        _ = fen.write_char(char::from_digit(start, 10).unwrap());
                        start = 0
                    }
                    _ = fen.write_char(figure.fen());
                } else {
                    start += 1;
                }
            }

            if start > 0 {
                _ = fen.write_char(char::from_digit(start, 10).unwrap());
            }
            _ = fen.write_char('/');
        }
        let mut fen_chars = fen.chars();
        fen_chars.next_back();
        fen = fen_chars.as_str().to_string();
        _ = fen.write_str(format!(" {} ", self.on_turn.to_fen()).as_str());
        _ = fen.write_str(self.castle.to_fen().as_str());
        _ = fen.write_str(format!(" - 0 {}", self.turns).as_str());
        fen
    }

    pub fn as_str(&self) -> String {
        let mut res = String::new();
        for row in &self.board {
            for cell in row {
                if let Some(f) = cell {
                    _ = res.write_char(f.fen());
                } else {
                    _ = res.write_char('°');
                }
            }
            _ = res.write_char('\n')
        }
        return res;
    }
}