pub struct RandomGenerator{
    seed: u32
}

pub trait Random{
    fn next_int(&mut self) -> u32;
    fn range(&mut self,from: u32, to: u32) -> u32;
}

impl Random for RandomGenerator{
    fn next_int(&mut self) -> u32 {
        self.seed = (self.seed * 1103515245 + 12345) & 0x7fffffff;
        self.seed
    }

    fn range(&mut self,from: u32, to: u32) -> u32{
        from + (self.next_int() % to)
    }
}

pub fn create_random(seed: u32) -> RandomGenerator{
    RandomGenerator{
        seed: seed
    }
}


pub fn rgb(r: u8, g: u8, b: u8) -> u32{
    rgba(r, g, b, 255)
}

pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> u32{
    u32::from_be_bytes([a,b,g,r])
}