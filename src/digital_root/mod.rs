/// Digital root is the recursive sum of all the digits in a number.
/// Given n, take the sum of the digits of n. If that value has more than one digit, continue reducing in this way until a single-digit number is produced. The input will be a non-negative integer.
/// 
/// Examples
/// ```
///     16  -->  1 + 6 = 7
///    942  -->  9 + 4 + 2 = 15  -->  1 + 5 = 6
/// 132189  -->  1 + 3 + 2 + 1 + 8 + 9 = 24  -->  2 + 4 = 6
/// 493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2
/// ```
pub fn digital_root(n: i64) -> i64 {
    // convert to string, split by space, sum all numbers
    // check number of digits, repeat until it's 1
    let mut to_str = n.to_string();
    let mut new_res = n;
    while to_str.len() > 1 {
        new_res = to_str.chars().fold(0, |acc, e| acc + (e.to_digit(10).unwrap() as i64));
        to_str = new_res.to_string();
    }
    new_res
 }