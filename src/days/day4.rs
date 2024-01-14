use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

fn parse_card(s: String) -> usize {
    let card_details: Vec<&str> = s.split("|").map(|x| x.trim()).collect();
    let mut winning_numbers = HashSet::new();
    let _: Vec<_> = card_details[0].split(' ').map(|x| winning_numbers.insert(x)).collect();
    winning_numbers.remove(" ");
    winning_numbers.remove("");
    let mut value: usize = 0;
    for n in card_details[1].split(' '){
        if winning_numbers.contains(n){
            if value == 0 {
                value = 1;
            }
            else {
                value = value * 2;
            }
        }
    }
    value
}

pub fn day4_val_part1() -> usize {
    let mut cal_val = 0;
    for line in fs::read_to_string("./src/days/inputs/day4_input.txt").unwrap().lines() {
        let card_details: Vec<&str> = line.split(":").collect();
        cal_val = cal_val + parse_card(card_details[1].to_string());
    }
    cal_val
}


fn parse_card_part2(s: &String, card: usize) -> usize {
    let card_details: Vec<&str> = s.split("|").map(|x| x.trim()).collect();
    let mut winning_numbers = HashSet::new();
    let _: Vec<_> = card_details[0].split(' ').map(|x| winning_numbers.insert(x)).collect();
    winning_numbers.remove(" ");
    winning_numbers.remove("");
    let mut value: usize = 0;
    for n in card_details[1].split(' '){
        if winning_numbers.contains(n){
            value = value + 1;
        }
    }
    value
}

pub fn day4_val_part2() -> usize {
    let mut cal_val = 0;
    let mut lines: Vec<String> = vec![];
    for line in fs::read_to_string("./src/days/inputs/day4_input.txt").unwrap().lines() {
        let card_details: Vec<&str> = line.split(":").collect();
        lines.push(card_details[1].to_string());
    }
    let mut q = vec![];
    let mut winners: HashMap<usize, usize> = HashMap::new();

    for (n, _) in lines.iter().enumerate() {
        q.push(n);
    }
    let max_card = lines.iter().count();
    // println!("Max: {max_card}");
    while let Some(card) = q.pop() {
        cal_val += 1;
        let copies;
        if winners.contains_key(&card){
            copies = *winners.get(&card).unwrap();
        }
        else {
            copies = parse_card_part2(&lines[card], card);
            winners.insert(card, copies);
        }
        // println!("{card} has {copies} winners");
    
        for i in 0..copies {
            let copied_card = card + i + 1;
            if copied_card <= max_card {
                // println!("Pushing: {copied_card}");
                q.push(copied_card);
            }
            else {
                println!("Skipped: {copied_card}");
            }
        }
    }
    cal_val
}
