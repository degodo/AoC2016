//! --- Day 7: Internet Protocol Version 7 ---
//! 
//! While snooping around the local network of EBHQ, you compile a list of
//! IP addresses (they're IPv7, of course; IPv6 is much too limited). You'd
//! like to figure out which IPs support TLS (transport-layer snooping).
//! 
//! An IP supports TLS if it has an Autonomous Bridge Bypass Annotation, or
//! ABBA. An ABBA is any four-character sequence which consists of a pair of
//! two different characters followed by the reverse of that pair, such as
//! xyyx or abba. However, the IP also must not have an ABBA within any
//! hypernet sequences, which are contained by square brackets.
//! 
//! For example:
//!     
//!     abba[mnop]qrst        supports TLS (abba outside square brackets).
//!     abcd[bddb]xyyx        does not support TLS (bddb is within square brackets,    
//!                           even though xyyx is outside square brackets).
//!     aaaa[qwer]tyui        does not support TLS (aaaa is invalid; the interior   
//!                           characters must be different).
//!     ioxxoj[asdfgh]zxcvbn  supports TLS (oxxo is outside square brackets,    
//!                           even though it's within a larger string).
//!     
//! How many IPs in your puzzle input support TLS?
//! 
//! Answer: 110
//! 
//! --- Part Two ---
//! 
//! You would also like to know which IPs support SSL (super-secret
//! listening).
//! 
//! An IP supports SSL if it has an Area-Broadcast Accessor, or ABA, anywhere
//! in the supernet sequences (outside any square bracketed sections), and a
//! corresponding Byte Allocation Block, or BAB, anywhere in the hypernet
//! sequences. An ABA is any three-character sequence which consists of the
//! same character twice with a different character between them, such as
//! xyx or aba. A corresponding BAB is the same characters but in reversed
//! positions: yxy and bab, respectively.
//! 
//! For example:
//!     
//!     aba[bab]xyz    supports SSL (aba outside square brackets with
//!                    corresponding bab within square brackets).    
//!     xyx[xyx]xyx    does not support SSL (xyx, but no corresponding yxy).    
//!     aaa[kek]eke    supports SSL (eke in supernet with corresponding kek    
//!                    in hypernet; the aaa sequence is not related, because
//!                    the interior character must be different).    
//!     zazbz[bzb]cdb  supports SSL (zaz has no corresponding aza, but zbz    
//!                    has a correspo1nding bzb, even though zaz and zbz overlap).
//!     
//! How many IPs in your puzzle input support SSL?
//! 
//! Answer: 242

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_data<P>(name: P) -> Vec<String>
where P: AsRef<Path> {
    let file = File::open(name).unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

fn check_pattern_abba(txt: &[u8]) -> bool {
    for i in 1..=txt.len()-3 {
        if txt[i-1] == txt[i+2]
           && txt[i] == txt[i+1]
           && txt[i-1] != txt[i] {
            return true;
        }
    }
    false
}

fn find_pattern_aba(txt: &[u8]) -> Vec<[u8;3]> {
    let mut result: Vec<[u8;3]> = Vec::new();
    for i in 0..=txt.len()-3 {
        if txt[i] == txt[i+2] && txt[i] != txt[i+1] {
            let a: [u8;3] = [txt[i],txt[i+1],txt[i+2]];
            result.push(a);
        }
    }
    result
}

fn find_pattern_bab(txt: &[u8]) -> Vec<[u8;3]> {
    let mut result: Vec<[u8;3]> = Vec::new();
    for i in 0..=txt.len()-3 {
        if txt[i] == txt[i+2] && txt[i] != txt[i+1] {
            // flip the letter to make bab look aba for book keeping
            let a: [u8;3] = [txt[i+1],txt[i],txt[i+1]];
            result.push(a);
        }
    }
    result
}

fn check_address_ssl(addr: &str) -> bool{
    let addr_b = addr.as_bytes();
    let mut ipas: HashSet<[u8;3]> = HashSet::new();
    let mut nhss: HashSet<[u8;3]> = HashSet::new();
    let mut lower: usize = 0;
    let mut upper: usize = 0;
    while upper < addr.len() {
        if addr_b[upper] == b'[' {
            ipas.extend(find_pattern_aba(&addr_b[lower..upper]));
            lower = upper + 1;
        } else if addr_b[upper] == b']' {
            nhss.extend(find_pattern_bab(&addr_b[lower..upper]));
            lower = upper + 1;
        } else if addr.len() == upper + 1 {
            ipas.extend(find_pattern_aba(&addr_b[lower..=upper]));
        }
        upper += 1;
    }
    ! ipas.is_disjoint(&nhss)
}

fn check_address_tls(addr: &str) -> bool {
    let addr_b = addr.as_bytes();
    let mut result: bool = false;
    let mut lower: usize = 0;
    let mut upper: usize = 0;
    while upper < addr.len() {
        if addr_b[upper] == b'[' {
            result |= check_pattern_abba(&addr_b[lower..upper]);
            lower = upper + 1;
        } else if addr_b[upper] == b']' {
            if check_pattern_abba(&addr_b[lower..upper]) {
                return  false;
            }
            lower = upper + 1;
        } else if addr.len() == upper + 1 {
            result |= check_pattern_abba(&addr_b[lower..=upper]);
        }
        upper += 1;
    }
    result
}

fn main() {
    let data = read_data("data/07/input.txt");
    let mut result_1: u32 = 0;
    let mut result_2: u32 = 0;
    for i in &data {
        if check_address_tls(i) {
            result_1 += 1;
        }
        if check_address_ssl(i) {
            result_2 += 1;
        }
    }
    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}