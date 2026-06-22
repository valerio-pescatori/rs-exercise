/// Your task in order to complete this Kata is to write a function which formats a duration, given as a number of seconds, in a human-friendly way.
///
/// The function must accept a non-negative integer. If it is zero, it just returns "now". Otherwise,
/// the duration is expressed as a combination of years, days, hours, minutes and seconds.
///
/// It is much easier to understand with an example:
///
/// * For seconds = 62, your function should return
///   "1 minute and 2 seconds"
/// * For seconds = 3662, your function should return
///   "1 hour, 1 minute and 2 seconds"
///
/// For the purpose of this Kata, a year is 365 days and a day is 24 hours.
///
/// Note that spaces are important.
///
/// Detailed rules
/// The resulting expression is made of components like 4 seconds, 1 year, etc. In general,
/// a positive integer and one of the valid units of time, separated by a space. The unit of time is used in plural if the integer is greater than 1.
///
/// The components are separated by a comma and a space (", "). Except the last component,
/// which is separated by " and ", just like it would be written in English.
///
/// A more significant units of time will occur before than a least significant one.
/// Therefore, 1 second and 1 year is not correct, but 1 year and 1 second is.
///
/// Different components have different unit of times.
/// So there is not repeated units like in 5 seconds and 1 second.
///
/// A component will not appear at all if its value happens to be zero.
/// Hence, 1 minute and 0 seconds is not valid, but it should be just 1 minute.
///
/// A unit of time must be used "as much as possible". It means that the function should not return 61 seconds,
/// but 1 minute and 1 second instead. Formally, the duration specified by of a component must not be greater than any valid more significant unit of time.
pub fn format_duration(seconds: u64) -> String {
    if seconds == 0 {
        return String::from("now");
    }

    let one_second = 1_u64;
    let one_minute = one_second * 60;
    let one_hour = one_minute * 60;
    let one_day = one_hour * 24;
    let one_year = one_day * 365;
    let mut parts: Vec<String> = vec![];

    let (years, rest) = div_rem(seconds, one_year);

    if years > 0 {
        let year_part = format!("{years} year{}", get_plural(years));
        parts.push(year_part);
    }

    let (days, rest) = div_rem(rest, one_day);

    if days > 0 {
        let days_part = format!("{days} day{}", get_plural(days));
        parts.push(days_part);
    }

    let (hours, rest) = div_rem(rest, one_hour);

    if hours > 0 {
        let hours_part = format!("{hours} hour{}", get_plural(hours));
        parts.push(hours_part);
    }

    let (minutes, rest) = div_rem(rest, one_minute);

    if minutes > 0 {
        let minutes_part = format!("{minutes} minute{}", get_plural(minutes));
        parts.push(minutes_part);
    }

    let (seconds, _rest) = div_rem(rest, one_second);

    if seconds > 0 {
        let seconds_part = format!("{seconds} second{}", get_plural(seconds));
        parts.push(seconds_part);
    }

    if parts.len() == 1 {
       return parts.join("");
    }

    let final_part = parts.split_off(parts.len() - 1);

    let mut res: String = parts.join(", ");

    if !res.is_empty() {
        res.push_str(" and ");
        res.push_str(&final_part[0]);
    }

    res
}

fn get_plural(n: u64) -> String {
    if n > 1 {
        String::from("s")
    } else {
        String::from("")
    }
}

fn div_rem(n: u64, d: u64) -> (u64, u64) {
    (n / d, n % d)
}
