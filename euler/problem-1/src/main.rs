fn is_multiply_of_three(num: u32) -> bool {
    num % 3 == 0
}

fn is_multiply_of_five(num: u32) -> bool {
    num % 5 == 0
}

fn sum_of_multiplies_in_range(range: std::ops::Range<u32>) -> u32 {
    range.fold(0, |sum, num| {
        if is_multiply_of_five(num) || is_multiply_of_three(num) {
            sum + num
        } else {
            sum
        }
    })
}

fn main() {
    println!(
        "The answer to the first problem is: {}",
        sum_of_multiplies_in_range(0..1000)
    );
}
