use std::collections::HashMap;

use input::input::get_input;
use itertools::Itertools;

fn part1(input: &str) -> String {
    let (initial_stack, procedure) = input.split_once("\n\n").unwrap();
    let max_stack_size = initial_stack.lines().count() - 1;

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();

    initial_stack
        .lines()
        .rev()
        .enumerate()
        .for_each(|(i, line)| {
            if i != 0 {
                for (j, ch) in line.chars().enumerate() {
                    if !ch.is_alphabetic() {
                        continue;
                    }

                    let n = (j - 1) / 4 + 1;

                    if let Some(s) = stacks.get_mut(&n) {
                        s.push(ch);
                    } else {
                        stacks.insert(n, vec![ch]);
                    }
                }
            }
        });

    for instruction in procedure.lines() {
        let s = instruction.split(" ").collect::<Vec<_>>();

        let move_count = s.get(1).unwrap().parse::<usize>().unwrap();
        let from = s.get(3).unwrap().parse::<usize>().unwrap();
        let to = s.get(5).unwrap().parse::<usize>().unwrap();

        for _ in 0..move_count {
            let from_stack = stacks.get_mut(&from).unwrap();
            let cra = from_stack.pop().unwrap();

            let to_stack = stacks.get_mut(&to).unwrap();
            to_stack.push(cra);
        }
    }

    dbg!(&stacks);

    stacks
        .iter()
        .sorted_by_key(|(i, _)| **i)
        .map(|(_, sta)| sta.last().unwrap().to_owned())
        .collect::<String>()
}

fn part2(input: &str) -> &str {
    todo!()
}

fn main() {
    let input = get_input(5);

    println!("Part 1 Output: {}", part1(&input));
    // println!("Part 2 Output: {}", part2(&input));
}
