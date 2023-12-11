use std::{str::FromStr, string::ParseError};

mod utils;

fn main() {
    println!("ola mundo");
}

#[derive(Debug, Default, PartialEq)]
enum SourceDest {
    #[default]
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl SourceDest {
    fn new(data: &str) -> Self {
        match data {
            "seed" => SourceDest::Seed,
            "soil" => SourceDest::Soil,
            "fertilizer" => SourceDest::Fertilizer,
            "water" => SourceDest::Water,
            "light" => SourceDest::Light,
            "temperature" => SourceDest::Temperature,
            "humidity" => SourceDest::Humidity,
            "location" => SourceDest::Location,
            _ => SourceDest::Seed,
        }
    }
}

#[derive(Debug, Default)]
struct Range {
    source: usize,
    dest: usize,
    lenght: usize,
}

#[derive(Debug, Default)]
struct Almanac {
    source: SourceDest,
    destination: SourceDest,
    range: Vec<Range>,
}

impl FromStr for Almanac {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (source_dest, rest) = s.split_once("\n").unwrap();

        let sd = source_dest.replace(" map:", "");
        let (source, dest) = sd.split_once("-to-").unwrap();

        let mut range: Vec<Range> = Vec::new();
        for line in rest.split("\n") {
            let info: Vec<usize> = line
                .split_whitespace()
                .map(|i| i.parse::<usize>().unwrap_or(0))
                .collect();

            let source = info.get(1).unwrap_or(&0);
            let dest = info.get(0).unwrap_or(&0);
            let length = info.get(2).unwrap_or(&0);

            range.push(Range {
                dest: *dest,
                source: *source,
                lenght: *length,
            })
        }

        let almanac = Almanac {
            source: SourceDest::new(source),
            destination: SourceDest::new(dest),
            range,
        };

        return Ok(almanac);
    }
}

impl Almanac {
    fn get_dest(&self, source: usize) -> usize {
        if let Some(idx) = self
            .range
            .iter()
            .position(|r| source >= r.source && source <= r.source + r.lenght)
        {
            if let Some(r) = self.range.get(idx) {
                let diff = source - r.source;
                if diff < r.lenght {
                    return diff + r.dest;
                }

                return source;
            }
        } else {
            return source;
        }

        return 0;
    }
}

fn part1(input: String) -> usize {
    let (seed, rest) = input.split_once("\n\n").unwrap_or_default();
    let seed_numbers = seed
        .replace("seeds: ", "")
        .trim()
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap_or(0))
        .collect::<Vec<usize>>();

    let almanacs: Vec<Almanac> = rest
        .split("\n\n")
        .map(|r| r.parse::<Almanac>().unwrap_or_default())
        .collect();

    let mut locations: Vec<usize> = Vec::new();
    for sn in seed_numbers {
        let seed_soil = almanacs[0].get_dest(sn);
        let soil_fertil = almanacs[1].get_dest(seed_soil);
        let fertil_watter = almanacs[2].get_dest(soil_fertil);
        let watter_light = almanacs[3].get_dest(fertil_watter);
        let light_temp = almanacs[4].get_dest(watter_light);
        let temp_humidity = almanacs[5].get_dest(light_temp);
        let humitidy_location = almanacs[6].get_dest(temp_humidity);

        println!(
            "Seed {:?}, soil {:?}, fertilizer {:?}, watter {:?}, light {:?}, temperature {:?}, humidity {:?}, location {:?}",
            sn, seed_soil, soil_fertil, fertil_watter, watter_light, light_temp, temp_humidity, humitidy_location
        );

        locations.push(humitidy_location)
    }

    return *locations.iter().min().unwrap();
}

fn part2(input: String) -> usize {
    return 0;
}

#[test]
fn test_part1_example() {
    let content = utils::read_file("5.example").unwrap();
    let response = part1(content);

    assert_eq!(response, 35);
}

#[test]
fn test_part1() {
    let content = utils::read_file("5").unwrap();
    let response = part1(content);

    assert_eq!(response, 26273516);
}

/*
#[test]
fn test_part2_example() {
    let content = utils::read_file("5").unwrap();
    let response = part2(content);

    if let Ok(_) = env::var("EXAMPLE") {
        assert_eq!(response, 30);
    } else {
        assert_eq!(response, 23806951);
    }
}
*/
