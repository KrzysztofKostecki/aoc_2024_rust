use std::{collections::HashMap, io};

fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), io::Error> {
    let mut distances: (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() != 2 {
            eprintln!("Invalid line format: {}", line);
            continue;
        }
        match (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
            (Ok(num1), Ok(num2)) => {
                distances.0.push(num1);
                distances.1.push(num2);
            }
            _ => eprintln!("Failed to parse numbers in line: {}", line),
        }
    }

    Ok(distances)
}


pub fn part1(input: &str) -> i32 {
    let mut distances = parse_input(&input).unwrap();

    distances.0.sort_unstable();
    distances.1.sort_unstable();

    if distances.0.len() != distances.1.len() {
        println!("NO");
        return 0;
    }

    let total_distance: i32 = distances.0.iter()
        .zip(distances.1.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    total_distance
}

pub fn part2(input: &str) -> i32 {
    let distances = parse_input(&input).unwrap();

    let mut map: HashMap<i32, i32> = HashMap::new();
    for num in distances.1 {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut similarity_score: i32 = 0;
    for num in distances.0 {
        similarity_score += num * map.get(&num).unwrap_or(&0)
    }

    similarity_score
}