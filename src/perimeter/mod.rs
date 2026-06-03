use std::collections::HashMap;
/// The drawing shows 6 squares whose side lengths are `1, 1, 2, 3, 5, 8`. It is easy to see that the sum of their perimeters is:
///
/// ```text
/// 4 * (1 + 1 + 2 + 3 + 5 + 8) = 4 * 20 = 80
/// ```
///
/// Could you give the sum of the perimeters of all the squares in a rectangle when there are `n + 1` squares arranged in the same manner as in the drawing?
///
/// **Hint:**
/// See the Fibonacci sequence.
///
/// The function `perimeter` has a parameter `n`, where `n + 1` is the number of squares (they are numbered from `0` to `n`), and returns the total perimeter of all the squares.
///
/// ## Examples
///
/// ```text
/// perimeter(5) --> 80
/// perimeter(7) --> 216
/// ```
pub fn perimeter(n: u64) -> u64 {
    let mut map = HashMap::from([(0, 0), (1, 1), (2, 1)]);
    let range = 0..=n + 1;
    // sum of all fib from n to 1 * 4
    let sum = range.fold(0_u64,|acc, n| acc + fib(n, &mut map));

    sum * 4
}

fn fib(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    // start with 1 and 2
    // if in map, return
    // if not, calc and save in map, return
    let res = map.get(&n);

    match res {
        None => {
            // calc n-1
            let n1 = fib(n - 1, map);
            // calc n-2
            let n2 = fib(n - 2, map);
            let res = n1 + n2;
            // save res
            map.insert(n, res);
            // return
            res
        }
        Some(v) => *v,
    }
}
