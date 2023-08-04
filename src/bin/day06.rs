//! --- Day 6: Signals and Noise ---
//! 
//! Something is jamming your communications with Santa. Fortunately, your
//! signal is only partially jammed, and protocol in situations like this is
//! to switch to a simple repetition code to get the message through.
//! 
//! In this model, the same message is sent repeatedly. You've recorded the
//! repeating message signal (your puzzle input), but the data seems quite
//! corrupted - almost too badly to recover. Almost.
//! 
//! All you need to do is figure out which character is most frequent for
//! each position. For example, suppose you had recorded the following messages:
//!
//!     eedadn
//!     drvtee
//!     eandsr
//!     raavrd
//!     atevrs
//!     tsrnev
//!     sdttsa
//!     rasrtv
//!     nssdts
//!     ntnada
//!     svetve
//!     tesnvt
//!     vntsnd
//!     vrdear
//!     dvrsen
//!     enarar
//!
//! The most common character in the first column is e; in the second, a; in
//! the third, s, and so on. Combining these characters returns the
//! error-corrected message, easter.
//! 
//! Given the recording in your puzzle input, what is the error-corrected
//! version of the message being sent?
//! 
//! Answer: gyvwpxaz
//! 
//! --- Part Two ---
//! 
//! Of course, that would be the message - if you hadn't agreed to use a
//! modified repetition code instead.
//! 
//! In this modified code, the sender instead transmits what looks like
//! random data, but for each character, the character they actually want to
//! send is slightly less likely than the others. Even after signal-jamming
//! noise, you can look at the letter distributions in each column and choose
//! the least common letter to reconstruct the original message.
//! 
//! In the above example, the least common character in the first column is
//! a; in the second, d, and so on. Repeating this process for the remaining
//! characters produces the original message, advent.
//! 
//! Given the recording in your puzzle input and this new decoding
//! methodology, what is the original message that Santa is trying to send?
//! 
//! Answer: jucfoary

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_data<P>(name: P) -> Vec<String>
where P: AsRef<Path> {
    let file = File::open(name).unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

fn main() {
    let data = read_data("data/06/input.txt");
    let mut counter: Vec<HashMap<char, u32>> = Vec::new();
    let mut result_1 = String::new();
    let mut result_2 = String::new();
    for _i in 0..data.get(0).unwrap().len() {
        counter.push(HashMap::new()); 
    }
    for l in &data {
        for (idx, ch) in l.chars().enumerate() {
            *counter.get_mut(idx).unwrap().entry(ch).or_insert(0) += 1;
        }
        //break;
    }
    println!("{:?}", &counter);
    for a in &counter {
        result_1.push(*a.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().0);
        result_2.push(*a.iter().min_by(|x, y| x.1.cmp(y.1)).unwrap().0);
    }
    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}
