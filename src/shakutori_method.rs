fn shakutori_method(n: usize, s: i64, a: Vec<i64>) -> usize {
    let (mut l, mut r): (usize, usize) = (0, 0);

    let mut ans = n + 1;
    let mut sum = 0;
    loop {
        while r < n && sum < s {
            sum += a[r];
            r += 1;
        }
        if sum < s {
            break;
        }
        ans = std::cmp::min(ans, r - l);
        sum -= a[l];
        l += 1;
    }
    println!("{}", ans);
    if ans == n - 1 {
        0
    } else {
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            2,
            shakutori_method(10, 15, vec![5, 1, 3, 5, 10, 7, 4, 9, 2, 8])
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(3, shakutori_method(5, 11, vec![1, 2, 3, 4, 5]));
    }
}
