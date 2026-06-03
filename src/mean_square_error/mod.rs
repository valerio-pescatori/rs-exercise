/// Complete the function that
///
/// * accepts two integer arrays of equal length
/// * compares the value each member in one array to the corresponding member in the other
/// * squares the absolute value difference between those two values
/// * and returns the average of those squared absolute value difference between each member pair.
///
/// ## Examples
///
/// ```text
///     [1, 2, 3], [4, 5, 6]              -->   9   because (9 + 9 + 9) / 3
///     [10, 20, 10, 2], [10, 25, 5, -2]  -->  16.5 because (0 + 25 + 25 + 16) / 4
///     [-1, 0], [0, -1]                  -->   1   because (1 + 1) / 2
/// ```
///
pub fn solution(array_a: &[i64], array_b: &[i64]) -> f64 {
    let mut sum = 0_i64;

    array_a.iter().enumerate().for_each(|(i_a, a)| {
        let diff = array_b[i_a] - a;
        sum += diff.pow(2);
    });

    sum as f64 / array_a.len() as f64
}
