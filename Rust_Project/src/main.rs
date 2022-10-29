#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you, ";
    io::stdin().read_line(&mut name)
        .expect("Failed to receive input");

    println!("Hello, {}! {}", name.trim_end(), greeting);
}
