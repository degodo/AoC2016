//! --- Day 1: No Time for a Taxicab ---
//! 
//! Santa's sleigh uses a very high-precision clock to guide its movements,
//! and the clock's oscillator is regulated by stars. Unfortunately, the
//! stars have been stolen... by the Easter Bunny. To save Christmas, Santa
//! needs you to retrieve all fifty stars by December 25th.
//! 
//! Collect stars by solving puzzles. Two puzzles will be made available on
//! each day in the Advent calendar; the second puzzle is unlocked when you
//! complete the first. Each puzzle grants one star. Good luck!
//! 
//! You're airdropped near Easter Bunny Headquarters in a city somewhere.
//! "Near", unfortunately, is as close as you can get - the instructions on
//! the Easter Bunny Recruiting Document the Elves intercepted start here,
//! and nobody had time to work them out further.
//! 
//! The Document indicates that you should start at the given coordinates
//! (where you just landed) and face North. Then, follow the provided
//! sequence: either turn left (L) or right (R) 90 degrees, then walk forward
//! the given number of blocks, ending at a new intersection.
//! 
//! There's no time to follow such ridiculous instructions on foot, though,
//! so you take a moment and work out the destination. Given that you can
//! only walk on the street grid of the city, how far is the shortest path to
//! the destination?
//! 
//! For example:
//! 
//! Following R2, L3 leaves you 2 blocks East and 3 blocks North, or 5 blocks
//! away   
//! R2, R2, R2 leaves you 2 blocks due South of your starting position, which
//! is 2 blocks away.   
//! R5, L5, R5, R3 leaves you 12 blocks away.
//! 
//! How many blocks away is Easter Bunny HQ?
//! 
//! Answer: 291
//! 
//! --- Part Two ---
//! 
//! Then, you notice the instructions continue on the back of the Recruiting
//! Document. Easter Bunny HQ is actually at the first location you visit
//! twice.
//! 
//! For example, if your instructions are R8, R4, R4, R8, the first location 
//! you visit twice is 4 blocks away, due East.
//! 
//! How many blocks away is the first location you visit twice?
//! 
//! Answer: 159



use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_data<P>(name: P) -> Vec<(String,u32)>

where P: AsRef<Path> {
    let file = File::open(name).unwrap();
    let s = BufReader::new(file).lines().map(|x| x.unwrap()).next().unwrap();
    s.split(',').map(|x|x.trim().split_at(1)).map(|(o,d)| (o.to_owned(), d.parse::<u32>().unwrap())).collect()
}

fn movement(data: &Vec<(String, u32)>) {
    let mut places: HashSet<(i32, i32)> = HashSet::new();
    let mut curpos: (i32, i32) = (0,0);
    let mut nextpos = curpos;
    let mut xpos = curpos;
    let mut direction: i32 = 0; // 0 = North, 1 = East, 2 = South, 3 = West
    places.insert(curpos);
    for (r, d) in data {
        if r.eq("R") {
            direction = (direction + 1) % 4;
        } else { 
            direction = direction - 1;
            if direction < 0 {
                direction = 3;
            }
        }
        match direction {
            0 => { nextpos.1 += *d as i32; }
            1 => { nextpos.0 += *d as i32; }
            2 => { nextpos.1 -= *d as i32; }
            3 => { nextpos.0 -= *d as i32; }
            _ => { println!("Error in direction"); }
        }
        {
            let a: i32;
            let b: i32;
            if curpos.0 == nextpos.0 {
                if curpos.1 > nextpos.1 {
                    a = nextpos.1;
                    b = curpos.1;
                } else {
                    a = curpos.1 + 1;  // cur_pos ist erfasst
                    b = nextpos.1 + 1; // wg. Schleifenobergrenze
                }
                for y in a..b {
                    if ! places.insert((curpos.0, y)) && xpos == (0, 0) {
                        xpos = (curpos.0, y);
                    }
                }
            } else if curpos.1 == curpos.1 {
                if curpos.0 > nextpos.0 {
                    a = nextpos.0;
                    b = curpos.0;
                } else {
                    a = curpos.0 + 1;  // cur_pos ist erfasst
                    b = nextpos.0 + 1; // wg. Schleifenobergrenze
                }
                for x in a..b {
                    if ! places.insert((x, curpos.1)) && xpos == (0, 0) {
                        xpos = (x, curpos.1);
                    }
                }
            } else {
                println!("Logikfehler! {:?} {:?}", curpos, nextpos);
            }
        }
        curpos = nextpos;
    }
    println!("Part 1: {}", curpos.0.abs() + curpos.1.abs());
    println!("Part 2: {}", xpos.0.abs() + xpos.1.abs());
}

fn main() {
    let data = read_data("data/01/input.txt");
    movement(&data);
}
