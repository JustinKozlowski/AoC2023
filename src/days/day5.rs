use std::fs;
use std::collections::HashMap;

fn load_map(map: HashMap<usize, usize>, line: String) -> HashMap<usize, usize> {
    let range_data: Vec<usize> = line.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
    let source_start = range_data[0];
    let target_start = range_data[1];
    let range = range_data[2];
    for i in 0..range {
        map.insert(source_start + i, target_start + i);
    }
    map
}

fn read_map(&map: &HashMap<usize, usize>, key: usize) -> usize {
    match map.get(&key) {
        Some(x) => return *x,
        None => return key
    }
}

fn parse_seed_line(s: &String) -> Vec<usize> {
    let t = s.split(":").collect::<Vec<&str>>();
    t[1].trim().split(' ').map(|x| x.parse::<usize>().unwrap()).collect()
}

pub fn day5_val_part1() -> usize {
    let mut temp_map: HashMap<usize, usize> = HashMap::new();

    let mut lines: Vec<String> = vec![];
    for line in fs::read_to_string("./src/days/inputs/day5_input.txt").unwrap().lines() {
        lines.push(line.to_string());
    }

    let mut seeds: Vec<usize> = parse_seed_line(&lines[0]);
    
    let mut i = 3;
    loop {
        if let Some(line) = lines.get(i) {
            if line == "" {
                //run translations
                //do this with mapped iterator
                seeds = seeds.iter().map(|x| read_map(&temp_map, *x)).collect();
                //reset map
                temp_map = HashMap::new();
                i += 1; // Skip label line
            }
            else{
                temp_map = load_map(temp_map, line.to_string());
            }
            i += 1;
        }
        else {
            break;
        }
    }
    *seeds.iter().min().unwrap()
}
