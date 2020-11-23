use utils::is_prime;

fn main() {
    let mut current_number = 1;
    let mut prime_count = 0;

    loop {
        if is_prime(current_number) {
            prime_count += 1;
        }

        if prime_count == 10_001 {
            println!("The answer to the seventh problem is {}", current_number);
            break;
        }

        current_number += 1;
    }
}
