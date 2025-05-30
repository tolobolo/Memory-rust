use std::usize;

use rand::rng;
use rand::seq::SliceRandom;
use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let mut game = Memory::new();
    game.steps()
}

#[derive(Debug, EnumIter, Clone)]
enum Card {
    Box,
    Apple,
    Dog,
    Orange,
    Kiwi,
    Pogo,
    TRex,
}

struct Memory {
    tabell: Vec<Card>,
    found: Vec<usize>,
}

impl Memory {
    fn new() -> Self {
        let mut tabell: Vec<Card> = Vec::new();
        for card in Card::iter() {
            tabell.push(card.clone());
            tabell.push(card);
        }

        Self {
            tabell,
            found: Vec::new(),
        }
    }

    fn sort_tabell(&mut self) {
        let mut rng = rng();
        self.tabell.shuffle(&mut rng);
    }

    fn print_tabell(&self, show_index1: Option<usize>, show_index2: Option<usize>) {
        for (index, card) in self.tabell.iter().enumerate() {
            if index % 3 == 0 {
                println!("\n")
            }
            if self.found.contains(&index) {
                print!("{:?} ", card)
            } else if show_index1.is_some() && show_index1.unwrap() == index {
                print!("{:?} ", card)
            } else if show_index2.is_some() && show_index2.unwrap() == index {
                print!("{:?} ", card)
            } else {
                print!("{} ", index)
            }
        }

        println!(" ")
    }

    fn ask_for_number(&self) -> usize {
        println!("enter a card ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input
            .trim()
            .parse::<usize>()
            .expect("Please enter a valid number")
    }

    fn round(&mut self) {
        self.print_tabell(None, None);
        let index1 = self.ask_for_number();
        self.print_tabell(Some(index1), None);
        let index2 = self.ask_for_number();
        self.print_tabell(Some(index1), Some(index2));

        if index1 == index2 {
            self.found.push(index1);
            self.found.push(index2);
        } else {
            println!("No a pair.");
        }
    }

    fn steps(&mut self) {
        self.sort_tabell();
        dbg!(&self.tabell);

        while self.found.len() > self.tabell.len() {
            self.round();
        }
    }
}
