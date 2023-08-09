//! --- Day 8: Two-Factor Authentication ---
//! 
//! You come across a door implementing what you can only assume is an
//! implementation of two-factor authentication after a long game of
//! requirements telephone.
//! 
//! To get past the door, you first swipe a keycard (no problem; there was
//! one on a nearby desk). Then, it displays a code on a little screen, and
//! you type that code on a keypad. Then, presumably, the door unlocks.
//! 
//! Unfortunately, the screen has been smashed. After a few minutes, you've
//! taken everything apart and figured out how it works. Now you just have
//! to work out what the screen would have displayed.
//! 
//! The magnetic strip on the card you swiped encodes a series of
//! instructions for the screen; these instructions are your puzzle input.
//! The screen is 50 pixels wide and 6 pixels tall, all of which start off,
//! and is capable of three somewhat peculiar operations:
//! 
//!     rect AxB                turns on all of the pixels in a rectangle at    
//!                             the top-left of the screen which is A wide    
//!                             and B tall.
//!     rotate row y=A by B     shifts all of the pixels in row A (0 is the    
//!                             top row) right by B pixels. Pixels that would    
//!                             fall off the right end appear at the left end    
//!                             of the row.
//!     rotate column x=A by B  shifts all of the pixels in column A (0 is    
//!                             the left column) down by B pixels. Pixels that    
//!                             would fall off the bottom appear at the top of    
//!                             the column.
//! 
//! For example, here is a simple sequence on a smaller screen:
//! 
//! rect 3x2 creates a small rectangle in the top-left corner:
//!     
//!     ###....
//!     ###....
//!     .......
//!     
//! rotate column x=1 by 1 rotates the second column down by one pixel:
//!     
//!     #.#....
//!     ###....
//!     .#.....
//!     
//! rotate row y=0 by 4 rotates the top row right by four pixels:
//!     
//!     ....#.#
//!     ###....
//!     .#.....
//!     
//! rotate column x=1 by 1 again rotates the second column down by one pixel,
//! causing the bottom pixel to wrap back to the top:
//!     
//!     .#..#.#
//!     #.#....
//!     .#.....
//!     
//! As you can see, this display technology is extremely powerful, and will
//! soon dominate the tiny-code-displaying-screen market. That's what the
//! advertisement on the back of the display tries to convince you, anyway.
//! 
//! There seems to be an intermediate check of the voltage used by the
//! display: after you swipe your card, if the screen did work, how many
//! pixels should be lit?
//! 
//! Answer: 106
//! 
//! --- Part Two ---
//! 
//! You notice that the screen is only capable of displaying capital letters;
//! in the font it uses, each letter is 5 pixels wide and 6 tall.
//! 
//! After you swipe your card, what code is the screen trying to display?
//! 
//! Answer: CFLELOYFCS

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_data<P>(name: P) -> Vec<String>
where P: AsRef<Path> {
    let file = File::open(name).unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

#[derive(Debug)]
enum CMD {
    RECT(u8, u8),
    ROTROW(u8, u8),
    ROTCOL(u8, u8),
}

type GRID = [[bool; 50]; 6];


fn decode_command(cmd: &str) -> CMD {
    let mut elm = cmd.split_whitespace();
    match elm.next() {
        Some("rect") => {
            let a = elm.next().unwrap().split_once('x')
                                .map(|(r,c)| (r.parse::<u8>().unwrap(),c.parse::<u8>().unwrap()))
                                .unwrap();
            CMD::RECT(a.0, a.1)
        },
        Some("rotate") => {
            let c = elm.next().unwrap();
            let a = elm.next().unwrap().split_once('=')
                            .map(|(r,c)| c.parse::<u8>().unwrap()).unwrap();
            let b = elm.last().unwrap().parse::<u8>().unwrap();
            match c {
                "row" => CMD::ROTROW(a, b),
                "column" => CMD::ROTCOL(a, b),
                _ => CMD::RECT(0, 0)
            }
        },
        Some(&_) => CMD::RECT(0, 0),
        None => CMD::RECT(0, 0)
    }
}

fn draw_display(data: &Vec<String>) ->  GRID {
    let mut grid: GRID = [[false; 50]; 6];
    for raw_cmd in data.iter() {
        println!("{:?}", decode_command(raw_cmd));        
        match decode_command(raw_cmd) {
            CMD::RECT(c,r) => { for a in 0..r as usize { for b in 0..c as usize { grid[a][b] = true; }}},
            CMD::ROTROW(c, w) => {  
                let g = grid[c as usize];
                for i in 0..50 as usize {
                    grid[c as usize][(i+w as usize) % 50] = g[i];
                }
            },
            CMD::ROTCOL(c, w ) => {
                let mut tmp: [bool; 6] = [false; 6];
                for i in 0..6 as usize {
                    tmp[i] = grid[i][c as usize];
                }
                for i in 0..6 as usize {
                    grid[(i+w as usize) % 6][c as usize] = tmp[i];
                }
            },
        }
    }
    grid
}

fn read_screen(grid: &GRID) -> String {
    let mut result: String = String::new();
    let mut col: usize = 0;
    while col < 50 {
        if grid[0][col] == false {
            if grid[1][col] == false {
                if grid[4][col] == false {
                    result.push('J');
                } else {
                    result.push('I');
                }
                col += 5;
            } else {
                if grid[5][col] == false {
                    col += 3;
                    if grid[5][col] == false {
                        if grid[3][col] == false {
                            result.push('C');
                        } else {
                            if grid[2][col] == false {
                                result.push('G');
                            } else {
                                result.push('O');
                            }
                        }
                    } else {
                        result.push('Q');
                    }
                    col += 2;
                } else {
                    if grid[4][col] == false {
                        result.push('S');
                    } else {
                        result.push('A');
                    }
                    col += 5;
                }
            }
        } else {
            if grid[1][col] == false { 
                if grid[5][col] == false {
                    result.push('T');
                } else {
                    result.push('Z');
                }
                col += 5;
            } else {
                if grid[2][col] == false {
                    if grid[5][col] == false {
                        result.push('Y');
                    } else {
                        result.push('X');
                    }
                    col += 5;
                } else {
                    if grid[4][col] == false {
                        result.push('V');
                        col += 5;
                    } else {
                        if grid[5][col] == false {
                            result.push('U');
                            col += 5;
                        } else {
                            col += 1;
                            if grid[0][col] == false {
                                if grid[5][col] == true {
                                    result.push('L');
                                } else if grid[4][col] == true {
                                    result.push('W');
                                } else if grid[3][col] == true {
                                    result.push('K');
                                } else if grid[2][col] == true {
                                    result.push('H');
                                } else {
                                    if grid[5][col+2] == false {
                                        result.push('M');
                                    } else {
                                        result.push('N');
                                    }
                                }
                                col += 4;
                            } else {
                                if grid[2][col] == false {
                                    if grid[5][col] == false {
                                        col += 2;
                                        if grid[5][col] == false {
                                            result.push('P');
                                        } else {
                                            result.push('R');
                                        }
                                        col += 2;
                                    } else {
                                        result.push('D');
                                        col += 4;
                                    } 
                                } else {
                                    if grid[5][col] == false {
                                        result.push('F');
                                        col += 4;
                                    } else {
                                        col += 2;
                                        if grid[5][col] == false {
                                            result.push('B');
                                        } else {
                                            result.push('E');
                                        }
                                        col += 2;
                                    } 
                                }
                            }

                        }
                    }

                }

            } 
        }
    }
    println!("{:?}", grid);
    result
}

fn count_pixels(grid: &GRID) -> u32 {
    let mut result:u32 = 0;
    for r in 0..6 {
        for c in 0..50 {
            if grid[r][c] {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let data = read_data("data/08/input.txt");
    let grid = draw_display(&data);
    println!("Part 1: {}", count_pixels(&grid));
    println!("Part 2: {}", read_screen(&grid));
}