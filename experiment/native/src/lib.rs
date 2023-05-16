use lazy_static::lazy_static;
use rustter::{dart_fn, dart_struct};

#[dart_fn]
pub fn add_two_number(x: u32, y: u32) -> u32 {
    x + y
}

lazy_static! {
    static ref ARRAY: Vec<u32> = vec![1, 2, 3, 4, 5];
}

#[dart_fn]
pub fn get_array() -> *mut u32 {
    ARRAY.as_ptr() as *mut u32
}

#[dart_fn]
pub fn print_array() {
    for i in ARRAY.iter() {
        println!("{}", i);
    }
}

#[dart_struct]
pub struct Person {
    name: String,
    age: u32,
}
