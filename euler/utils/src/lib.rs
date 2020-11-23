pub fn is_prime(n: u64) -> bool {
    if n <= 3 {
        n > 1
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6
        }
        return true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_checks_primes() {
        // primes
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(13));
        assert!(is_prime(17));
        assert!(is_prime(19));
        assert!(is_prime(23));
        assert!(is_prime(29));
        assert!(is_prime(31));
        // not primes
        assert!(!is_prime(1));
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(8));
        assert!(!is_prime(9));
        assert!(!is_prime(10));
        assert!(!is_prime(12));
        assert!(!is_prime(14));
        assert!(!is_prime(15));
        assert!(!is_prime(18));
        assert!(!is_prime(16));
    }
}
