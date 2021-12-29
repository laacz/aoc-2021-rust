use std::fmt::{Display, Formatter};
use aoc2021::{get_arg_or_die, read_file_as_lines};

enum CardNumber {
    Number(u8),
    None,
}

impl CardNumber {
    fn is_number(&self) -> bool {
        match *self {
            CardNumber::Number(_) => true,
            CardNumber::None => false,
        }
    }

    fn get_number(&self) -> u8 {
        match *self {
            CardNumber::Number(x) => x,
            _ => 100,
        }
    }
}

impl Display for CardNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            CardNumber::None => write!(f, " x"),
            CardNumber::Number(x) => write!(f, "{: >2}", x)
        }
    }
}

struct Card {
    numbers: Vec<Vec<CardNumber>>,
}

impl Card {
    fn call(&mut self, number: &u8) {
        for row in self.numbers.iter_mut() {
            for card_number in row.iter_mut() {
                if card_number.is_number() && card_number.get_number() == *number {
                    *card_number = CardNumber::None;
                }
            }
        }
    }

    fn is_win(&self) -> bool {
        for row in self.numbers.iter() {
            if row.iter().filter(|n| n.is_number()).count() == 0 {
                return true;
            }
        }

        for col_no in 0..5 {
            let mut has_numbers = false;
            for row in self.numbers.iter() {
                if row[col_no].is_number() {
                    has_numbers = true;
                    break;
                }
            }
            if !has_numbers {
                return true;
            }
        }

        false
    }

    fn score(&self, number: &u8) -> usize {
        let mut ret: usize = 0;
        for row in self.numbers.iter() {
            for card_number in row.iter() {
                if card_number.is_number() {
                    ret += card_number.get_number() as usize;
                }
            }
        }
        ret * *number as usize
    }

    fn new(numbers: Vec<Vec<CardNumber>>) -> Self {
        Self {
            numbers,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "---\n{}", self.numbers
            .iter()
            .map(|row| row.into_iter().map(|n| format!("{: >2}", n)).collect::<Vec<String>>().join(" "))
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

fn main() {
    if let Some(file_name) = get_arg_or_die(1) {
        let input: Vec<String> = read_file_as_lines(&file_name);

        let called: Vec<u8> = input[0].split(",").map(|x| x.parse().unwrap()).collect();

        let mut cards: Vec<Card> = vec![];

        for card in input[2..].chunks(6) {
            let mut numbers: Vec<Vec<CardNumber>> = vec![];
            for line in card.iter() {
                if line.len() > 0 {
                    numbers.push(line.split_whitespace().map(|x| CardNumber::Number(x.parse().unwrap())).collect());
                }
            }
            let card = Card::new(numbers);
            cards.push(card);
        }

        let mut part1 = 0;
        let mut part2 = 0;
        for num in called {
            let cards_in_game = cards.iter().filter(|c| !c.is_win()).count();
            for card in cards.iter_mut() {
                if card.is_win() {
                    continue;
                }
                card.call(&num);
                if card.is_win() {
                    if part1 == 0 {
                        println!("Part 1 result: {}", card.score(&num));
                        part1 = card.score(&num);
                    }
                    if part2 == 0 && cards_in_game == 1 {
                        println!("Part 2 result: {}", card.score(&num));
                        part2 = card.score(&num);
                    }
                }
            }
        }
    }
}
