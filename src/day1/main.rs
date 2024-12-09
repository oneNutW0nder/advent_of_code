use std::io::Read;

fn part_1(list_one: &Vec<u32>, list_two: &Vec<u32>) {
    // Sort the lists least -> greatest
    println!("{:?}", list_one);
    println!("{:?}", list_two);

    // Take absolute value of difference between corresponding numbers
    let mut acum = 0;
    for (n1, n2) in std::iter::zip(list_one, list_two) {
        // let t1 = *n1;
        // let t2 = *n2;
        acum += n1.abs_diff(*n2);
    }

    println!("Part 1 => {acum}");
}

fn part_2(list_one: &Vec<u32>, list_two: &Vec<u32>) {
    todo!();
}

fn main() {
    // Take input and make 2 lists
    println!("Give me your input (Press 'Ctrl+D' when finished):");
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let nums = input.split_ascii_whitespace();

    let mut list_one = Vec::new();
    let mut list_two = Vec::new();
    for (i, n) in nums.enumerate() {
        match i % 2 {
            0 => list_one.push(n.parse::<u32>().unwrap()),
            _ => list_two.push(n.parse::<u32>().unwrap()),
        }
    }

    list_one.sort();
    list_two.sort();
    part_1(&list_one, &list_two);
}
