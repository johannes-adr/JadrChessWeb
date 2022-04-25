pub mod color;
pub mod chess_figure;
pub mod chess_board;

mod jdrcanvas;
mod utils;



use chess_board::ChessBoard;
use jdrcanvas::*;
use wasm_bindgen::prelude::*;
use utils::rgb;

const WIDTH: usize = 700;
const HEIGHT: usize = 700;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Game {
    pub start_time: u32,
    pub width: usize,
    pub height: usize,
    pub img_buf_ptr:  *const u32,
    buffer: &'static mut Vec<u32>,
    active_keys: [bool;255],
    chess_board: ChessBoard
    // renderables: &'static mut Vec<dyn >
}
#[wasm_bindgen]
impl Game {
    fn update(&mut self){

    }

    fn render(&mut self){
        let mut draw_buffer = DrawBuffer{buffer: self.buffer, width: self.width, height: self.height};
        let cellw = draw_buffer.width / 8;
        let cellh = draw_buffer.height / 8;
        

        let mut cell_black = false;
        let cblack = rgb(210,139,71);
        let cwhite = rgb(254,206,157);
        for y in 0..8{
            for x in 0..8{
                draw_buffer.fill_rect(x*cellw, y*cellh, cellw, cellh, if cell_black{
                    cblack
                }else{
                    cwhite
                });

                let s = self.chess_board.get_figure(x, y);
                if let Some(f) = s{
                    let bgc;
                    let fgc;
                    if f.color().equals(color::Color::Black){
                        bgc = rgb(0, 0, 0);
                        fgc = rgb(255, 255, 255)
                    }else{
                        fgc = rgb(0, 0, 0);
                        bgc = rgb(255, 255, 255)
                    };
                    let cellx = x*cellw+10;
                    let celly = y*cellh + 10;
                    let cellx_max = cellw - 20;
                    let celly_max = cellh - 20;
                    draw_buffer.fill_rect(cellx, celly, cellx_max, celly_max, bgc);
                    match f.figure(){
                        chess_figure::figure_type::ChessFigureType::Pawn => {},
                        chess_figure::figure_type::ChessFigureType::Knight => {},
                        chess_figure::figure_type::ChessFigureType::Bishop => {
                            draw_buffer.draw_line(cellx, celly_max, cellx, celly, 2, fgc)
                        },
                        chess_figure::figure_type::ChessFigureType::Rook => {
                            // let x = cellx;
                            // draw_buffer.draw_line(x, celly, x, celly+celly_max, 2, rgb(123, 123, 123))
                        },
                        chess_figure::figure_type::ChessFigureType::Queen => {},
                        chess_figure::figure_type::ChessFigureType::King => {},
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
    pub fn keyevent(&mut self, key: char, is_pressed: bool){
        self.active_keys[key as usize] = is_pressed;
    }
}

fn constructor_game(start_time: u32, width: usize, height: usize) -> Game {
    static mut buf: Vec<u32> = Vec::new();
    unsafe{
        buf.resize(width*height, 0xff_AA_AA_AA);
        Game {
            start_time: start_time,
            width: width,
            height: height,
            buffer: &mut buf,
            img_buf_ptr: buf.as_ptr(),
            active_keys: [false;255],
            chess_board: ChessBoard::default()
        }
    }
}

#[wasm_bindgen]
pub fn start(start_time: u32) -> Game {
    let g = constructor_game(start_time, WIDTH, HEIGHT);

    let b = ChessBoard::default();
    return g
}
