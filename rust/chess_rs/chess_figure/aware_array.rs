// use std::{
//     borrow::BorrowMut,
//     cell::{RefCell, UnsafeCell},
// };

// use arrayvec::ArrayVec;

// use super::figure_move::Point;

// type LookUpElement<T> = Option<UnsafeCell<T>>;

// #[derive(Debug)]
// pub struct AwareArray2D<
//     T: Clone,
//     const WIDTH: usize,
//     const HEIGHT: usize,
//     const AMOUNT_CONTAINERS: usize,
//     const CONTAINER_SIZE: usize,
// > {
//     look_up_table: [[LookUpElement<T>; WIDTH]; HEIGHT],
//     containers: ArrayVec<ArrayVec<UnsafeCell<AwareElement<T>>, CONTAINER_SIZE>, AMOUNT_CONTAINERS>,
// }

// impl<
//         T: Clone,
//         const WIDTH: usize,
//         const HEIGHT: usize,
//         const AMOUNT_CONTAINERS: usize,
//         const CONTAINER_SIZE: usize,
//     > AwareArray2D<T, WIDTH, HEIGHT, AMOUNT_CONTAINERS, CONTAINER_SIZE>
// {
//     pub fn new() -> AwareArray2D<T, WIDTH, HEIGHT, AMOUNT_CONTAINERS, CONTAINER_SIZE> {
//         let mut containers = ArrayVec::<ArrayVec<_, CONTAINER_SIZE>, AMOUNT_CONTAINERS>::new();
//         for _ in 0..AMOUNT_CONTAINERS {
//             containers.push(ArrayVec::new())
//         }
//         Self {
//             look_up_table: [[None; WIDTH]; HEIGHT],
//             containers: containers,
//         }
//     }

//     pub fn get_container(&self, container_id: usize) -> &ArrayVec<UnsafeCell<AwareElement<T>>, CONTAINER_SIZE> {
//         &self.containers[container_id]
//     }

//     pub fn add_element(&mut self, container_id: usize, t: T, coords: Point) {
//         let cntr = self.containers.get_mut(container_id).unwrap();
//         let uc = UnsafeCell::new(AwareElement {
//             pos: Some(coords),
//             element: t,
//         }); 
//         cntr.push(uc);
//         let elem = cntr.get(cntr.len()-1).unwrap();
//         // let ptr = unsafe { cntr.get_mut(len).unwrap() } as *mut AwareElement<T>;
//         elem.g
//         *self.field_mut(coords) = Some(elem.borrow_mut());
//     }

//     pub fn get_field_mut(&mut self, coords: Point) -> Option<&mut AwareElement<T>> {
//         unsafe { self.look_up_table[coords.y()][coords.x()].as_mut() }
//     }

//     pub fn get_field(&self, coords: Point) -> Option<&AwareElement<T>> {
//         unsafe { self.look_up_table[coords.y()][coords.x()].as_ref() }
//     }

//     fn field_mut(&mut self, coords: Point) -> &mut LookUpElement<T> {
//         &mut self.look_up_table[coords.y()][coords.x()]
//     }

//     fn field(&self, coords: Point) -> LookUpElement<T> {
//         self.look_up_table[coords.y()][coords.x()]
//     }

//     /**
//      * Set position of possible element to `None`
//      * Set field to `None`
//      * returns Optional Element on field before delete
//      */
//     pub fn delete_field(&mut self, coords: Point) -> Option<&AwareElement<T>> {
//         let f = self.field_mut(coords);
//         let val = unsafe { f.as_mut() };
//         if let Some(v) = val {
//             v.pos = None;
//             return Some(v);
//         };
//         *f = 0 as *mut AwareElement<T>;
//         None
//     }

//     pub fn create_ref_vec(&self) -> Vec<Vec<Option<AwareElement<T>>>> {
//         (0..HEIGHT)
//             .map(|y| {
//                 (0..WIDTH)
//                     .map(|x| {
//                         let f = self.get_field(Point::new(x, y));
//                         if f.is_none() {
//                             return None;
//                         } else {
//                             return Some((*f.unwrap()).clone());
//                         }
//                     })
//                     .collect::<Vec<_>>()
//             })
//             .collect::<Vec<_>>()
//     }

//     pub fn swap_field(&mut self, start_coord: Point, end_coord: Point) -> UndoSwapField<T> {
//         let start_ptr = self.field(start_coord);
//         let end_ptr = self.field(end_coord);

//         *self.field_mut(end_coord) = start_ptr; //Set target to start element
//         *self.field_mut(start_coord) = 0 as *mut AwareElement<T>; // set start to none

//         let start = unsafe { &mut *start_ptr };
//         start.pos = Some(end_coord);

//         let end = unsafe { end_ptr.as_mut() };

//         if let Some(end) = end {
//             end.pos = None;
//         };

//         return UndoSwapField {
//             start_ptr,
//             end_ptr,
//             start_coord,
//             end_coord,
//         };
//     }
// }

// pub struct UndoSwapField<T: Clone> {
//     start_ptr: *mut AwareElement<T>,
//     end_ptr: *mut AwareElement<T>,
//     start_coord: Point,
//     end_coord: Point,
// }

// impl<T: Clone> UndoSwapField<T> {
//     pub fn undo<
//         const WIDTH: usize,
//         const HEIGHT: usize,
//         const AMOUNT_CONTAINERS: usize,
//         const CONTAINER_SIZE: usize,
//     >(
//         &self,
//         aware_array: &mut AwareArray2D<T, WIDTH, HEIGHT, AMOUNT_CONTAINERS, CONTAINER_SIZE>,
//     ) {
//         unsafe{
//             *aware_array.field_mut(self.start_coord) = self.start_ptr;
//             *aware_array.field_mut(self.end_coord) = self.end_ptr;

//             let start = self.start_ptr.as_mut().unwrap();
//             start.pos = Some(self.start_coord);
            

//             let end = self.end_ptr.as_mut();
//             if let Some(f) = end{
//                 f.pos = Some(self.end_coord)
//             }
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct AwareElement<T: Clone> {
//     pos: Option<Point>,
//     element: T,
// }

// impl<T: Clone> AwareElement<T> {
//     pub fn pos(&self) -> Option<Point> {
//         self.pos
//     }
// }

// impl<T: Clone> std::ops::Deref for AwareElement<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.element
//     }
// }