use std::{collections::BTreeMap, io::stdin};

fn read_lists() -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in stdin().lines() {
        let line = line
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|split| split.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        list1.push(line[0]);
        list2.push(line[1]);
    }

    (list1, list2)
}

fn part1(list1: &[u32], list2: &[u32]) -> u32 {
    let list1 = {
        let mut temp = list1.to_vec();
        temp.sort_unstable();
        temp
    };

    let list2 = {
        let mut temp = list2.to_vec();
        temp.sort_unstable();
        temp
    };

    list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|(list1_i, list2_i)| list1_i.abs_diff(list2_i))
        .sum()
}

fn part2(list1: &[u32], list2: &[u32]) -> u32 {
    let list2 = {
        let mut counter = BTreeMap::new();

        for i in list2 {
            counter
                .entry(i)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        counter
    };

    list1.iter().map(|i| i * list2.get(i).unwrap_or(&0)).sum()
}

fn main() {
    let (list1, list2) = read_lists();
    let part1_result = part1(&list1, &list2);
    let part2_result = part2(&list1, &list2);

    println!(
        "Part 1 result: {}\nPart 2 result: {}",
        part1_result, part2_result
    );
}
