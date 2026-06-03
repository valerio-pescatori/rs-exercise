/// Write a function, which takes a non-negative integer (seconds) as input and returns the time in a human-readable format (`HH:MM:SS`).
///
/// * `HH` = hours, padded to 2 digits, range: `00 - 99`
/// * `MM` = minutes, padded to 2 digits, range: `00 - 59`
/// * `SS` = seconds, padded to 2 digits, range: `00 - 59`
///
/// The maximum time never exceeds `359999` (`99:59:59`).
///
/// You can find some examples in the test fixtures.
pub fn make_readable(seconds: u32) -> String {
    let one_second = 1;
    let one_minute = one_second * 60;
    let one_hour = one_minute * 60;

    let (hours, rem_minutes) = div_rem(seconds, one_hour); // take the remainder and divide it by minutes
    let (minutes, rem_seconds) = div_rem(rem_minutes, one_minute); // the remainder are seconds

    format!("{hours:0>2}:{minutes:0>2}:{rem_seconds:0>2}")
}

fn div_rem(x: u32, y: u32) -> (u32, u32) {
    let div = x / y;
    let rem = x % y;
    (div, rem)
}
