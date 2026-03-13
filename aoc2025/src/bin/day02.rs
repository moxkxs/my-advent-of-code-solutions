use std::fs;

fn main() {
    let raw_input = fs::read_to_string("input/day02.txt")
        .expect("Failed to read input");
    let collected_input: Vec<&str> = raw_input.trim().split(",")
        .collect();

    let range_input: Vec<(usize, usize)> = collected_input.iter()
        .map(|x| {
                let mut iter = x.split("-");
                let first = iter.next().unwrap().parse::<usize>().expect("Couldn't parse first range val");
                let second = iter.next().unwrap().parse::<usize>().expect("Couldn't parse second range val");
                (first, second)
            })
        .collect();

    let mut running_total = 0;

    for ids in range_input {
        for id in ids.0..=ids.1 {
            let temp = id.to_string();
            if temp.len() % 2 == 0 && temp[..temp.len()/2] == temp[temp.len()/2..] {
                running_total += id
            }
        }
    }

    println!("{running_total}");
}
