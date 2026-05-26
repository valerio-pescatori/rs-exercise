pub fn find_average(slice: &[f64]) -> f64 {
    let sum: f64 = slice.iter().sum();
    let avg = sum / (slice.len() as f64);
    if f64::is_nan(avg) { 0_f64 } else { avg }
}