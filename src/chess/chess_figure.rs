use super::Color;
impl ChessFigureMetaData {
    pub fn from_fen(c: char) -> Option<ChessFigureMetaData> {
        for cfm in CHESS_FIGURE_META_DATAS {
            if cfm.fen_char == c {
                return Some(cfm);
            }
        }
        return None;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChessFigure {
    Pawn(Color),
    Knight(Color),
    Bishop(Color),
    Rook(Color),
    Queen(Color),
    King(Color),
}

impl ChessFigure {
    pub fn static_stats(&self) -> &ChessFigureMetaData {
        match self {
            Self::Pawn(_) => &PAWN_DATA,
            Self::Knight(_) => &KNIGHT_DATA,
            Self::Bishop(_) => &BISHOP_DATA,
            Self::Rook(_) => &ROOK_DATA,
            Self::Queen(_) => &QUEEN_DATA,
            Self::King(_) => &KING_DATA,
        }
    }

    pub fn fen(&self) -> char{
        let c = match self{
            ChessFigure::Pawn(c) => c,
            ChessFigure::Knight(c) => c,
            ChessFigure::Bishop(c) =>c,
            ChessFigure::Rook(c) => c,
            ChessFigure::Queen(c) =>c,
            ChessFigure::King(c) =>c,
        };
        let s = self.static_stats();
        match c{
            Color::Black => s.fen_char,
            Color::White => s.fen_char.to_ascii_uppercase(),
        }
    }

    pub fn from_fen(fen: char) -> Option<Self> {
        let t = ChessFigureMetaData::from_fen(fen.to_ascii_lowercase());
        if t.is_none() {
            return None;
        }
        let color = char::is_ascii_uppercase(&fen);
        let c;
        if color {
            c = Color::White;
        } else {
            c = Color::Black
        }

        let t = t.unwrap();
        let ret = match t.chess_figure {
            ChessFigure::Pawn(_) => ChessFigure::Pawn(c),
            ChessFigure::Knight(_) => ChessFigure::Knight(c),
            ChessFigure::Bishop(_) => ChessFigure::Bishop(c),
            ChessFigure::Rook(_) => ChessFigure::Rook(c),
            ChessFigure::Queen(_) => ChessFigure::Queen(c),
            ChessFigure::King(_) => ChessFigure::King(c),
        };
        return Some(ret);
    }
}






struct ChessFigureMetaData {
    value: u16,
    chess_figure: ChessFigure,
    fen_char: char,
    value_map: [i8; 64],
}

const CHESS_FIGURE_META_DATAS: [ChessFigureMetaData; 6] = [
    PAWN_DATA,
    KNIGHT_DATA,
    BISHOP_DATA,
    ROOK_DATA,
    QUEEN_DATA,
    KING_DATA,
];

const PAWN_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigure::Pawn(Color::Black),
    value: 100,
    fen_char: 'p',
    value_map: [
        0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5,
        5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10,
        -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
};
const KNIGHT_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigure::Knight(Color::Black),
    value: 300,
    fen_char: 'n',
    value_map: [
        -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15,
        10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15,
        15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
    ],
};

const BISHOP_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigure::Bishop(Color::Black),
    value: 300,
    fen_char: 'b',
    value_map: [
        -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5,
        0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10,
        10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
    ],
};
const ROOK_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigure::Rook(Color::Black),
    value: 500,
    fen_char: 'r',
    value_map: [
        0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
        0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0,
        -5, 0, 0, 0, 5, 5, 0, 0, 0,
    ],
};

const QUEEN_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigure::Queen(Color::Black),
    value: 900,
    fen_char: 'q',
    value_map: [
        -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0,
        -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0,
        5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
    ],
};
const KING_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigure::King(Color::Black),
    value: u16::MAX,
    fen_char: 'k',
    value_map: [
        -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
        -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40,
        -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20,
        30, 10, 0, 0, 10, 30, 20,
    ],
};