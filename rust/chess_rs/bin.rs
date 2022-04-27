use std::{time::Instant, mem::size_of};

use chess_board::ChessBoard;
use color::Color;


pub mod color;
pub mod chess_figure;
pub mod chess_board;

mod utils;
fn main(){
    let mut b = chess_board::ChessBoard::default();
    // let before = Instant::now();
    // println!("{}",make_moves_rec(&mut b, 3, Color::White));
    // print!("Elapsed time: {:.2?}", before.elapsed())
}

// fn make_moves_rec(cb: &mut ChessBoard, depth: u8, side: Color) -> usize{
//     let mut cnt = 0;
//     if depth == 0{
//         return cnt
//     }
//     let cother = if side.equals(Color::Black){
//         Color::White
//     }else{
//         Color::Black
//     };
    
//     for m in cb.get_moves_for_side(side){
//         let undo = m.make_move(cb);
//         cnt += 1;
//         cnt += make_moves_rec(cb, depth-1, cother);
//         undo.undo(cb.board())
//     }
//     return cnt;
// }