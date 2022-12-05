use std::str::FromStr;

pub fn main() {
    let input = include_str!("./input.txt");
    let elves = input
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>()))
        .collect::<Vec<Vec<i32>>>();
    println!("{:?}", elves.into_iter());

    get_most_calories(elves)
}

fn get_most_calories(elves: Vec<Vec<i32>>) -> i32 {
    let calories: Vec<i32> = elves
        .into_iter()
        .flat_map(|elf| elf.into_iter().reduce(|a, b| a + b))
        .collect::<Vec<i32>>();

    let most_calories: i32 = calories.into_iter().max().unwrap();

    most_calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let elves: Vec<Vec<i32>> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        let most_calories = get_most_calories(elves);
        assert_eq!(24000, most_calories);
    }

    #[test]
    fn this_test_will_fail() {
        let elves: Vec<Vec<i32>> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        let most_calories = get_most_calories(elves);
        assert_ne!(2000, most_calories);
    }
}
