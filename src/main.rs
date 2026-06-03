mod mean_square_error;
use mean_square_error::solution;

fn main() {
    // [1, 2, 3], [4, 5, 6]
    println!(
        "first_non_repeating: {:?}",
        solution(&[1, 2, 3], &[4, 5, 6])
    );
}
