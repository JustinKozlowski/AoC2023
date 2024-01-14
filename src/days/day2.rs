use std::fs;


#[derive(Copy, Clone, Debug)]
struct Round {
    red: usize,
    blue: usize,
    green: usize
}

#[derive(Copy, Clone, Debug)]
struct GameConstraints {
    max_red: usize,
    max_green: usize,
    max_blue: usize
}

#[derive(Copy, Clone, Debug)]
struct GameAttributes {
    min_red: usize,
    min_green: usize,
    min_blue: usize
}

const GAME_CONSTRAINTS: GameConstraints = GameConstraints{
    max_red: 12,
    max_blue: 14,
    max_green: 13
};

fn process_round_part1(game: GameConstraints, round: Round)-> bool {
    return !(round.red > game.max_red || round.blue > game.max_blue || round.green > game.max_green);
}

fn process_round_part2(mut game: GameAttributes, round: Round)-> GameAttributes {
    if round.red > game.min_red {
        game.min_red = round.red;
    }
    if round.green > game.min_green {
        game.min_green = round.green;
    }
    if round.blue > game.min_blue {
        game.min_blue = round.blue;
    }
    return game;

}

fn parse_round(s: String) -> Round {
    let details = s.split(",");
    let trim_details = details.map(|x| x.trim());
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for detail in trim_details{
        let mut z = detail.split(" ");
        let val = z.next();
        let color = z.next();
        match color {
            Some("red") => red = val.unwrap().parse().unwrap(),
            Some("blue") => blue = val.unwrap().parse().unwrap(),
            Some("green") => green = val.unwrap().parse().unwrap(),
            Some(&_) => {},
            None => {}
        }
    }
    return Round{
        red: red,
        blue: blue,
        green: green
            };

}

fn parse_game(s: String) -> (usize, String) {
    let game_vec: Vec<&str> = s.split(":").collect();
    let game: &str = game_vec[0];
    let id: Vec<&str> = game.split(" ").collect();
    return (id[1].parse().unwrap(), game_vec[1].to_string())
}

pub fn day2_val_part2() -> usize {
    let mut cal_val = 0;
    for line in fs::read_to_string("./src/days/inputs/day2_input.txt").unwrap().lines() {
        let mut game_attributes = GameAttributes{
            min_red: 0,
            min_blue: 0,
            min_green: 0,
        };
        let (_, game_details) = parse_game(line.to_string());
        
        for round in game_details.split(";").map(|x| parse_round(x.to_string())) {
            game_attributes = process_round_part2(game_attributes, round);
        }
        cal_val = cal_val + game_attributes.min_red * game_attributes.min_blue * game_attributes.min_green;
    }

    cal_val
}

pub fn day2_val_part1() -> usize {
    let mut cal_val = 0;
    'game: for line in fs::read_to_string("./src/days/inputs/day2_input.txt").unwrap().lines() {
        let (id, game_details) = parse_game(line.to_string());
        
        for round in game_details.split(";").map(|x| parse_round(x.to_string())) {
            if !process_round_part1(GAME_CONSTRAINTS, round){
                continue 'game;
            }

        }
        cal_val = cal_val + id;
    }

    cal_val
}
