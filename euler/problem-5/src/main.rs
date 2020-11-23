fn is_divisible(n: u64) -> bool {
    (1..20).all(|divider| n % divider == 0)
}

fn main() {
    let mut i = 1;
    let answer = loop {
        if is_divisible(i) {
            break i;
        }

        i += 1;
    };
    println!("The answer to the fifth problem is {}", answer);
}
