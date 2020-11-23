use utils::is_prime;

fn into_factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![];
    let mut factor: u64 = 2;

    while n > 1 {
        if n % factor == 0 {
            dbg!(factor);
            factors.push(factor);
            n /= factor;
        } else {
            factor += 1;
        }
    }
    factors
}

fn main() {
    let biggest_factor = into_factors(600851475143)
        .last()
        .map(|n| n.clone())
        .expect("This number should have at least one factor!");
    println!("The answer to the third problem is {}", biggest_factor);
}
