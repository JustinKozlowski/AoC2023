use std::fs;


fn read_from_string(s: String) -> Option<isize> {
        if let Some(x) = s.chars().nth(s.chars().count()-1)?.to_digit(10) {
            return Some(x as isize);
        }
        match s.as_str() {
            s if s.ends_with("zero") => Some(0), 
            s if s.ends_with("one") => Some(1), 
            s if s.ends_with("two") => Some(2), 
            s if s.ends_with("three") => Some(3), 
            s if s.ends_with("four") => Some(4), 
            s if s.ends_with("five") => Some(5), 
            s if s.ends_with("six") => Some(6), 
            s if s.ends_with("seven") => Some(7), 
            s if s.ends_with("eight") => Some(8), 
            s if s.ends_with("nine") => Some(9),
            _ => None
        } 
}

fn num_from_str(line: String) -> isize {
    let mut first: isize = 0;
    let mut last: isize = 0;
    let mut char_iter = line.chars();
    let mut s: String = String::new();
    while let Some(c) = char_iter.next() {
        s.push(c);
        if let Some(c_int) = read_from_string(s.clone()){
            if first == 0 {
                first = c_int as isize;
            }
            last = c_int as isize;
        }
    }
    let output = first * 10 + last;
    return output;
}

pub fn cal_val() -> isize {
    let mut cal_val = 0;
    for line in fs::read_to_string("./src/days/inputs/day1_input.txt").unwrap().lines() {
        cal_val = cal_val + num_from_str(line.to_string());
    }

    cal_val
}
