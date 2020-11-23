fn fib_sum_of_even(max: u128) -> u128 {
    let mut start = 0;
    let mut result = 0;

    let (mut first, mut second): (u128, u128) = (0, 1);

    loop {
        if max <= first {
            break;
        }

        let next = first + second;
        first = second;
        second = next;
        start += 1;

        if first % 2 == 0 {
            result += first;
        }
    }
    result
}

fn main() {
    println!(
        "The answer to the second problem is: {}",
        fib_sum_of_even(4_000_000)
    );
}
