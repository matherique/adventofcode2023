use std::collections::HashMap;

mod utils;

fn main() {
    let content = utils::read_file("2").unwrap();

    let mut result = 0;
    let mut maximum: HashMap<&str, usize> = HashMap::new();

    maximum.insert("green", 13);
    maximum.insert("red", 12);
    maximum.insert("blue", 14);

    content.trim_end().lines().for_each(|line| {
        let (game_id, data) = line.split_once(":").unwrap();
        let (_, id) = game_id.split_once(" ").unwrap();

        let mut is_valid = true;
        data.split(";").for_each(|player| {
            let subsets: Vec<(&str, &str)> = player
                .split(",")
                .map(|play| play.trim_start().split_once(" ").unwrap())
                .collect();

            for ss in subsets {
                let qtd = ss.0.parse::<usize>().unwrap_or(0);
                if qtd > *maximum.get(ss.1).unwrap() {
                    is_valid = false;
                    return;
                }
            }
        });

        if is_valid {
            result += id.parse::<i32>().unwrap();
        }
    });

    println!("{}", result)
}
