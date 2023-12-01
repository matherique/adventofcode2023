use std::io::Result;

mod utils;

/*
1abc2           12
pqr3stu8vwx     38
a1b2c3d4e5f     15
treb7uchet      77
*/

fn main() -> Result<()> {
    let content = utils::read_file("1")?;

    // let letters = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let all_digits: Vec<usize> = content
        .trim_end()
        .split("\n")
        .map(|line| {
            let numbers: String = line.chars().filter(|c| c.is_numeric()).collect();

            let mut digits = String::from("");

            if let Some(first) = numbers.chars().next() {
                digits.push(first);
            }

            if let Some(last) = numbers.chars().last() {
                digits.push(last);
            }

            digits.parse::<usize>().unwrap_or_default()
        })
        .collect();

    let total: usize = all_digits.iter().sum();

    println!("total {}", total);

    Ok(())
}
