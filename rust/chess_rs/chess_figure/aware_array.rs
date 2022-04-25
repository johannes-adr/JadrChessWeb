use arrayvec::ArrayVec;

use super::figure_move::Point;


struct AwareElement<T: Sized>{
    element: T,
    position: Option<Point>
}

struct AwareArray2D<'a, T: Sized, const WIDTH: usize, const HEIGHT: usize>{
    arr: [[Option<&'a mut AwareElement<T>>;WIDTH]; HEIGHT]
}


impl <'a,T: Sized, const WIDTH: usize, const HEIGHT: usize>AwareArray2D<'a, T, WIDTH, HEIGHT>{
    pub fn new() -> Self{
        Self{
            arr: [[None;WIDTH];HEIGHT]
        }
    }

    pub fn create_aware_container<const SIZE: usize>(&mut self) -> ArrayVec<AwareElement<T>, SIZE>{
        ArrayVec::new()
    }
    /**
     * Set position of possible element to `None`
     * Set field to `None`
     * returns Optional Element on field before delete
     */
    pub fn delete_field(&mut self, coords: Point) -> Option<&mut AwareElement<T>> {
        let field = &mut self.arr[coords.y()][coords.x()];
        let before = *field;
        if let Some(f) = before{
            f.position = None;
            *field = None;
        }
        before
    }

    pub fn set_element(&mut self,e: &'a mut AwareElement<T>, coords: Point) -> Option<&mut AwareElement<T>>{
        let before = self.delete_field(coords);
        e.position = Some(coords);
        self.arr[coords.y()][coords.x()] = Some(e);
        before
    }
}

pub fn test(){
    let chessboard = AwareArray2D::<i32,8,8>::new();
    let mut container = chessboard.create_aware_container::<16>();
    // chessboard.set_element(container.get_mut(0).unwrap(), 0, 0);
    
}