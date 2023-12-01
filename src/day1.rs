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

    let letters = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let options = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];

    let all_digits: Vec<usize> = content
        .trim_end()
        .split("\n")
        .map(|line| {
            let mut digits: Vec<usize> = Vec::new();

            for i in 0..line.len() {
                if let Some(parts) = line.get(i..line.len()) {
                    for (j, letter) in options.iter().enumerate() {
                        if parts.starts_with(letter) {
                            if j >= 10 {
                                digits.push(j - 10);
                            } else {
                                digits.push(j)
                            }

                            println!("{letter} {parts}");
                            break;
                        }
                    }
                }
            }

            let (first, last) = (digits.first().unwrap_or(&0), digits.last().unwrap_or(&0));
            return (first * 10) + last;
        })
        .collect();

    let total: usize = all_digits.iter().sum();

    println!("total {}", total);

    Ok(())
}
