//! --- Day 10: Balance Bots ---
//! 
//! You come upon a factory in which many robots are zooming around handing
//! small microchips to each other.
//! 
//! Upon closer examination, you notice that each bot only proceeds when it
//! has two microchips, and once it does, it gives each one to a different bot
//! or puts it in a marked "output" bin. Sometimes, bots take microchips from
//! "input" bins, too.
//! 
//! Inspecting one of the microchips, it seems like they each contain a single
//! number; the bots must use some logic to decide what to do with each chip.
//! You access the local control computer and download the bots' instructions
//! (your puzzle input).
//! 
//! Some of the instructions specify that a specific-valued microchip should
//! be given to a specific bot; the rest of the instructions indicate what a
//! given bot should do with its lower-value or higher-value chip.
//! 
//! For example, consider the following instructions:
//!     
//!     value 5 goes to bot 2
//!     bot 2 gives low to bot 1 and high to bot 0
//!     value 3 goes to bot 1
//!     bot 1 gives low to output 1 and high to bot 0
//!     bot 0 gives low to output 2 and high to output 0
//!     value 2 goes to bot 2
//!     
//! Initially, bot 1 starts with a value-3 chip, and bot 2 starts with a
//! value-2 chip and a value-5 chip.
//! Because bot 2 has two microchips, it gives its lower one (2) to bot 1 and
//! its higher one (5) to bot 0.
//! Then, bot 1 has two microchips; it puts the value-2 chip in output 1 and
//! gives the value-3 chip to bot 0.
//! Finally, bot 0 has two microchips; it puts the 3 in output 2 and the 5 in
//! output 0.
//! In the end, output bin 0 contains a value-5 microchip, output bin 1
//! contains a value-2 microchip, and output bin 2 contains a value-3
//! microchip. In this configuration, bot number 2 is responsible for
//! comparing value-5 microchips with value-2 microchips.
//! 
//! Based on your instructions, what is the number of the bot that is
//! responsible for comparing value-61 microchips with value-17 microchips?

//! Answer: 181
//! 
//! --- Part Two ---
//! 
//! What do you get if you multiply together the values of one chip in each
//! of outputs 0, 1, and 2?
//! 
//! Answer: 12567

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;


#[derive(Default, Debug, Clone)]
enum Destination {
    #[default]
    None,
    Bot(u32),
    Bin(u32)
}

#[derive(Default, Debug, Clone)]
struct Control {
    data_a: Option<u32>,
    data_b: Option<u32>,
    dest_high: Destination,
    dest_low: Destination,
}

#[derive(Default, Debug)]
struct Factory {
    bots: HashMap<u32, Control>,
    bins: HashMap<u32, u32>,
}

impl Factory {
    fn new() -> Factory {
        Factory {
            bots: HashMap::new(),
            bins: HashMap::new(),
        }
    }

    fn eval_bot(&mut self, bot: u32) {
        let bot_ctl = self.bots.get(&bot).unwrap().clone();
        if let Some(data_a) = bot_ctl.data_a {
            if let Some(data_b) = bot_ctl.data_b {
                let high_val = data_a.max(data_b); 
                let low_val = data_a.min(data_b);
                match bot_ctl.dest_high {
                    Destination::Bot(id) => { self.update_bot(id, high_val); },
                    Destination::Bin(id) => { self.bins.insert(id, high_val); },
                    Destination::None => (),
                }
                match bot_ctl.dest_low {
                    Destination::Bot(id) => { self.update_bot(id, low_val); },
                    Destination::Bin(id) => { self.bins.insert(id, low_val); },
                    Destination::None => (),
                }
            }
        }
    }

    fn update_bot(&mut self, bot: u32, val: u32) {
        if let Some(bot_ctl) = self.bots.get_mut(&bot) {
            if bot_ctl.data_a == None {
                bot_ctl.data_a = Some(val);
            } else {   
                bot_ctl.data_b = Some(val);
            }
            self.eval_bot(bot);
        } else {
            self.bots.insert(bot, Control { data_a: Some(val), .. Default::default() });
        }
    }

    fn configure_bot(&mut self, bot: u32, dest_high: Destination, dest_low: Destination) {
        if let Some(bot_ctl) = self.bots.get_mut(&bot) {
            bot_ctl.dest_high = dest_high;
            bot_ctl.dest_low = dest_low;
            self.eval_bot(bot);
        } else {
            self.bots.insert(bot, Control { dest_high, dest_low, .. Default::default() });
        }

    }

    fn part_1(&self, val_a: u32, val_b: u32) -> u32 {
        //println!("bots: {:?}", self.bots);
        //println!("bins: {:?}", self.bins);
        for (bot, ctl) in self.bots.iter() {
            if let Some(data_a) = ctl.data_a {
                if let Some(data_b) = ctl.data_b {
                    if (data_a == val_a && data_b == val_b) || (data_a == val_b && data_b == val_a) {
                        return *bot;
                    }
                }
            }
        }
        0
    }

    fn part_2(&self) -> u32 {
        self.bins.get(&0).unwrap() * self.bins.get(&1).unwrap() * self.bins.get(&2).unwrap()
    }

}


fn build_factory(cmds: &Vec<String>) -> Factory{
    let mut factory = Factory::new();
    for cmd in cmds {
        let mut parts = cmd.split_whitespace();
        match parts.next().unwrap() {
            "bot" => {
                let bot: u32 = parts.next().unwrap().parse().unwrap();
                let dest_low;
                let dest_high;
                match parts.nth(3).unwrap() {
                    "bot" => { dest_low = Destination::Bot(parts.next().unwrap().parse().unwrap()) }
                    "output" => { dest_low = Destination::Bin(parts.next().unwrap().parse().unwrap()) }
                    _ => { continue; }
                };
                match parts.nth(3).unwrap() {
                    "bot" => { dest_high = Destination::Bot(parts.next().unwrap().parse().unwrap()) }
                    "output" => { dest_high = Destination::Bin(parts.next().unwrap().parse().unwrap()) }
                    _ => { continue; }
                };
                factory.configure_bot(bot, dest_high, dest_low);
            }
            "value" => {
                let val: u32 = parts.next().unwrap().parse().unwrap();
                let bot: u32 = parts.nth(3).unwrap().parse().unwrap();
                factory.update_bot(bot, val);
            }
            _ => { continue; }
        }
    }
    factory
}

fn read_data<P>(name: P) -> Vec<String>
where P: AsRef<Path> {
    let file = File::open(name).unwrap();
    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

fn main() {
    let data = read_data("data/10/input.txt");
    let factory = build_factory(&data);
    println!("Part 1: {}", factory.part_1(17, 61));
    println!("Part 2: {}", factory.part_2());
}
