use std::io::Result;

mod utils;

#[derive(Debug)]
struct Symble {
    s: char,
    x: usize,
    y: usize,
}

impl Symble {
    fn is_x_valid(&self) -> bool {
        return self.x as i32 - 1 >= 0;
    }

    fn is_y_valid(&self) -> bool {
        return self.y as i32 - 1 >= 0;
    }

    fn is_x_max(&self, size: usize) -> bool {
        return self.x >= size - 1;
    }

    fn is_y_max(&self, size: usize) -> bool {
        return self.y >= size - 1;
    }
}

#[derive(Debug)]
struct Numbers {
    n: usize,
    size: usize,
    x: usize,
    y: usize,
}

fn get_full_numbers(chrs: &Vec<Vec<char>>, x: usize, y: usize, numbers: &mut Vec<Numbers>) {
    let mut nb = String::from("");

    {
        let mut i: usize = 0;
        loop {
            if y < i {
                break;
            }

            if chrs[x][y - i].is_digit(10) {
                nb = chrs[x][y - i].to_string() + &nb;
                i += 1;
                continue;
            }

            break;
        }
    }

    {
        let mut i: usize = 1;
        loop {
            if y + i > chrs[x].len() - 1 {
                break;
            }

            if chrs[x][y + i].is_digit(10) {
                nb += &chrs[x][y + i].to_string();
                i += 1;
                continue;
            }

            break;
        }
    }

    let actual_number: String = nb.chars().collect();
    let n = actual_number.parse::<usize>().unwrap_or(0);
    let size = nb.len();

    for number in &mut *numbers {
        if number.n == n && number.x == x && y >= number.y {
            println!("check {} {} {} {} {} {} ", number.n, n, number.x, x, number.y, y);
            return;
        }
    }

    numbers.push(Numbers { n, size, x, y });
}

fn main() -> Result<()> {
    let content = utils::read_file("3")?;

    let cols = content.trim_end().lines();

    let mut symbles: Vec<Symble> = Vec::new();
    let mut chrs: Vec<Vec<char>> = Vec::new();

    for (x, line) in cols.enumerate() {
        let mut l: Vec<char> = Vec::new();
        for (y, c) in line.chars().enumerate() {
            l.push(c);
            let is_symble = match c {
                '/' | '!' | '@' | '#' | '$' | '%' | '&' | '*' | '-' | '+' | '=' => true,
                _ => false,
            };

            if is_symble {
                symbles.push(Symble { s: c, x, y });
            }
        }
        chrs.push(l);
    }

    // a b c
    // d z e
    // f g h
    //
    // a = (i-1), (j-1)
    // b = (i-1), (j)
    // c = (i-1), (j+1)
    // d = (i)  , (j-1)
    // e = (i)  , (j+1)
    // f = (i+1), (j-1)
    // g = (i+1), (j)
    // h = (i+1), (j+1)
    //
    let mut numbers: Vec<Numbers> = Vec::new();

    for s in symbles {
        if s.is_x_valid() && s.is_y_valid() && chrs[s.x - 1][s.y - 1].is_digit(10) {
            let x = s.x - 1;
            let y = s.y - 1;

            get_full_numbers(&chrs, x, y, &mut numbers);
        }

        if s.is_x_valid() && chrs[s.x - 1][s.y].is_digit(10) {
            let x = s.x - 1;
            let y = s.y;

            get_full_numbers(&chrs, x, y, &mut numbers);
        }

        if s.is_x_valid() && !s.is_y_max(chrs[0].len()) && chrs[s.x - 1][s.y + 1].is_digit(10) {
            let x = s.x - 1;
            let y = s.y + 1;

            get_full_numbers(&chrs, x, y, &mut numbers);
        }

        if s.is_y_valid() && chrs[s.x][s.y - 1].is_digit(10) {
            let x = s.x;
            let y = s.y - 1;

            get_full_numbers(&chrs, x, y, &mut numbers);
        }

        if !s.is_y_max(chrs[0].len()) && chrs[s.x][s.y + 1].is_digit(10) {
            let x = s.x;
            let y = s.y + 1;

            get_full_numbers(&chrs, x, y, &mut numbers);
        }

        if s.is_y_valid() && !s.is_x_max(chrs[0].len()) && chrs[s.x + 1][s.y - 1].is_digit(10) {
            let x = s.x + 1;
            let y = s.y - 1;

            get_full_numbers(&chrs, x, y, &mut numbers);
        }

        if !s.is_x_max(chrs[0].len()) && chrs[s.x + 1][s.y].is_digit(10) {
            let x = s.x + 1;
            let y = s.y;

            get_full_numbers(&chrs, x, y, &mut numbers);
        }

        if !s.is_x_max(chrs[0].len())
            && !s.is_y_max(chrs[0].len())
            && chrs[s.x + 1][s.y + 1].is_digit(10)
        {
            let x = s.x + 1;
            let y = s.y + 1;

            get_full_numbers(&chrs, x, y, &mut numbers);
        }
    }

    let mut total: usize = 0;

    for n in numbers {
        // println!("{:?}", n);
        total += n.n
    }

    println!("{}", total);

    Ok(())
}
