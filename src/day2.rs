mod utils;

fn main() {
    let content = utils::read_file("2").unwrap();

    let mut result = 0;

    content.trim_end().lines().for_each(|line| {
        let (_, data) = line.split_once(":").unwrap();

        let mut green: Vec<usize> = Vec::new();
        let mut blue: Vec<usize> = Vec::new();
        let mut red: Vec<usize> = Vec::new();

        data.split(";").for_each(|player| {
            let subsets: Vec<(&str, &str)> = player
                .split(",")
                .map(|play| play.trim_start().split_once(" ").unwrap())
                .collect();

            for ss in subsets {
                let qtd = ss.0.parse::<usize>().unwrap_or(0);

                match ss.1 {
                    "red" => red.push(qtd),
                    "green" => green.push(qtd),
                    "blue" => blue.push(qtd),
                    _ => panic!("invalid color"),
                }
            }
        });

        let max_green = green.iter().max().unwrap_or(&0);
        let max_red = red.iter().max().unwrap_or(&0);
        let max_blue = blue.iter().max().unwrap_or(&0);

        result += max_red * max_blue * max_green;
    });

    println!("{}", result)
}
