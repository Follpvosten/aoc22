fn sum_all_elves(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.split("\n\n").map(|cal_rows| {
        cal_rows
            .split_whitespace()
            // nit: no actual error handling
            .filter_map(|n| n.parse::<u32>().ok())
            .sum()
    })
}

pub fn find_max_calories(input: &str) -> u32 {
    sum_all_elves(input)
        .max()
        // no max means there were 0 items in the iterator
        // default to 0
        .unwrap_or(0)
}

pub fn top_3_calories_total(input: &str) -> u32 {
    // nit: std only implementation
    // itertools would make the whole function body:
    // sum_all_elves(input).sorted().rev().take(3).sum()
    let mut all_elves = sum_all_elves(input).collect::<Vec<_>>();
    all_elves.sort();
    all_elves.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {

    const EXAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn find_max_works() {
        assert_eq!(super::find_max_calories(EXAMPLE_INPUT), 24_000);
    }

    #[test]
    fn top_3_works() {
        assert_eq!(super::top_3_calories_total(EXAMPLE_INPUT), 45_000);
    }
}
