use std::{collections::HashMap, io::Result};

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

#[derive(Debug)]
struct Number {
    symble_pos: String,
    number: usize,
}

fn main() -> Result<()> {
    let content = utils::read_file("3")?;

    // content.split("\n").for_each(|item| println!("{}", item));

    let table: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    // let mut response: Vec<Number> = Vec::new();
    let mut pairs: Vec<String> = Vec::new();
    let mut map: HashMap<String, usize> = HashMap::new();

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

            let numparse = num.parse::<usize>().unwrap();
            if start != 0 && is_symble(table[x][start - 1]) {
                y += 1;
                let pos = format!("({},{})", x, start - 1);
                if let Some(_) = map.get(&pos) {
                    pairs.push(pos.clone());
                }

                *map.entry(pos.clone()).or_insert(1) *= numparse;
                continue;
            }

            if is_symble2(&table, x as i32, y as i32) {
                y += 1;
                let pos = format!("({},{})", x, y);
                if let Some(_) = map.get(&pos) {
                    pairs.push(pos.clone());
                }
                *map.entry(pos.clone()).or_insert(1) *= numparse;
                continue;
            }

            for a in (start as i32 - 1)..(y as i32 + 1) {
                if is_symble2(&table, x as i32 - 1, a as i32) {
                    let pos = format!("({},{})", x - 1, a);
                    if let Some(_) = map.get(&pos) {
                        pairs.push(pos.clone());
                    }

                    *map.entry(pos.clone()).or_insert(1) *= numparse;
                    continue;
                }

                if is_symble2(&table, x as i32 + 1, a as i32) {
                    let pos = format!("({},{})", x + 1, a);
                    if let Some(_) = map.get(&pos) {
                        pairs.push(pos.clone());
                    }

                    *map.entry(pos.clone()).or_insert(1) *= numparse;
                    continue;
                }
            }
        }
    }

    let mut total = 0;
    for p in &pairs {
        if let Some(v) = map.get(p) {
            total += *v;
        }
    }

    println!("{:?}", total);

    Ok(())
}
