use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    part_1()?;
    part_2()?;

    Ok(())
}

fn part_1() -> Result<(), Box<dyn Error>> {
    let bag = Reveal {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut id_sum: u32 = 0;

    for line in read_lines("input.txt") {
        let game = parse_game(&line)?;
        if game.is_possible(&bag) {
            id_sum += u32::from(game.id);
        }
    }

    println!("Sum of possible game IDs: {id_sum}");
    Ok(())
}

fn part_2() -> Result<(), Box<dyn Error>> {
    let mut power_sum: u32 = 0;

    for line in read_lines("input.txt") {
        let game = parse_game(&line)?;

        let mut minimum_bag = Reveal {
            red: 0,
            green: 0,
            blue: 0,
        };
        for reveal in game.reveals {
            if reveal.red > minimum_bag.red {
                minimum_bag.red = reveal.red;
            }

            if reveal.green > minimum_bag.green {
                minimum_bag.green = reveal.green;
            }

            if reveal.blue > minimum_bag.blue {
                minimum_bag.blue = reveal.blue;
            }
        }

        power_sum += minimum_bag.power();
    }

    println!("Sum of minimum game powers: {power_sum}");
    Ok(())
}

fn read_lines<P>(filename: P) -> Box<dyn Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    Box::new(
        io::BufReader::new(file)
            .lines()
            .map(std::result::Result::unwrap),
    )
}

fn parse_game(line: &str) -> Result<Game, Box<dyn Error>> {
    let mut input_parts = line.split(':');
    let input_before_colon = input_parts.next().unwrap();
    let game_id: u8 = input_before_colon.split(' ').nth(1).unwrap().parse()?;
    let input_after_colon = input_parts.next().unwrap();
    let input_reveals = input_after_colon.split(';');
    let mut game = Game {
        id: game_id,
        reveals: vec![],
    };

    for input_reveal in input_reveals {
        let mut reveal = Reveal {
            red: 0,
            green: 0,
            blue: 0,
        };
        let input_revealed_colors = input_reveal.split(',');
        for input_revealed_color in input_revealed_colors {
            let input_revealed_color = input_revealed_color
                .strip_prefix(' ')
                .unwrap_or(input_revealed_color);
            let mut input_revealed_color_parts = input_revealed_color.split(' ');
            let amount: u8 = input_revealed_color_parts.next().unwrap().parse()?;
            let color = input_revealed_color_parts.next().unwrap();
            match color {
                "red" => reveal.red += amount,
                "green" => reveal.green += amount,
                "blue" => reveal.blue += amount,
                c => Err(format!("Unexpected color '{c}'"))?,
            }
        }
        game.reveals.push(reveal);
    }

    Ok(game)
}

#[derive(Debug)]
struct Game {
    id: u8,
    reveals: Vec<Reveal>,
}

impl Game {
    fn is_possible(&self, bag: &Reveal) -> bool {
        for reveal in &self.reveals {
            if reveal.red > bag.red || reveal.green > bag.green || reveal.blue > bag.blue {
                return false;
            }
        }
        true
    }
}

#[derive(Debug)]
struct Reveal {
    red: u8,
    green: u8,
    blue: u8,
}

impl Reveal {
    fn power(&self) -> u32 {
        u32::from(self.red) * u32::from(self.green) * u32::from(self.blue)
    }
}
