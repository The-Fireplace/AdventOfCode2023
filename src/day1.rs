use std::fs;

pub fn day1() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let lines = contents.split("\n");
    let mut total = 0;
    for line in lines {
        let digits: Vec<char> = line.chars().filter(|character| character.is_numeric()).collect();
        let number = format!(
            "{}{}",
            digits.get(0).unwrap_or(&'0'),
            digits.last().unwrap_or(&'0')
        );

        total += number.parse::<i32>().unwrap_or(0);
    }

    println!("Total: {}", total);
}