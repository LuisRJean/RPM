#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


use std::ops::Add;

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}


fn main() {
    print!("5 + 4 {}", get_sum_gen(5, 4));
    print!("5.2 + 4.6 {}", get_sum_gen(5.2, 4.6));
}
 