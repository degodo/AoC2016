//! --- Day 3: Squares With Three Sides ---
//! 
//! Now that you can think clearly, you move deeper into the labyrinth of
//! hallways and office furniture that makes up this part of Easter Bunny HQ.
//! This must be a graphic design department; the walls are covered in
//! specifications for triangles.
//! 
//! Or are they?
//! 
//! The design document gives the side lengths of each triangle it describes,
//!  but... 5 10 25? Some of these aren't triangles. You can't help but mark
//! the impossible ones.
//! 
//! In a valid triangle, the sum of any two sides must be larger than the
//! remaining side. For example, the "triangle" given above is impossible,
//! because 5 + 10 is not larger than 25.
//! 
//! In your puzzle input, how many of the listed triangles are possible?
//! 
//! Answer: 983
//! 
//! --- Part Two ---
//! 
//! Now that you've helpfully marked up their design documents, it occurs to 
//! you that triangles are specified in groups of three vertically. Each set
//! of three numbers in a column specifies a triangle. Rows are unrelated.
//! 
//! For example, given the following specification, numbers with the same
//! hundreds digit would be part of the same triangle:
//!     
//!     101 301 501
//!     102 302 502
//!     103 303 503
//!     201 401 601
//!     202 402 602
//!     203 403 603
//!     
//! In your puzzle input, and instead reading by columns, how many of the
//! listed triangles are possible?
//! 
//! Amswer: 1836



use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use itertools::Itertools;


fn read_data<P>(name: P) -> Vec<(u32, u32, u32)>
where P: AsRef<Path> {
    let file = File::open(name).unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap()).
        map(|x| x.trim().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect_tuple::<(u32,u32,u32)>().unwrap()).
        collect()
}

fn main() {
    let data = read_data("data/03/input.txt");
    let mut count: u32 = 0;
    for a in &data {
       if a.0 + a.1 > a.2 && a.0 + a.2 > a.1 && a.1 + a.2 > a.0 {
            count += 1;
       }
    }
    println!("Part 1: {}", count);
    
    count = 0;
    let mut i = data.iter();
    while let Some(a) = i.next() {
        let b = i.next().unwrap();
        let c = i.next().unwrap();
        if a.0 + b.0 > c.0 && a.0 + c.0 > b.0 && b.0 + c.0 > a.0 {
            count += 1;
       }
        if a.1 + b.1 > c.1 && a.1 + c.1 > b.1 && b.1 + c.1 > a.1 {
            count += 1;
       }
        if a.2 + b.2 > c.2 && a.2 + c.2 > b.2 && b.2 + c.2 > a.2 {
            count += 1;
       }
    }
    println!("Part 2: {}", count);

}