use input::input::get_input;

fn get_item_priority(item: char) -> i32 {
    let alphas = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<_>>();
    let base_priority = alphas
        .iter()
        .position(|&i| i == item.to_ascii_lowercase())
        .unwrap() as i32
        + 1;

    if item.is_uppercase() {
        return base_priority + 26;
    }
    base_priority
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let rucksack_size = line.len();
            let comp1 = &&line[..(rucksack_size / 2)];
            let comp2 = &&line[(rucksack_size / 2)..];
            println!("{} = {} + {}", line, comp1, comp2);

            let common_item = comp1
                .chars()
                .filter(|&item| comp2.contains(item))
                .next()
                .unwrap();
            let priority = get_item_priority(common_item);

            priority
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    println!("Not implemented yet!");
    0
}

fn main() {
    let input = get_input(3);

    println!("Part 1 Output: {}", part1(&input));
    println!("Part 2 Output: {}", part2(&input));
}
