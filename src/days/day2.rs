use phf::{phf_map};

static SCORE_MAP: phf::Map<&'static str, &'static i32> = phf_map! {
    "A" => &1,
    "B" => &2,
    "C" => &3,
};

static BEAT_BY: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "B",
    "B" => "C",
    "C" => "A",
};

static BEATS: phf::Map<&'static str, &'static str> = phf_map! {
    "A" => "C",
    "B" => "A",
    "C" => "B",
};

static MOVE_MAP: phf::Map<&'static str, &'static str> = phf_map! {
    "X" => "A",
    "Y" => "B",
    "Z" => "C",
};

pub fn do_day_2(){
    println!("Day 2:");
    let input = super::super::read_input(2);
    let mut moves: Vec<(String, String)> = Vec::new();

    for lines in input{
        let split: Vec<_> = lines.split(" ").map(|s| s.to_string()).collect();
        moves.push((split[0].to_owned(), split[1].to_owned()));
    }

    let mut score = 0;

    for round in moves.as_slice(){
        let my_move = MOVE_MAP.get(&round.1).unwrap().to_string();
        let their_move = &round.0;
        score += get_score(&my_move, their_move);
    }

    println!("  Part 1: {}", score);

    let mut score2 = 0;

    for round in moves.as_slice(){
        let their_move = round.0.to_owned();
        let mut my_move = round.1.to_owned();

        match my_move.as_str() {
            "X" => my_move = BEATS.get(&their_move).unwrap().to_string(),
            "Y" => my_move = their_move.to_owned(),
            "Z" => my_move = BEAT_BY.get(&their_move).unwrap().to_string(),
            _ => println!("Match not found"),
        }

        score2 += get_score(&my_move, &their_move)
    }

    println!("  Part 2: {}", score2);

}

fn get_score(my_move:&String, their_move:&String) -> i32{
    let move_score = SCORE_MAP.get(my_move).unwrap().to_owned();
    if my_move == their_move {
        return move_score + 3;
    }
    else if BEAT_BY.get(my_move).unwrap() == their_move {
        return *move_score;
    } else {
        return move_score + 6;
    }
}



