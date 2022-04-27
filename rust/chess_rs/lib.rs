pub mod chess_board;
pub mod chess_figure;
pub mod color;

mod jdrcanvas;
mod utils;

use std::{
    cell::{RefCell, RefMut, UnsafeCell},
    rc::Rc,
    slice,
    sync::{Arc, Mutex}, borrow::{Borrow, BorrowMut},
};
use async_recursion::async_recursion;
use chess_board::ChessBoard;
use jdrcanvas::*;
use js_sys::Promise;
use once_cell::sync::Lazy;
use utils::rgb;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture, future_to_promise};
use web_sys::window;

const WIDTH: usize = 700;
const HEIGHT: usize = 700;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(module = "/index.js")]
extern "C" {
    fn sleep(ms: i32) -> Promise;
}

fn print<S: AsRef<str>>(s:S ) {
    unsafe { log(s.as_ref()) }
}

pub async fn timer(ms: i32) -> Result<(), JsValue> {
    let promise = Promise::new(&mut |yes, _| {
        let win = window().unwrap();
        win.set_timeout_with_callback_and_timeout_and_arguments_0(&yes, ms)
            .unwrap();
    });
    let js_fut = JsFuture::from(promise);
    js_fut.await?;
    Ok(())
}

#[wasm_bindgen]
#[derive(Clone,Debug)]
pub struct RcBoard{
    val: Rc<UnsafeCell<ChessBoard>>
}


impl RcBoard{
    pub fn new(g: ChessBoard) -> Self{
        Self{val: Rc::new(UnsafeCell::new(g))}
    }
}

#[wasm_bindgen]
pub struct Game {
    pub start_time: u32,
    pub width: usize,
    pub height: usize,
    pub img_buf_ptr: *const u32,
    buffer: &'static mut Vec<u32>,
    active_keys: [bool; 255],
    chess_board: RcBoard, // renderables: &'static mut Vec<dyn >
}
#[wasm_bindgen]
pub async fn play_rec(board: RcBoard) {
    let val = board.val.clone();
    let mut b = unsafe{val.get().as_mut()}.unwrap();

    #[async_recursion(?Send)]
    async fn play_rec_recursive(c: color::Color, b: &mut ChessBoard, depth: u8) -> usize{
        let moves = b.get_moves_for_side(c);
        let mut count = 0;
        if depth == 0{
            return 0;
        }
        let cother = if c.equals(color::Color::Black){
            color::Color::White
        }else{
            color::Color::Black
        };
        for m in moves{
            
            timer(200).await;
            let undo = m.make_move(b);
            // print(b.as_str());
            count+=1;
            count += play_rec_recursive(cother, b, depth-1).await;
            timer(200).await;
            undo.undo(b);
        }
        return count;
    }
    let moves = play_rec_recursive(color::Color::White, &mut b, 2).await;
    print(moves.to_string())
}

#[wasm_bindgen]
impl Game {
    // pub async fn play_rec(&self) -> Promise {
    //     timer(1000).await;
    //     future_to_promise(async move{
    //         Ok(JsValue::UNDEFINED)
    //     })
        
    // }

    fn update(&mut self) {}

    pub fn get_board(&self) -> RcBoard {
        self.chess_board.clone()
    }

    fn render(&mut self) {
        let mut draw_buffer = DrawBuffer {
            buffer: self.buffer,
            width: self.width,
            height: self.height,
        };
        let cellw = draw_buffer.width / 8;
        let cellh = draw_buffer.height / 8;

        let mut cell_black = false;
        let cblack = rgb(210, 139, 71);
        let cwhite = rgb(254, 206, 157);
        let b = unsafe {self.chess_board.val.get().as_mut().unwrap() };
        for y in 0..8 {
            for x in 0..8 {
                draw_buffer.fill_rect(
                    x * cellw,
                    y * cellh,
                    cellw,
                    cellh,
                    if cell_black { cblack } else { cwhite },
                );

                let s = b.get_figure(x, y);
                if let Some(f) = s {
                    let bgc;
                    let fgc;
                    if f.color().equals(color::Color::Black) {
                        bgc = rgb(0, 0, 0);
                        fgc = rgb(255, 255, 255)
                    } else {
                        fgc = rgb(0, 0, 0);
                        bgc = rgb(255, 255, 255)
                    };
                    let cellx = x * cellw + 10;
                    let celly = y * cellh + 10;
                    let cellx_max = cellw - 20;
                    let celly_max = cellh - 20;
                    draw_buffer.fill_rect(cellx, celly, cellx_max, celly_max, bgc);
                    match f.figure() {
                        chess_figure::figure_type::ChessFigureType::Pawn => {}
                        chess_figure::figure_type::ChessFigureType::Knight => {}
                        chess_figure::figure_type::ChessFigureType::Bishop => {
                            draw_buffer.draw_line(cellx, celly_max, cellx, celly, 2, fgc)
                        }
                        chess_figure::figure_type::ChessFigureType::Rook => {
                            // let x = cellx;
                            // draw_buffer.draw_line(x, celly, x, celly+celly_max, 2, rgb(123, 123, 123))
                        }
                        chess_figure::figure_type::ChessFigureType::Queen => {}
                        chess_figure::figure_type::ChessFigureType::King => {}
                    };
                }
                cell_black = !cell_black;
            }
            cell_black = !cell_black;
        }
    }

    pub fn js_tick(&mut self, time: u32) {
        self.render();
        self.update();
    }
    pub fn keyevent(&mut self, key: char, is_pressed: bool) {
        self.active_keys[key as usize] = is_pressed;
    }

    pub fn get_game() {}
}

fn constructor_game(start_time: u32, width: usize, height: usize) -> Game {
    static mut buf: Vec<u32> = Vec::new();
    let g = unsafe {
        buf.resize(width * height, 0xff_AA_AA_AA);
        Game {
            start_time: start_time,
            width: width,
            height: height,
            buffer: &mut buf,
            img_buf_ptr: buf.as_ptr(),
            active_keys: [false; 255],
            chess_board: RcBoard::new(ChessBoard::default()),
        }
    };

    g
}

#[wasm_bindgen]
pub fn start(start_time: u32) -> Game {
    let g = constructor_game(start_time, WIDTH, HEIGHT);

    return g;
}
