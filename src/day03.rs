use regex::Regex;


fn parse_input(input: &str, enable_conditional_logic: bool) -> Vec<(i32,i32)> {
    let mut valid_muls = Vec::new();

    let re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut enable = true;
    for cap in re.captures_iter(input) {
        if let Some(do_match) = cap.get(1) {
            match do_match.as_str() {
                "do()" => enable = true,
                "don't()" => enable = false,
                _ => {
                    if !enable_conditional_logic || enable {
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
    parse_input(input, false).iter().map(|(a, b)| a * b).sum()
}

pub fn part2(input: &str) -> i32 {
    parse_input(input, true).iter().map(|(a, b)| a * b).sum()
}