use crate::{
    chess_board::{ChessBoard, FenBuilder},
    color::Color,
};

use super::{figure_type::ChessFigureType, ChessFigure};

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    Castle,
    Promote,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i8,
    y: i8,
}

impl Point {
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn new(x: usize, y: usize) -> Self {
        Point {
            x: x as i8,
            y: y as i8,
        }
    }

    pub fn x(&self) -> usize {
        self.x as usize
    }

    pub fn x_raw(&self) -> i8 {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y as usize
    }

    pub fn y_raw(&self) -> i8 {
        self.y
    }
}

#[derive(Debug)]
pub struct Move {
    start: Point,
    target: Point,
    flag: Option<Flag>,
}


impl Move {
    pub fn new(start: Point, tx: usize, ty: usize, flag: Option<Flag>) -> Self {
        Move {
            start: start,
            target: Point::new(tx, ty),
            flag: flag,
        }
    }

    pub fn start(&self) -> Point {
        self.start
    }
    pub fn target(&self) -> Point {
        self.target
    }
    pub fn flag(&self) -> Option<Flag> {
        self.flag
    }

    pub fn make_move(&self, board: &mut ChessBoard) -> UndoMove{
        let s = self.start;
        let t = self.target;
        let mut fig = board.get_figure(s.x(), s.y()).unwrap();
        let fig_cpy = fig.clone();
        board.set_field(s.x(), s.y(), None);
        fig.set_pos(t.x(), t.y());
        fig.flag_moved();
        
        let target = board.get_figure(t.x(), t.y());
        board.set_field(t.x(), t.y(), Some(fig));
        UndoMove{
            start: fig_cpy,
            target,
            target_point: t
        }
        // board.move_figure(t.x(), t.y(), f);
    }
}

pub struct UndoMove{
    start: ChessFigure,
    target: Option<ChessFigure>,
    target_point: Point
}

impl UndoMove{
    pub fn undo(&self,board: &mut ChessBoard){
        let s = &self.start;
        let sp = s.pos;
        let t = &self.target;
        let tp = self.target_point;
        board.set_field(sp.x(), sp.y(), Some(s.clone()));
        board.set_field(tp.x(), tp.y(), t.clone());
    }
}

impl Default for Move {
    fn default() -> Self {
        Self {
            start: Point::zero(),
            target: Point::zero(),
            flag: None,
        }
    }
}

pub fn generate_move(fig: &ChessFigure, board: &ChessBoard) -> Vec<Move> {
    match fig.figure {
        ChessFigureType::Pawn => pawn(fig, board),
        ChessFigureType::Knight => knight(fig, board),
        ChessFigureType::Rook => rook(fig, board),
        ChessFigureType::Bishop => bishop(fig, board),
        ChessFigureType::Queen => {
            let mut v1 = rook(fig, board);
            let mut v2 = bishop(fig, board);
            let mut v3 = Vec::with_capacity(v1.len() + v2.len());
            v3.append(&mut v1);
            v3.append(&mut v2);
            v3
        }
        ChessFigureType::King => king(fig, board),
        _ => Vec::new(),
    }
}

fn in_bounds(x: i8) -> bool {
    x >= 0 && x <= 7
}

fn pawn(fig: &ChessFigure, board: &ChessBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(4);
    let move_direction = if let Color::White = fig.color() {
        -1
    } else {
        1
    };
    let pos = fig.pos;
    let y = pos.y_raw();
    let y_one = y + move_direction;
    let x = pos.x as usize;
    let xi = pos.x;

    let promotion = if y_one == 0 || y_one == 7 {
        Some(Flag::Promote)
    } else {
        None
    };

    if !in_bounds(y_one) {
        return moves;
    }

    //Single jump
    if board.empty_at(x, y_one as usize) {
        moves.push(Move::new(pos, x, y_one as usize, promotion));

        //Double jump
        let double_new_y = (y + move_direction * 2) as usize;
        if !fig.already_moved && board.empty_at(x, double_new_y) {
            moves.push(Move::new(pos, x, double_new_y, promotion));
        }
    }

    // Right / left attack

    if in_bounds(xi + 1) && board.moveable(x + 1, y_one as usize, fig) == 1 {
        moves.push(Move::new(pos, x + 1, y_one as usize, promotion))
    }

    if in_bounds(xi - 1) && board.moveable(x - 1, y_one as usize, fig) == 1 {
        moves.push(Move::new(pos, x - 1, y_one as usize, promotion))
    }
    moves
}

fn knight(fig: &ChessFigure, board: &ChessBoard) -> Vec<Move> {
    // (x, y)
    const KNIGHT_DELTAS: [(i8, i8); 8] = [
        (-2, -1),
        (-2, 1),
        (2, -1),
        (2, 1),
        (-1, -2),
        (-1, 2),
        (1, -2),
        (1, 2),
    ];
    let mut moves: Vec<Move> = Vec::with_capacity(8);
    for (x, y) in KNIGHT_DELTAS {
        let pos = fig.get_pos();
        let x = pos.x + x;
        let y = pos.y + y;
        if !in_bounds(x) || !in_bounds(y) {
            continue;
        }
        let x = x as usize;
        let y = y as usize;
        if board.moveable(x, y, fig) != 0 {
            moves.push(Move::new(pos, x, y, None))
        }
    }
    moves
}

fn rook(fig: &ChessFigure, board: &ChessBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(7 + 8);
    let pos = fig.pos;
    let y = pos.y;
    let x = pos.x;
    debug_assert!(in_bounds(x));
    let x = x as usize;

    //DOWN
    for y in y + 1..8 {
        debug_assert!(in_bounds(y));
        let y = y as usize;
        let res = board.moveable(x, y, fig);

        //if field is not with ally figure
        if res != 0 {
            moves.push(Move::new(pos, x, y, None));
        }

        //if field is not empty
        if res != 2 {
            break;
        }
    }
    //UP
    for y in (0..y).rev() {
        debug_assert!(in_bounds(y));
        let y = y as usize;
        let res = board.moveable(x, y, fig);

        //if field is not with ally figure
        if res != 0 {
            moves.push(Move::new(pos, x, y, None));
        }

        //if field is not empty
        if res != 2 {
            break;
        }
    }
    debug_assert!(in_bounds(y));
    let y = y as usize;

    //RIGHT
    for x in (x + 1)..8 {
        let res = board.moveable(x, y, fig);
        //if field is not with ally figure
        if res != 0 {
            moves.push(Move::new(pos, x, y, None));
        }

        //if field is not empty
        if res != 2 {
            break;
        }
    }

    //LEFT
    for x in (0..x).rev() {
        let res = board.moveable(x, y, fig);
        //if field is not with ally figure
        if res != 0 {
            moves.push(Move::new(pos, x, y, None));
        }

        //if field is not empty
        if res != 2 {
            break;
        }
    }
    moves
}

fn bishop(fig: &ChessFigure, board: &ChessBoard) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::with_capacity(7 + 8);
    let pos = fig.pos;
    let y = pos.y;
    let x = pos.x;
    //RIGHT DOWN
    let mut i = 1;
    for y in y + 1..8 {
        let x = x + i;
        i += 1;
        if !in_bounds(x){
            break;
        }

        debug_assert!(in_bounds(y));
        let x = x as usize;
        let y = y as usize;
        let res = board.moveable(x, y, fig);
        //if field is not with ally figure
        if res != 0 {
            moves.push(Move::new(pos, x, y, None));
        }

        //if field is not empty
        if res != 2 {
            break;
        }
    }

    //LEFT DOWN
    let mut i = -1;
    for y in y + 1..8 {
        let x = x + i;
        i -= 1;
        if !in_bounds(x){
            break;
        }
        debug_assert!(in_bounds(y));
        let x = x as usize;
        let y = y as usize;
        let res = board.moveable(x, y, fig);
        //if field is not with ally figure
        if res != 0 {
            moves.push(Move::new(pos, x, y, None));
        }

        //if field is not empty
        if res != 2 {
            break;
        }
    }

    //RIGHT UP
    let mut i = 1;
    for y in (0..y).rev() {
        let x = x + i;
        i += 1;
        if !in_bounds(x){
            break;
        }
        debug_assert!(in_bounds(y));
        let x = x as usize;
        let y = y as usize;
        let res = board.moveable(x, y, fig);
        //if field is not with ally figure
        if res != 0 {
            moves.push(Move::new(pos, x, y, None));
        }

        //if field is not empty
        if res != 2 {
            break;
        }
    }

    //LEFT UP
    let mut i = -1;
    for y in (0..y).rev() {
        let x = x + i;
        i -= 1;
        if !in_bounds(x){
            break;
        }
        debug_assert!(in_bounds(y));
        let x = x as usize;
        let y = y as usize;
        let res = board.moveable(x, y, fig);
        //if field is not with ally figure
        if res != 0 {
            moves.push(Move::new(pos, x, y, None));
        }

        //if field is not empty
        if res != 2 {
            break;
        }
    }

    moves
}
fn king(fig: &ChessFigure, board: &ChessBoard) -> Vec<Move> {
    const KING_DELTAS: [(i8, i8); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut moves: Vec<Move> = Vec::with_capacity(10);
    let p = fig.pos;

    for (x,y) in KING_DELTAS{
        let x = p.x+x;
        let y = p.y+y;
        if !in_bounds(x) || !in_bounds(y){
            continue;
        }
        let x = x as usize;
        let y = y as usize;
        if board.moveable(x, y, fig) != 0{
            moves.push(Move::new(p, x, y, None))
        }
    }
    
    //Castle

    moves
}
