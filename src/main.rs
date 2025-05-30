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

    fn print_tabell(&self, show_index: Option<usize>) {
        for (index, card) in self.tabell.iter().enumerate() {
            if index % 3 == 0 {
                println!("\n")
            }
            if self.found.contains(&index) {
                print!("{:?} ", card)
            } else if show_index.is_some() && show_index.unwrap() == index {
                print!("{:?} ", card)
            } else {
                print!("{} ", index)
            }
        }
        println!(" ")
    }

    fn flip_card(&self) -> usize {
        println!("enter a card a card");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        return input
            .trim()
            .parse::<usize>()
            .expect("Please enter a valid number");
    }

    fn steps(&mut self) {
        self.sort_tabell();
        dbg!(&self.tabell);

        self.print_tabell(None);
        let player_input = self.flip_card();
        self.print_tabell(Some(player_input));
    }
}
