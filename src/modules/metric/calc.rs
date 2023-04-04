pub fn median(numbers: Vec<i64>) -> f64 {
    let mut sorted = numbers;
    sorted.sort();

    let n = sorted.len();
    if n % 2 == 0 {
        let mid = n / 2;
        (sorted[mid - 1] as f64 + sorted[mid] as f64) / 2.0
    } else {
        let mid = (n - 1) / 2;
        sorted[mid] as f64
    }
}
