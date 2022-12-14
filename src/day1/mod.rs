pub fn main() {
    let input = include_str!("a.txt");

    let mut elves: Vec<i32> = Vec::new();
    let mut calories_sum = 0;
    input.lines().for_each(|l| {
        let calories = l.parse::<i32>();

        match calories {
            Ok(c) => calories_sum = calories_sum + c,
            Err(_) => {
                elves.push(calories_sum);
                calories_sum = 0;
            }
        };
    });
    elves.push(calories_sum);
    elves.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    // Solution to first problem
    let most_calories_elf: i32 = elves.iter().take(1).sum();
    // let most_calories_elf = elves.pop().unwrap();
    println!("{most_calories_elf}");

    // Solution to second problem
    let most_calories_elves: i32 = elves.iter().take(3).sum();
    // let mut most_calories_elves = elves.pop().unwrap();
    // most_calories_elves = most_calories_elves + elves.pop().unwrap();
    // most_calories_elves = most_calories_elves + elves.pop().unwrap();

    println!("{most_calories_elves}");
}
