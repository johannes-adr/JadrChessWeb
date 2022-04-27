use std::{time::Instant, mem::size_of};

use chess_board::ChessBoard;
use color::Color;


pub mod color;
pub mod chess_figure;
pub mod chess_board;

mod utils;
fn main(){
    let mut b = chess_board::ChessBoard::default();
    let before = Instant::now();
    fn play_rec_recursive(c: Color, b: &mut ChessBoard,mut depth: u8) -> usize{
        depth-=1;
        let moves = b.get_moves_for_side(c);
        let mut count = 0;
        let cother = if c.equals(Color::Black){
            Color::White
        }else{
            Color::Black
        };
        for m in moves{
            
            // timer(200).await;
            let undo = m.make_move(b);
            // print(b.as_str());
            
            if depth > 0{
                count += play_rec_recursive(cother, b, depth);
            }else{
                count+=1;
            }
            // timer(200).await;
            undo.undo(b);
        }
        return count;
    }
    let moves = play_rec_recursive(color::Color::White, &mut b, 5);
    println!("Moves done: {}",moves.to_string());
    print!("Elapsed time: {:.2?}", before.elapsed())
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