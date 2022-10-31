#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
        
        let st4: &str = "Random String";
        let mut st5: String =st4.to_string();
        println!("{}", st5);
        let byte_arr1 = st5.as_bytes();
        let st6 = &st5[0..6];
        println!("String length : {}", st6.len());
        st5.clear();
    }
}
 