pub fn carmichael_numbers(n: usize) -> bool {
    if is_prime(n) {
        false
    } else {
        (1..n).all(|x| {
            let x_n = mod_pow(x, n, n);
            x_n == x
        })
    }
}

fn mod_pow(x: usize, n: usize, m: usize) -> usize {
    let mut x = x;
    let mut n = n;
    let mut ret = 1;
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % m;
        }
        x = x * x % m;
        n >>= 1;
    }
    ret
}

fn is_prime(n: usize) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(false, carmichael_numbers(17));
    }

    #[test]
    fn test_2() {
        assert_eq!(true, carmichael_numbers(561));
    }

    #[test]
    fn test_3() {
        assert_eq!(false, carmichael_numbers(4));
    }
}
