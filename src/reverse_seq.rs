
pub fn reverse_seq(n: u32) -> Vec<u32> {
    (0..n).map(|i|  n-i).collect()
}