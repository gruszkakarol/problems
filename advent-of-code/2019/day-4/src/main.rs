use std::collections::HashSet;
use std::ops::Range;
use std::str::Chars;

fn get_range() -> Range<i32> {
    (235741..706948)
}

fn check_descend(nums: &Vec<u32>) -> bool {
    let mut previous = nums[0];
    nums.into_iter().all(|num| {
        let not_smaller = num >= &previous;
        previous = *num;
        not_smaller
    })
}

fn has_siblings(nums: &Vec<u32>) -> bool {
    let mut occurrences: HashSet<&u32> = HashSet::new();

    for num in nums {
        if occurrences.contains(num) {
            return true;
        } else {
            occurrences.insert(num);
        }
    }
    false
}

fn part_one() -> usize {
    let mut matches: Vec<i32> = Vec::new();
    for num in get_range() {
        let digits: Vec<u32> = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).expect("This shouldn't happen"))
            .collect();

        if check_descend(&digits) && has_siblings(&digits) {
            matches.push(num);
        }
    }
    matches.len()
}

fn no_bigger_groups(nums: &Vec<u32>, num: i32) -> bool {
    let mut digit_counter = [1usize; 10];
    let mut previous = &nums[0];
    nums.iter().skip(1).for_each(|num| {
        if previous == num {
            digit_counter[*num as usize] += 1;
        }
        previous = num;
    });
    digit_counter.iter().find(|num| **num == 2).is_some()
}

fn part_two() -> usize {
    let mut matches: Vec<i32> = Vec::new();

    for num in get_range() {
        let digits: Vec<u32> = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).expect("This shouldn't happen"))
            .collect();

        if check_descend(&digits) && no_bigger_groups(&digits, num) {
            matches.push(num);
        }
    }
    matches.len()
}

fn main() {
    println!("PART ONE: {}", part_one());
    println!("PART TWO: {}", part_two());
}
