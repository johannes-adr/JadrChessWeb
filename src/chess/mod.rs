use std::fmt::Write;

use self::chess_figure::ChessFigure;

mod chess_figure;

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn to_fen(&self) -> char {
        match self {
            Color::Black => 'b',
            Color::White => 'w',
        }
    }

    pub fn from_fen(c: char) -> Option<Self> {
        if c == 'w' {
            Some(Color::White)
        } else if c == 'b' {
            Some(Color::Black)
        } else {
            None
        }
    }
}

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
            s.write_char('K');
        }
        if self.white_long{
            s.write_char('Q');
        }
        if self.black_short{
            s.write_char('k');
        }
        if self.black_long{
            s.write_char('q');
        }
        return s;
    }

    pub fn default() -> Self {
        Self::from_fen("")
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

    pub fn clear(&self) {}

    pub fn load_fen(&mut self, fen: &str) -> core::result::Result<(), String> {
        self.clear();
        let mut fen_parts: Vec<&str> = fen.split(" ").collect();

        let fen_board = fen_parts[0];
        self.on_turn = Color::from_fen(
            fen_parts[1]
                .chars()
                .next()
                .ok_or("No fen char for on_turn color provided")?,
        )
        .ok_or("Error parsing fen char for on_turn color")?;

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
                    let f = ChessFigure::from_fen(c)
                        .ok_or(format!("Error parsing fenchar {}", c).to_string())?;
                    self.board[y][x] = Some(f);
                }
                x += 1;
            }
        }
        Ok(())
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
            fen.write_char('/');
        }
        let mut fenChars = fen.chars();
        fenChars.next_back();
        fen = fenChars.as_str().to_string();
        _=fen.write_str(format!(" {} ",self.on_turn.to_fen()).as_str());
        _=fen.write_str(self.castle.to_fen().as_str());
        _=fen.write_str(format!(" - 0 {}",self.turns).as_str());
        fen
    }
}
