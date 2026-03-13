use std::fs;

fn main() {
    let input_path = "input/day01.txt";
    let instructions = fs::read_to_string(input_path).expect("Day01.txt not found in input dir");

    let mut start_pos = 50;
    let mut zero_count = 0;

    for i in instructions.lines() {
        let curr_move = &i[1..]
            .parse::<isize>()
            .expect("Couldn't parse rotational value");


        match i.chars().next() {
            Some('L') => {
                if start_pos == 0 {
                    zero_count -= 1;
                }
                start_pos += -1 * curr_move;
            },
            Some('R') => {
                start_pos += curr_move;
            },
            _ => panic!("The input is out of format"),
        }
       
        if start_pos < 0 {
            zero_count += start_pos / (-100) + 1;
        } else if start_pos >= 100 {
            zero_count += start_pos / 100;
        } else if start_pos == 0 {
            zero_count += 1;
        }

        start_pos = start_pos.rem_euclid(100);
    }

    println!("{zero_count}");
}
