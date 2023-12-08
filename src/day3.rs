use std::io::Result;

mod utils;

fn is_digit(arr: &str, index: usize) -> bool {
    if arr.len() == index {
        return false;
    }

    if let Some(value) = arr.chars().nth(index) {
        return value.is_digit(10);
    }

    println!("{}, {}", arr, index);

    return false;
}

fn is_symble(c: char) -> bool {
    match c {
        '/' | '!' | '@' | '#' | '$' | '%' | '&' | '*' | '-' | '+' | '=' => true,
        _ => false,
    }
}

fn is_symble2(table: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    if x < 0 || x >= table.len() as i32 {
        return false;
    }

    if y < 0 || y >= table[x as usize].len() as i32 {
        return false;
    }

    let c = table[x as usize][y as usize];

    match c {
        '/' | '!' | '@' | '#' | '$' | '%' | '&' | '*' | '-' | '+' | '=' => true,
        _ => false,
    }
}

fn main() -> Result<()> {
    let content = utils::read_file("3")?;

    // content.split("\n").for_each(|item| println!("{}", item));

    let table: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut total: usize = 0;

    for (x, line) in content.lines().enumerate() {
        let end = line.len() - 1;

        let mut start = 0;
        let mut y = 0;

        while y < end {
            start = y;
            let mut num = String::from("");
            while is_digit(line, y) {
                num.push(line.chars().nth(y).unwrap());
                y += 1;
            }

            if num == "" {
                y += 1;
                continue;
            }

            if start != 0 && is_symble(table[x][start - 1]) {
                y += 1;
                total += num.parse::<usize>().unwrap();
                println!("{}", num);
                continue;
            }

            if is_symble2(&table, x as i32, y as i32) {
                total += num.parse::<usize>().unwrap();
                println!("{}", num);
                y += 1;
                continue;
            }

            for a in (start as i32 - 1)..(y as i32 + 1) {
                if is_symble2(&table, x as i32 - 1, a as i32)
                    || is_symble2(&table, x as i32 + 1, a as i32)
                {
                    total += num.parse::<usize>().unwrap();
                    break;
                }
            }
        }
    }

    println!("{}", total);

    Ok(())
}
