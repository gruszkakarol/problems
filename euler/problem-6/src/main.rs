use std::ops::Range;

fn main() {
    let sum_square = (1..101).sum::<i64>().pow(2);
    let numbers_square_sum: i64 = (1..101).map(|num: i64| num.pow(2)).sum();
    println!(
        "The answer to the sixth problem is {}",
        sum_square - numbers_square_sum
    );
}
