use input::input::get_input;

fn main() {
    let input = get_input(1);
    let mut max_calories = 0;

    input.split("\n\n").for_each(|elf| {
        let mut elf_calories = 0;

        elf.split("\n")
            .for_each(|calories_str| match calories_str.parse::<i32>() {
                Ok(calories) => elf_calories += calories,
                Err(_) => {}
            });

        if elf_calories > max_calories {
            max_calories = elf_calories;
        }
    });

    println!("{max_calories}");
}
