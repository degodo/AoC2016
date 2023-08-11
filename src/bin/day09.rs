//! --- Day 9: Explosives in Cyberspace ---
//! 
//! Wandering around a secure area, you come across a datalink port to a new
//! part of the network. After briefly scanning it for interesting files, you
//! find one file in particular that catches your attention. It's compressed
//! with an experimental format, but fortunately, the documentation for the
//! format is nearby.
//! 
//! The format compresses a sequence of characters. Whitespace is ignored. To
//! indicate that some sequence should be repeated, a marker is added to the
//! file, like (10x2). To decompress this marker, take the subsequent 10
//! characters and repeat them 2 times. Then, continue reading the file after
//! the repeated data. The marker itself is not included in the decompressed
//! output.
//! 
//! If parentheses or other characters appear within the data referenced by a
//! marker, that's okay - treat it like normal data, not a marker, and then
//! resume looking for markers after the decompressed section.
//! 
//! For example:
//!     
//!     ADVENT           contains no markers and decompresses to itself with
//!                      no changes, resulting in a decompressed length of 6.
//!     A(1x5)BC         repeats only the B a total of 5 times, becoming
//!                      ABBBBBC for a decompressed length of 7.
//!     (3x3)XYZ         becomes XYZXYZXYZ for a decompressed length of 9.
//!     A(2x2)BCD(2x2)EFG   doubles the BC and EF, becoming ABCBCDEFEFG for a
//!                      decompressed length of 11.
//!     (6x1)(1x3)A      simply becomes (1x3)A - the (1x3) looks like a marker,
//!                      but because it's within a data section of another
//!                      marker, it is not treated any differently from the A
//!                      that comes after it. It has a decompressed length of 6.
//!     X(8x2)(3x3)ABCY  becomes X(3x3)ABC(3x3)ABCY (for a decompressed length
//!                      of 18), because the decompressed data from the (8x2)
//!                      marker (the (3x3)ABC) is skipped and not processed further.
//!     
//! What is the decompressed length of the file (your puzzle input)? Don't
//! count whitespace.
//! 
//! Answer: 110346
//! 
//! --- Part Two ---
//! 
//! Apparently, the file actually uses version two of the format.
//! 
//! In version two, the only difference is that markers within decompressed
//! data are decompressed. This, the documentation explains, provides much
//! more substantial compression capabilities, allowing many-gigabyte files
//! to be stored in only a few kilobytes.
//! 
//! For example:
//!     
//!     (3x3)XYZ still becomes XYZXYZXYZ, as the decompressed section contains no markers.
//!     X(8x2)(3x3)ABCY becomes XABCABCABCABCABCABCY, because the decompressed data from the (8x2) marker is then further decompressed, thus triggering the (3x3) marker twice for a total of six ABC sequences.
//!     (27x12)(20x12)(13x14)(7x10)(1x12)A decompresses into a string of A repeated 241920 times.
//!     (25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN becomes 445 characters long.
//!     
//! Unfortunately, the computer you brought probably doesn't have enough
//! memory to actually decompress the file; you'll have to come up with
//! another way to get its decompressed length.
//! 
//! What is the decompressed length of the file using this improved format?
//! 
//! Answer:10774309173

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_data<P>(name: P) -> String
where P: AsRef<Path> {
    let file = File::open(name).unwrap();
    String::from_iter(BufReader::new(file).lines().map(|x| x.unwrap()))
}

fn unpack(input: &str) -> String {
    let mut result: String = String::new(); 
    let mut ipt = input.chars();
    while let Some(ch) = &ipt.next() {
        if *ch == '(' {
            let mut tmp:String = String::new();
            while let Some(ch) = &ipt.next() {
                if *ch == ')' {
                    break;
                } else {
                    tmp.push(*ch);
                }
            }
            let (c, n) = tmp.split_once('x').map(|(a,b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())).unwrap();
            tmp.clear();
            for _i in 0..c {
                tmp.push(ipt.next().unwrap());
            }
            for _i in 0..n {
                result.push_str(&tmp);
            }
        } else {
            result.push(*ch);
        }
    }
    result
}

fn count_recursive_unpack(input: &str) -> u64 {
    let mut result: u64 = 0;
    let mut ipt = input.chars();
    while let Some(ch) = ipt.next() {
        if ch == '(' {
            let mut tmp:String = String::new();
            while let Some(ch) = ipt.next() {
                if ch == ')' {
                    break;
                } else {
                    tmp.push(ch);
                }
            }
            let (c, n) = tmp.split_once('x').map(|(a,b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap())).unwrap();
            // by_ref erlaugt es, &mut weiter zu reichen, ohne den Owner zu ändern
            // hier werden c Zeichen in count_sub verarbeitet und ipt weitergeschaltet 
            result += n as u64 * count_recursive_unpack(&String::from_iter(&mut ipt.by_ref().take(c as usize)));
        } else {
            result += 1;
        }
    }
    result
}

fn main() {
    let data = read_data("data/09/input.txt");
    let txt = unpack(&data);
    println!("Part 1: {}", txt.len());
    println!("Part 2: {}", count_recursive_unpack(&data));

}
