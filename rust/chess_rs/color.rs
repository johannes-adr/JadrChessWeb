
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

    pub fn from_char(c: char) -> Self {
        if c == 'w' {
            Color::White
        } else if c == 'b' {
            Color::Black
        } else if c.is_uppercase(){
            Color::White
        }else{
            Color::Black
        }
    }

    pub fn equals(&self, other: Color) -> bool{
        if let Color::Black = other{
            if let Color::Black = self{
                return true;
            }
        }
        if let Color::White = other{
            if let Color::White = self{
                return true;
            }
        }

        false
    }
}
