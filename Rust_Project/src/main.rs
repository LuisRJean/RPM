#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn print_str(x: String){
    println!("A string: {}", x);
}


fn print_return_str(x: String) -> String{
    println!("A string {}", x);
    x
}

fn change_string(name: & mut String){
    name.push_str("is happy");
    println!("Message: {}", name);
}

fn main() {
    let str1 = String::from("Luis");
    change_string(&mut str1);
}
 