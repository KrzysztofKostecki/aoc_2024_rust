use regex::Regex;


fn parse_input_1(input: &str) -> Vec<(i32,i32)> {
    let mut valid_muls = Vec::new();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for cap in re.captures_iter(input) {
        println!("{:?}", cap);
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        valid_muls.push((a, b));
    }

    valid_muls
}

fn parse_input_2(input: &str) -> Vec<(i32,i32)> {
    let mut valid_muls = Vec::new();

    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut enable = true;
    for cap in re.captures_iter(input) {
        if let Some(do_match) = cap.get(1) {
            match do_match.as_str() {
                "do()" => enable = true,
                "don't()" => enable = false,
                _ => {
                    if enable {
                        let a = cap[2].parse::<i32>().unwrap();
                        let b = cap[3].parse::<i32>().unwrap();
                        valid_muls.push((a, b));
                    }
                }
            }
        }
    }

    valid_muls
}

pub fn part1(input: &str) -> i32 {
    let valid_muls = parse_input_1(input);

    let mut total = 0;
    for (a, b) in valid_muls {
        total += a * b;
    }

    total
}

pub fn part2(input: &str) -> i32 {
    let valid_muls = parse_input_2(input);

    let mut total = 0;
    for (a, b) in valid_muls {
        total += a * b;
    }

    total
}