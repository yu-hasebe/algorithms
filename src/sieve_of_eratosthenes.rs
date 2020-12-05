pub fn sieve_of_eratosthenes(n: usize) -> usize {
    let mut sieve = vec![true; n + 1];
    sieve[0] = false;
    sieve[1] = false;

    let mut ans = 0;
    for i in 2..=n {
        if sieve[i] {
            ans += 1;
        }
        let mut j = 2;
        while i * j <= n {
            sieve[i * j] = false;
            j += 1;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(5, sieve_of_eratosthenes(11));
    }

    #[test]
    fn test_2() {
        assert_eq!(78_498, sieve_of_eratosthenes(1_000_000));
    }
}
