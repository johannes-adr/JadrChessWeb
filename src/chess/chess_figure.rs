struct ChessFigureMetaData{
    value: u16,
    is_king: bool,
    fen_char: char,
    value_map: [i8; 64],
}

const PAWN_DATA: ChessFigureMetaData = ChessFigureMetaData{value: 100,is_king: false, fen_char: 'p', value_map: [0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5, 10,
25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20, -20, 10, 10,
5, 0, 0, 0, 0, 0, 0, 0, 0]};
const KNIGHT_DATA: ChessFigureMetaData = ChessFigureMetaData{value: 300,is_king: false, fen_char: 'n', value_map: [-50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10, 5, -30,
-40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50]};

const BISHOP_DATA: ChessFigureMetaData = ChessFigureMetaData{value: 300,is_king: false, fen_char: 'b', value_map:[ -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
-10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10, -10,
-10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20]};
const ROOK_DATA: ChessFigureMetaData = ChessFigureMetaData{value: 500,is_king: false, fen_char: 'r', value_map:[0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 5, 5,
0, 0, 0]};

const QUEEN_DATA: ChessFigureMetaData = ChessFigureMetaData{value: 900,is_king: false, fen_char: 'q', value_map:[-20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
-5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0, 0, 0,
-10, -20, -10, -10, -5, -5, -10, -10, -20]};
const KING_DATA: ChessFigureMetaData = ChessFigureMetaData{value: u16::MAX,is_king: true, fen_char: 'k', value_map:[ -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40,
-50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40, -40, -30, -30, -20,
-10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20, 30, 10, 0, 0, 10, 30, 20]};

use super::{Color, ChessFigure};



impl ChessFigure{
    
    pub fn static_stats(&self) -> &ChessFigureMetaData{
        match self{
            ChessFigure::Pawn(_) => &PAWN_DATA,
            ChessFigure::Knight(_) => &KNIGHT_DATA,
            ChessFigure::Bishop(_) => &BISHOP_DATA,
            ChessFigure::Rook(_) => &ROOK_DATA,
            ChessFigure::Queen(_) => &QUEEN_DATA,
            ChessFigure::King(_) => &KING_DATA,
        }
    }

}