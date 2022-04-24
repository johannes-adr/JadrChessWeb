use chess_rs::chess_board::{self, ChessBoard};

fn do_move_rec(depth: u8,board: &mut ChessBoard) -> usize{
    let mut cntr = 0;
    if depth == 0{
        return 0;
    }
    let moves = board.get_moves();
    for m in moves{
        let undo = m.make_move(board);
        cntr+=1;
        cntr += do_move_rec(depth-1, board);
        undo.undo(board);
    }
    return cntr
}

fn main(){
    let mut b = chess_board::ChessBoard::default();
    // let res = ChessBoard::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");
    // b = res.unwrap();
    // println!("{}",b.as_str());
    use std::time::Instant;
    let now = Instant::now();
    let ctr = do_move_rec(3, &mut b);
    
    println!("Elapsed: {:.2?}", now.elapsed());
    println!("{}",ctr);
    // println!("{:#?}",b.get_figure(3, 0).unwrap().get_avaible_moves(&b).len())
}


fn time_function(func: fn()){
   
    let mut mid = 0;
    // Code block to measure.
    let runs = 100;
    for _ in 0..runs{
        use std::time::Instant;
        let now = Instant::now();
        for _ in 0..100{
            func();
        }
        let elapsed = now.elapsed();
        mid+=elapsed.as_micros();
    }
    println!("Elapsed: {:.2?}", mid / runs);
}