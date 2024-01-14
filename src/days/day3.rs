
// iterate through string, find index of start of numbers
// then with set of start numbers, parse and check if symbol adjacent
// store all as list of lists
use std::fs;

fn check_if_symbol(lines: Vec<Option<&str>>) -> bool {
    let mut symbol = false;
    for line in lines {
        if let Some(l) = line {
            for ch in l.chars() {
                if None == ch.to_digit(10){
                    //filtered for numbers
                    match ch {
                        '.' => {},
                        _ => symbol = true,
                    }

                }
            }

        }
    }
    return symbol;
}

fn get_slice(line: &String, index: usize) -> &str {
    let length = line.chars().count();
    if index+2 >= line.chars().count() {
        let out = &line[index-1..];
        return out;

    }
    else if index == 0 {
        return &line[..index+2];
    }
    &line[index-1..index+2]
}

fn create_matrix<'a>(cur_line: &'a String, next_line: Option<&'a String>, prev_line: Option<&'a String>, index: usize) -> Vec<Option<&'a str>> {
    match (cur_line, next_line, prev_line) {
        (cur_line, Some(next_line), Some(prev_line)) => {
            return vec![
                Some(get_slice(prev_line, index)),
                Some(get_slice(cur_line, index)),
                Some(get_slice(next_line, index))
            ]
        },
        (cur_line, None, Some(prev_line)) => {
            return vec![
                Some(get_slice(prev_line, index)),
                Some(get_slice(cur_line, index)),
                None
            ]
        },
        (cur_line, Some(next_line), None) => {
            return vec![
                None,
                Some(get_slice(cur_line, index)),
                Some(get_slice(next_line, index)),
            ]
        },
        _ => return vec![None, None, None] // Should never get here
    }
}

fn process_line(cur_line: &String, prev_line: Option<&String>, next_line: Option<&String>) -> usize {
    let mut running_count = 0;
    let mut number = 0;
    let mut symbol = false;
    for (n, c) in cur_line.chars().enumerate() {
        if let Some(i) = c.to_digit(10) {
            number = number * 10 + i;
            symbol = symbol || check_if_symbol(create_matrix(cur_line, next_line, prev_line, n));
        }
        else if number > 0 {
            //end of number
            if symbol {
                running_count = running_count + number;
            }
            number = 0;
            symbol = false;
        }

    }
    if number > 0 {
        if symbol {
            running_count = running_count + number;
        }
    }
    return running_count as usize;
    
    // return number value (0 if not valid), index of last char
}

fn find_nums(line: &str, index: usize) -> Vec<usize> {
    match (
        line.chars().nth(index-1).unwrap().to_digit(10),
        line.chars().nth(index).unwrap().to_digit(10),
        line.chars().nth(index+1).unwrap().to_digit(10)
        ) {
        (None, None, None) => return vec![],
        (None, None, Some(x)) => return vec![index+1],
        (None, Some(x), None) => return vec![index],
        (None, Some(x), Some(y)) => return vec![index],
        (Some(x), None, None) => return vec![index-1],
        (Some(x), None, Some(y)) => return vec![index-1, index+1],
        (Some(x), Some(y), None) => return vec![index-1],
        (Some(x), Some(y), Some(z)) => return vec![index-1],
    }
}

fn get_number(line: &String, index: usize) -> usize {
    //read from current index left and right until not number
    let mut num = line.chars().nth(index).unwrap().to_digit(10).unwrap();
    let mut scale = 10;
    let mut i = index;
    loop {
        i = i - 1;
        if let Some(d) = line.chars().nth(i).unwrap().to_digit(10) {
            num = d * scale + num;
            scale = scale * 10;
        }
        else {
            break;
        }
    }
    let mut i = index;
    loop {
        i = i + 1;
        if let Some(d) = line.chars().nth(i).unwrap().to_digit(10) {
            num = d + num * 10;
        }
        else {
            break;
        }
    }
    num as usize
}

fn process_star(cur_line: &String, prev_line: &String, next_line: &String, index: usize) -> usize {
    let mut symbol = false;
    let mut nums_found: Vec<usize> = vec![];
    for line in [cur_line, prev_line, next_line].iter() {
        let found_nums = find_nums(line, index);
        for num_index in found_nums{
            let asdf = get_number(&line.to_string(), num_index);
            nums_found.push(asdf);
        }
    }
    let nums_found_len = nums_found.iter().count();
    if nums_found_len == 2 {
        return nums_found[0] * nums_found[1]
    }
    return 0;
}

fn process_line_part2(cur_line: &String, prev_line: &String, next_line: &String) -> usize {
    let mut running_count = 0;
    let mut number = 0;
    for (n, c) in cur_line.chars().enumerate() {
        if c == '*' {
            let num = process_star(cur_line, prev_line, next_line, n);
            running_count = running_count + num;
        }

    }
    return running_count as usize;
}


pub fn day3_val_part1() -> usize {
    let mut cal_val = 0;
    let mut lines: Vec<String> = vec![];
    for line in fs::read_to_string("./src/days/inputs/day3_input.txt").unwrap().lines() {
        lines.push(line.to_string());
    }
    for (n, line) in lines.iter().enumerate() {
        let number;
        if n == 0 {
            number = process_line(line, None, lines.get(n+1));

        }
        else {
            number = process_line(line, lines.get(n-1), lines.get(n+1));
        }
        cal_val = cal_val + number
    }

    cal_val
}

pub fn day3_val_part2() -> usize {
    let mut cal_val = 0;
    let mut lines: Vec<String> = vec![];
    for line in fs::read_to_string("./src/days/inputs/day3_input.txt").unwrap().lines() {
        let mut my_line = ".".to_string();
        my_line.push_str(line);
        my_line.push_str("..");
        lines.push(my_line);
    }
    let line_length = lines[0].chars().count();
    let buffer = (0..line_length).map(|_| ".").collect::<String>();
    let lines_num = lines.iter().count();
    for (n, line) in lines.iter().enumerate() {
        let number;
        if n == 0 {
            number = process_line_part2(line, &buffer, &lines[n+1]);
        }
        else if n+1 == lines_num {
            number = process_line_part2(line, &lines[n-1], &buffer);
        }
        else {
            number = process_line_part2(line, &lines[n-1], &lines[n+1]);
        }
        cal_val = cal_val + number
    }

    cal_val
}
