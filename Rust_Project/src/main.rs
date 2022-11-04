#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;


fn main() {
    let mut heroes = HashMap::new();
    heroes.insert("SUperman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    println!("{:?}", heroes);
}
 