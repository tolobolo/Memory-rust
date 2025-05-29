use rand::rng;
use rand::seq::SliceRandom;
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
    fn print_tabell(&self) {
        for (index, num) in self.tabell.iter().enumerate() {
            if index % 3 == 0 {
                println!("\n")
            }
            print!("{} ", index)
        }
        println!()
    }

    fn steps(&mut self) {
        self.sort_tabell();
        dbg!(&self.tabell);

        self.print_tabell();
    }
}
