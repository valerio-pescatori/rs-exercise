use std::char::from_digit;
use std::cmp::max;
use std::cmp::min;

/// Implement a function which receives two arguments:
///
/// the sum of digits (sum)
/// the number of digits (count)
/// This function should return three values:
///
/// the total number of values which have count digits that add up to sum and are in increasing order
/// the lowest such value
/// the greatest such value
/// Note: if there are no values which satisfy these constaints, you should return an empty value (refer to the examples to see what exactly is expected).
///
/// Examples:
///
/// ```
///     find_all(10, 3)  =>  [8, 118, 334]
///     find_all(27, 3)  =>  [1, 999, 999]
///     find_all(84, 4)  =>  []
/// ```
pub fn find_all(sum_dig: u8, digs: u8) -> Option<(usize, u64, u64)> {
    let mut res: Option<(usize, u64, u64)> = None;
    recurse(sum_dig, digs, 0, &mut res);
    res
}

/// Recursive fn
///     
/// - `sum_dig` - goal sum
/// - `rem_digs` - how many digits are left
/// - `so_far` - the current number
fn recurse(
    sum_dig: u8,
    rem_digs: u8,
    so_far: u64,
    res: &mut Option<(usize, u64, u64)>,
) -> Option<u64> {
    // happy path - rem_digs is 0 and so_far sums up to sum_dig
    // we return the value (how do we keep track of min/max and count?)
    let sum = calc_sum(so_far);
    if rem_digs == 0 && sum == sum_dig.into() {
        // happy
        return Some(so_far);
    }

    // check if so_far is still valid
    // it is not valid if
    // - the sum of the digits is > sum_dig
    // - rem_digs = 0 && so_far != sum_dig
    if sum > sum_dig as u64 || (rem_digs == 0 && sum != sum_dig as u64) {
        return None;
    }

    // otherwise try all possible solutions: for each digit that is >= than the last char of so_far
    // call recurse by adding that digit to so_far
    let last_digit = so_far
        .to_string()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let digits: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    let possbile_digits = digits
        .iter()
        .filter(|d| **d >= last_digit)
        .collect::<Vec<_>>();

    for digit in possbile_digits {
        // new so_far value
        let new_so_far = append_digit(so_far, *digit);
        let value = recurse(sum_dig, rem_digs - 1, new_so_far, res);

        // if this branch ends up in a successful leaf
        if let Some(v) = value {
            // check min/max, add count
            match res {
                None => {
                    *res = Some((1, v, v));
                }
                Some((count, min_v, max_v)) => {
                    *res = Some((*count + 1, min(v, *min_v), max(v, *max_v)));
                }
            }
        }
    }
    // non-leaf nodes always return None
    None
}

fn append_digit(n: u64, digit: u32) -> u64 {
    let mut s = if n != 0 {
        n.to_string()
    } else {
        String::new()
    };
    s.push(from_digit(digit, 10).unwrap());
    s.parse().unwrap()
}

fn calc_sum(digits: u64) -> u64 {
    digits
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum()
}
