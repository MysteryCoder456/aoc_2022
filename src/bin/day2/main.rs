use input::input::get_input;

/// lose = -1
/// draw = 0
/// win = +1
fn calculate_game_outcome(elf: &str, me: &str) -> i32 {
    let win_states = [("A", "Y"), ("B", "Z"), ("C", "X")];

    for (e, m) in win_states {
        if e == elf && m == me {
            return 1;
        }
    }

    let lose_states = [("X", "B"), ("Y", "C"), ("Z", "A")];

    for (m, e) in lose_states {
        if e == elf && m == me {
            return -1;
        }
    }

    // Nothing matched, draw
    0
}

fn main() {
    let input = get_input(2);
    let total_points = input
        .lines()
        .map(|line| {
            let (elf, me) = line.split_once(" ").unwrap();

            let move_points = match me {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => 0,
            };

            let game_outcome = calculate_game_outcome(elf, me);
            let outcome_points = (game_outcome + 1) * 3;

            outcome_points + move_points
        })
        .sum::<i32>();

    println!("Total points: {total_points}");
}
