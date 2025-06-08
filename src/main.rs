use rand::rng;
use rand::seq::SliceRandom;
use std::thread;
use std::time::Duration;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    let mut game = Memory::new();
    game.steps()
}

fn clear_console(second: u64) {
    thread::sleep(Duration::new(second, 0));
    clearscreen::clear().expect("Failed to clear screen");
}

#[derive(Debug, EnumIter, Clone, PartialEq)]
enum Card {
    Box,
    Boat,
    Dog,
    Bed,
    Kiwi,
    Pogo,
    TRex,
    Tesla,
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
        println!("________________________");
        for (index, card) in self.tabell.iter().enumerate() {
            if index % 4 == 0 {
                println!("\n")
            }
            if show_index1.is_some() && show_index1.unwrap() == index
                || show_index2.is_some() && show_index2.unwrap() == index
                || self.found.contains(&index)
            {
                print!("{:?} |", card)
            } else {
                print!("{} |", index)
            }
        }
        println!("\n");
        println!("________________________")
    }

    fn ask_for_number(&self, index1: Option<usize>) -> usize {
        println!("enter a card ");
        let mut input = String::new();

        loop {
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(num)
                    if num < self.tabell.len() && (index1.is_none() || num != index1.unwrap()) =>
                {
                    return num;
                }
                _ => {
                    println!("Invalid input. Please enter a valid number.");
                    input.clear();
                }
            }
        }
    }

    fn round(&mut self) {
        self.print_tabell(None, None);
        let index1 = self.ask_for_number(None);
        clear_console(0);
        self.print_tabell(Some(index1), None);
        let index2 = self.ask_for_number(Some(index1));
        clear_console(0);
        self.print_tabell(Some(index1), Some(index2));

        if self.tabell[index1] == self.tabell[index2] {
            println!("Found a pair!");
            self.found.push(index1);
            self.found.push(index2);
        } else {
            println!("Not a pair.");
        }
    }

    fn steps(&mut self) {
        self.sort_tabell();
        clear_console(0);
        while self.found.len() < self.tabell.len() {
            self.round();
            clear_console(2);
        }
        println!("Congratulations! You found all pairs.");
    }
}
