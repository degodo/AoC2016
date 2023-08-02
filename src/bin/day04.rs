//! --- Day 4: Security Through Obscurity ---
//! 
//! Finally, you come across an information kiosk with a list of rooms. Of
//! course, the list is encrypted and full of decoy data, but the instructions
//! to decode the list are barely hidden nearby. Better remove the decoy data
//! first.
//! 
//! Each room consists of an encrypted name (lowercase letters separated by
//! dashes) followed by a dash, a sector ID, and a checksum in square
//! brackets.
//! 
//! A room is real (not a decoy) if the checksum is the five most common
//! letters in the encrypted name, in order, with ties broken by
//! alphabetization. For example:
//! 
//!      aaaaa-bbb-z-y-x-123[abxyz]    
//! is a real room because the most common letters are a (5), b (3), and then
//! a tie between x, y, and z, which are listed alphabetically.
//! 
//!      a-b-c-d-e-f-g-h-987[abcde]   
//!  is a real room because although the letters are all tied (1 of each),
//! the first five are listed alphabetically.
//! 
//!      not-a-real-room-404[oarel]    
//! is a real room.
//! 
//!      totally-real-room-200[decoy]    
//! is not.
//! 
//! Of the real rooms from the list above, the sum of their sector IDs
//! is 1514.
//! 
//! What is the sum of the sector IDs of the real rooms?
//! 
//! Answer: 409147
//! 
//! --- Part Two ---
//! 
//! With all the decoy data out of the way, it's time to decrypt this list
//! and get moving.
//! 
//! The room names are encrypted by a state-of-the-art shift cipher, which is
//! nearly unbreakable without the right software. However, the information
//! kiosk designers at Easter Bunny HQ were not expecting to deal with a
//! master cryptographer like yourself.
//! 
//! To decrypt a room name, rotate each letter forward through the alphabet a
//! number of times equal to the room's sector ID. A becomes B, B becomes C,
//! Z becomes A, and so on. Dashes become spaces.
//! 
//! For example, the real name for qzmt-zixmtkozy-ivhz-343 is very encrypted
//! name.
//! 
//! What is the sector ID of the room where North Pole objects are stored?
//! 
//! Answer: 991



use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use itertools::Itertools;

fn read_data<P>(name: P) -> Vec<String>
where P: AsRef<Path> {
    let file = File::open(name).unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

fn verify_checksum(code: &str, checksum: &str) -> bool {
    let mut count: HashMap<char, u32> = HashMap::new();
    let clean_code: String = String::from_iter(code.split('-'));
    for c in clean_code.chars() {
        *count.entry(c).or_insert(0) += 1;
    } 
    //let char_frequency: Vec<(char, u32)> = count.iter()
    //    .sorted_by(|a, b| Ord::cmp(&b.1, &a.1).then(Ord::cmp(&a.0, &b.0)))
    //    .map(|(a,b )| (*a, *b)).collect();
    let mut frequency_str = String::from_iter(count.iter()
        .sorted_by(|a, b| Ord::cmp(&b.1, &a.1).then(Ord::cmp(&a.0, &b.0)))
        .map(|(a, _b )| *a));
    frequency_str.truncate(checksum.len());
    let y: &str = frequency_str.as_ref();
    y.cmp(checksum).is_eq()
}

fn split_code(s: &str) -> (String, u32, String) {
    let Some((raw_code,s_rest)) = s.rsplit_once('-') else { todo!() };
    let Some((sid, cs)) = s_rest.split_once('[') else { todo!() };
    let selector_id: u32 = sid.parse().unwrap();
    let checksum: String = String::from(&cs[..cs.len()-1]);
    (String::from(raw_code), selector_id, checksum)
}

fn next_char(ch: char) -> char {
    match ch {
        'a' => 'b',
        'b' => 'c',
        'c' => 'd',
        'd' => 'e',
        'e' => 'f',
        'f' => 'g',
        'g' => 'h',
        'h' => 'i',
        'i' => 'j',
        'j' => 'k',
        'k' => 'l',
        'l' => 'm',
        'm' => 'n',
        'n' => 'o',
        'o' => 'p',
        'p' => 'q',
        'q' => 'r',
        'r' => 's',
        's' => 't',
        't' => 'u',
        'u' => 'v',
        'v' => 'w',
        'w' => 'x',
        'x' => 'y',
        'y' => 'z',
        'z' => 'a',
        _ => ' ',
    }
}

fn rotate_char(mut ch: char, n: u32) -> char {
    if ch == '-' {
        ' '
    } else {
        for _i in 0..n % 26 {
            ch = next_char(ch);
        }
        ch
    }
}

fn main () {
    let data = read_data("data/04/input.txt");
    let mut selector_sum: u32 = 0;
    let mut storage_selectorid: u32 = 0;
    for i in &data {
        let (code, selectorid, checksum) = split_code(i);
        if verify_checksum(&code, &checksum) {
            selector_sum += selectorid;
            let decoded = String::from_iter(code.chars().map(|x| rotate_char(x, selectorid)));
            if decoded.starts_with("northpole") {
                storage_selectorid = selectorid;
            };
        }
    }
    println!("Part 1: {}", selector_sum);
    println!("Part 2: {}", storage_selectorid);
}

