
mod chess_figure;

#[derive(Debug,Clone, Copy)]
enum Color{
    Black,
    White
}

#[derive(Debug,Clone, Copy)]
pub enum ChessFigure{
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}


#[derive(Debug)]
pub struct ChessBoard{
    turns: u32,
    on_turn: Color,
    board: [Option<ChessFigure>; 64]
}

impl ChessBoard{

    pub fn defualt() -> Self{
        Self{
            turns: 0,
            on_turn: Color::White,
            board: [Option::None.clone();64]
        }
    }
}