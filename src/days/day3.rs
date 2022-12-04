use std::convert::TryInto;

pub fn do_day_3() {
    let alphabet:Vec<char> = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    let input = super::super::read_input(3);

    let mut score:i32 = 0;

    for line in &input {
        let bag:Vec<char> = line.chars().collect();
        let size:usize = line.chars().count();
        let bags:Vec<Vec<char>> = bag.chunks(size/2).map(|s| s.into()).collect();
        for i in 0..size/2 {
            let character = bags[0][i];
            let matched = bags[1].iter().find(|x| x == &&character);
            if matched.is_some() {
                score += get_character_value(*matched.unwrap(), alphabet.to_owned());
                break;
            }
        }
    }

    println!("Sum total of item priorities part 1: {}", score);

    let mut score2 = 0;

    for i in (0..input.len()).step_by(3){
        let bag1:Vec<char> = input[i].chars().collect();
        let bag2:Vec<char> = input[i+1].chars().collect();
        let bag3:Vec<char> = input[i+2].chars().collect();

        for item in bag1 {
            if bag2.iter().find(|x| x == &&item).is_some(){
                if bag3.iter().find(|x| x == &&item).is_some(){
                    score2 += get_character_value(item, alphabet.to_owned());
                    break;
                }
            }
        }
    }

    println!("Sum total of item priorities part 2: {}", score2);
}

fn get_character_value(ch:char, alpha:Vec<char>)-> i32{
    return alpha.iter().position(|&x| x == ch).unwrap().try_into().unwrap();
}