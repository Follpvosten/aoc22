use day2::Rounds;

fn main() {
    let res = include_str!("../input.txt")
        .parse::<Rounds>()
        .expect("input should parse");
    println!("Answer Part 2: {}", res.evaluate_all());
}
