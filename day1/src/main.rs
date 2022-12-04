fn main() {
    const INPUT: &str = include_str!("../input.txt");

    println!("Answer Part 1: {}", day1::find_max_calories(INPUT));
    println!("Answer Part 2: {}", day1::top_3_calories_total(INPUT));
}
