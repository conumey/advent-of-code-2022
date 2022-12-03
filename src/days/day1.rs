pub fn do_day_1(){
    let input = super::super::read_input(1);
    let mut elf_food: Vec<i32> = Vec::new();
    let mut elf_calories: Vec<i32> = Vec::new();

    for line in input{
        if line.chars().count() > 0 {
            elf_food.push(line.parse().expect("cannot parse number"));
        } else {
            let mut i:i32 = 0;
            for food in elf_food{
                i += food
            }
            elf_calories.push(i);
            elf_food = Vec::new()
        }
    }

    if elf_food.len() > 0 {
        let mut i:i32 = 0;
        for food in elf_food{
            i += food
        }
        elf_calories.push(i)
    }

    elf_calories.sort_unstable_by(|a, b| b.cmp(a));

    let top_three_slice = &elf_calories[0..3];

    let top_three_total: i32 = top_three_slice.iter().sum();
    
    println!("Highest number of calories on a single elf: {}", elf_calories.iter().max().expect("can't parse calories").to_string());

    println!("Total number of calories on top 3 elves: {}", top_three_total.to_string());
}

