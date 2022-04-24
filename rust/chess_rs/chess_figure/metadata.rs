use super::figure_type::ChessFigureType;

pub struct ChessFigureMetaData {
    pub value: u16,
    pub chess_figure: ChessFigureType,
    pub fen_char: char,
    pub value_map: [i8; 64],
}

impl ChessFigureMetaData {
    pub fn from_fen(c: char) -> Option<ChessFigureMetaData> {
        for cfm in CHESS_FIGURE_META_DATAS {
            if cfm.fen_char == c {
                return Some(cfm);
            }
        }
        return None;
    }

    pub fn from_figure_type(figure: ChessFigureType) -> &'static ChessFigureMetaData{
            match figure {
                ChessFigureType::Pawn => &PAWN_DATA,
                ChessFigureType::Knight => &KNIGHT_DATA,
                ChessFigureType::Bishop=> &BISHOP_DATA,
                ChessFigureType::Rook => &ROOK_DATA,
                ChessFigureType::Queen=> &QUEEN_DATA,
                ChessFigureType::King=> &KING_DATA,
            }
        }
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
    chess_figure: ChessFigureType::Pawn,
    value: 100,
    fen_char: 'p',
    value_map: [
        0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5,
        5, 10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10,
        -20, -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
};
const KNIGHT_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigureType::Knight,
    value: 300,
    fen_char: 'n',
    value_map: [
        -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15,
        10, 0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15,
        15, 10, 5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
    ],
};

const BISHOP_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigureType::Bishop,
    value: 300,
    fen_char: 'b',
    value_map: [
        -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5,
        0, -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10,
        10, 10, -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
    ],
};
const ROOK_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigureType::Rook,
    value: 500,
    fen_char: 'r',
    value_map: [
        0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0,
        0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0,
        -5, 0, 0, 0, 5, 5, 0, 0, 0,
    ],
};

const QUEEN_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigureType::Queen,
    value: 900,
    fen_char: 'q',
    value_map: [
        -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0,
        -10, -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0,
        5, 0, 0, 0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
    ],
};
const KING_DATA: ChessFigureMetaData = ChessFigureMetaData {
    chess_figure: ChessFigureType::King,
    value: u16::MAX,
    fen_char: 'k',
    value_map: [
        -30, -40, -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -30, -40,
        -40, -50, -50, -40, -40, -30, -30, -40, -40, -50, -50, -40, -40, -30, -20, -30, -30, -40,
        -40, -30, -30, -20, -10, -20, -20, -20, -20, -20, -20, -10, 20, 20, 0, 0, 0, 0, 20, 20, 20,
        30, 10, 0, 0, 10, 30, 20,
    ],
};