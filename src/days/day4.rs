pub fn do_day_4 () {
    println!("Day 4:");

    let input = super::super::read_input(4);

    let mut sections_contained:i32 = 0;
    let mut sections_overlap:i32 = 0;

    for line in input {
        let split: Vec<&str> = line.split(",").collect();
        let elf1: Vec<i16> = split[0].split("-").map(|x| x.parse::<i16>().unwrap()).collect();
        let elf2: Vec<i16> = split[1].split("-").map(|x| x.parse::<i16>().unwrap()).collect();
        let section1 = (elf1[0], elf1[1]);
        let section2 = (elf2[0], elf2[1]);

        if section1.0 == section2.0 { //if equal no need to check, one definitely fits in the other
            sections_contained += 1;
            sections_overlap += 1;
        } if is_contained(section1.0, section2) {
            sections_overlap += 1;
            if is_contained(section1.1, section2) {
                sections_contained +=1;
            }
        } else if is_contained(section2.0, section1) {
            sections_overlap += 1;
            if is_contained(section2.1, section1) {
                sections_contained +=1;
            }
        }
    }

    println!("  Part 1: {}", sections_contained);
    println!("  Part 2: {}", sections_overlap);
}

fn is_contained(num: i16, container: (i16, i16)) -> bool{
    if num < container.0 || num > container.1 {
        return false;
    } else {
        return true;
    }
}