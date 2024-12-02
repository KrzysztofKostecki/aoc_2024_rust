
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let numbers: Vec<i32> = line.split_whitespace()
                                    .map(|num| num.parse::<i32>().unwrap())
                                    .collect();
        reports.push(numbers);
    }

    reports
}

fn check_ordering(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }
    
    let descending = report[0] > report[1];
    report.windows(2).all(|pair| {
        let (a, b) = (pair[0], pair[1]);
        if descending {
            a > b && (a - b) <= 3
        } else {
            a < b && (b - a) <= 3
        }
    })
}

fn check_ordering_with_allowed_error(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    if check_ordering(report) {
        return true;
    }

    report.iter().enumerate().any(|(i, _)| {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        check_ordering(&modified_report)
    })
}

pub fn part1(input: &str) -> i32 {
    parse_input(input).iter().filter(|report| check_ordering(report)).count() as i32
}

pub fn part2(input: &str) -> i32 {
    parse_input(input).iter().filter(|report| check_ordering_with_allowed_error(report)).count() as i32
}


