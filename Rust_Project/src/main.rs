#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "20";
    let mut age: u32 = age.trim().parse()
        .expect("Please enter a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);

}
