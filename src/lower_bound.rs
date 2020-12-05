fn lower_bound(n: usize, a: Vec<i64>, k: i64) -> usize {
    let mut low = 0;
    let mut high = n;
    // After the loop, low + 1 == high must be true.
    while low + 1 < high {
        let mid = low + (high - low) / 2;
        if ok(&a, k, mid) {
            high = mid;
        // The answer must be in (low, high].
        } else {
            low = mid;
            // The answer must be in (low, high].
        }
    }
    // After the loop, low + 1 == high is true and the answer is in (low, high].
    // Therefore, the answer is high.
    high
}

fn ok(a: &Vec<i64>, k: i64, m: usize) -> bool {
    k <= a[m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, lower_bound(5, vec![2, 3, 3, 5, 6], 3));
    }
}
