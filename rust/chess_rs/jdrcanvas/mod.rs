use std::collections::HashMap;


pub struct DrawBuffer<'a> {
    pub buffer: &'a mut Vec<u32>,
    pub width: usize,
    pub height: usize,
}

pub struct RenderableList<'a>{
    map: HashMap<i32,Vec<Box<&'a dyn Renderable<'a>>>>
}

impl  <'a> RenderableList <'a> {
    pub fn add(&mut self, ra: &'a dyn Renderable<'a>){
        let z_index = ra.z_index();
        let m = &mut self.map;
        let mut res = m.get_mut(&z_index);
        if res.is_none(){
            let mut llist:Vec<Box<&'a dyn Renderable<'a>>> = vec![];
            // res.insert(&mut llist);
            // m.insert(z_index, llst);
        }
        res.unwrap().push(Box::new(ra));
    }
}

impl DrawBuffer<'_> {
    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        let wb = self.width;
        let hb = self.height;
        if x + w > wb {
            panic!("DrawBuffer x + width outside of image")
        } else if y + h > hb {
            panic!("DrawBuffer y + height outside of image")
        }
        let h = y + h;
        let w = x + w;
        for y in y..h {
            for x in x..w {
                self.buffer[y * wb + x] = color;
            }
        }
    }

    pub fn draw_line(&mut self, mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize, width: usize, color: u32){
        if x1 > x2{
            let x1c = x1;
            x1 = x2;
            x2 = x1c;
        }

        if y1 > y2{
            let y1c = y1;
            y1 = y2;
            y2 = y1c;
        }
        let yadd = (y2 - y1) as f64 / (x2 - x1) as f64;
        let wb = self.width;
        let hb = self.height;
        let mut y = y1 as f64;
        for x in x1..x2{
            for xr in x-width..x+width{
                let y = y as usize;
                for yr in y-width..y+width{
                    self.buffer[ yr as usize* wb + xr] = color;
                }
            }

            
            y+=yadd;
        }
    }

    pub fn set_all(&mut self, color: u32) {
        for i in 0..(self.width * self.height) as usize {
            self.buffer[i] = color;
        }
    }

    pub fn coord(&mut self, x: usize, y: usize) -> usize{
        y*self.width + x
    }
}

trait Renderable<'a> {
    fn render(&self, drawbuffer: DrawBuffer);
    fn z_index(&self) -> i32;
}
