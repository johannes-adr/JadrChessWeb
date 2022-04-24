use super::metadata::ChessFigureMetaData;

#[derive(Debug, Clone, Copy)]
pub enum ChessFigureType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ChessFigureType{
    pub fn stats(self) -> &'static ChessFigureMetaData{
        ChessFigureMetaData::from_figure_type(self)
    }

    pub fn from_fen(fen: char) -> Option<ChessFigureType>{
        if let Some(f) = ChessFigureMetaData::from_fen(fen){
            Some(f.chess_figure)
        }else{
            None
        }
    }
}
