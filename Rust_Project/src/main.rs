#![allow(unused)]

use std::io;
use std::io::{Write, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false,
            }
        }
    }
    let today:Day = Day::Monday;
    match today {
        Day::Monday => println!("Make Mondays Great again!"),
        Day::Tuesday => println!("Make Tuesdays Great again!"),
        Day::Wednesday => println!("Make Wednesdays Great again!"),
        Day::Thursday => println!("Make Thursdays Great again!"),
        Day::Friday => println!("Make Fridays Great again!"),
        Day::Saturday => println!("Make Saturdays Great again!"),
        Day::Sunday => println!("Make Sundays Great again!"),
    }
    println!("Is today the weekend {}", today.is_weekend());
}
 