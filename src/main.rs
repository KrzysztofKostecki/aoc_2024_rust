use std::env;

mod utils;

mod day01;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];  // Get day number from command line

    match day.as_str() {
        "1" => {
            let input = std::fs::read_to_string("inputs/day01.txt").unwrap();
            println!("Day 1 Part 1: {}", day01::part1(&input));
            println!("Day 1 Part 2: {}", day01::part2(&input));
        }
        "2" => print!("Day 2: Not implemented yet"),
        "3" => println!("Day 3: Not implemented yet"),
        "4" => println!("Day 4: Not implemented yet"),
        "5" => println!("Day 5: Not implemented yet"),
        "6" => println!("Day 6: Not implemented yet"),
        "7" => println!("Day 7: Not implemented yet"),
        "8" => println!("Day 8: Not implemented yet"),
        "9" => println!("Day 9: Not implemented yet"),
        "10" => println!("Day 10: Not implemented yet"),
        "11" => println!("Day 11: Not implemented yet"),
        "12" => println!("Day 12: Not implemented yet"),
        "13" => println!("Day 13: Not implemented yet"),
        "14" => println!("Day 14: Not implemented yet"),
        "15" => println!("Day 15: Not implemented yet"),
        "16" => println!("Day 16: Not implemented yet"),
        "17" => println!("Day 17: Not implemented yet"),
        "18" => println!("Day 18: Not implemented yet"),
        "19" => println!("Day 19: Not implemented yet"),
        "20" => println!("Day 20: Not implemented yet"),
        "21" => println!("Day 21: Not implemented yet"),
        "22" => println!("Day 22: Not implemented yet"),
        "23" => println!("Day 23: Not implemented yet"),
        "24" => println!("Day 24: Not implemented yet"),
        "25" => println!("Day 25: Not implemented yet"),
        _ => println!("Please specify a valid day")
    }
}