fn into_digits(mut n: u64) -> Vec<u64> {
    let mut digits: Vec<u64> = vec![];

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    digits
}

fn is_palindrome(n: u64) -> bool {
    let digits = into_digits(n);
    digits.iter().eq(digits.iter().rev())
}

fn biggest_palindrom() -> u64 {
    let mut biggest = 0;
    for x in (100..999).rev() {
        for y in (100..999).rev() {
            let n = x * y;
            if is_palindrome(n) && n > biggest {
                biggest = n;
            }
        }
    }
    biggest
}

fn main() {
    println!(
        "The answer to the fourth problem is {}",
        biggest_palindrom()
    );
}
