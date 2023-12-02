use std::fs;

pub fn day1() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let lines = contents.split("\n");

    let lines: Vec<&str> = lines.into_iter().collect();
    println!("Coordinate sum: {}", part1(&lines));
    println!("Coordinate sum (words included): {}", part2(&lines));
}

fn part1(lines: &Vec<&str>) -> u32 {
    let mut total: u32 = 0;
    for line in lines {
        let digits: Vec<char> = line.chars().filter(|character| character.is_numeric()).collect();
        let number = format!(
            "{}{}",
            digits.get(0).unwrap_or(&'0'),
            digits.last().unwrap_or(&'0')
        );

        total += number.parse::<u32>().unwrap_or(0);
    }

    total
}

fn part2(lines: &Vec<&str>) -> u32 {
    let allowed_str_digits = [
        "1",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
    ];
    let mut total = 0;
    for line in lines {
        let mut first_index: usize = usize::MAX;
        let mut last_index: usize = 0;
        let mut first_digit = None;
        let mut last_digit = None;
        for (digit_index, digit) in allowed_str_digits.iter().enumerate() {
            if let Some(index) = line.find(digit) {
                if index < first_index {
                    first_index = index;
                    first_digit = Some(digit.parse::<usize>().unwrap_or(digit_index));
                }
            }
            if let Some(index) = line.rfind(digit) {
                if index >= last_index {
                    last_index = index;
                    last_digit = Some(digit.parse::<usize>().unwrap_or(digit_index));
                }
            }
        }
        if let Some(first_digit) = first_digit {
            if let Some(last_digit) = last_digit {
                let formatted_number = format!("{}{}", first_digit, last_digit).parse::<u32>();
                total += formatted_number.expect("We should only have a number at this point!");
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1_sum_is_142() {
        let lines = vec![
            "1abc2",
            "pqr3stu8vwx",
            "a1b2c3d4e5f",
            "treb7uchet",
            "",
        ];
        let actual = part1(&lines);
        assert_eq!(142, actual)
    }

    #[test]
    fn test_example_part2_sum_is_281() {
        let lines = vec![
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "",
        ];
        let actual = part2(&lines);
        assert_eq!(281, actual)
    }
}