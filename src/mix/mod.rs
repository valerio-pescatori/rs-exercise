use std::{
    char,
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashMap,
};

#[derive(Clone)]
struct Count {
    count: usize,
    from: String,
}

/// Given two strings `s1` and `s2`, we want to visualize how different the two strings are. We will only take into account the lowercase letters (`a` to `z`). First let us count the frequency of each lowercase letter in `s1` and `s2`.
///
/// ```text id="vw8f2m"
///     s1 = "A aaaa bb c"
///     s2 = "& aaa bbb c d"
/// ```
///
/// ```text id="p3z1qk"
///     s1 has 4 'a', 2 'b', 1 'c'
///     s2 has 3 'a', 3 'b', 1 'c', 1 'd'
/// ```
///
/// So the maximum for `'a'` in `s1` and `s2` is `4` from `s1`; the maximum for `'b'` is `3` from `s2`. In the following we will not consider letters when the maximum of their occurrences is less than or equal to `1`.
///
/// We can summarize the differences between `s1` and `s2` in the following string:
///
/// ```text id="c6n9rt"
///     "1:aaaa/2:bbb"
/// ```
///
/// where `1` in `1:aaaa` stands for string `s1` and `aaaa` because the maximum for `a` is `4`. In the same manner `2:bbb` stands for string `s2` and `bbb` because the maximum for `b` is `3`.
///
/// The task is to produce a string in which each lowercase letter of `s1` or `s2` appears as many times as its maximum if this maximum is strictly greater than `1`; these letters will be prefixed by the number of the string where they appear with their maximum value and `:`. If the maximum is in `s1` as well as in `s2`, the prefix is `=:`.
///
/// In the result, substrings (for example `2:nnnnn` or `1:hhh`; a substring contains the prefix) will be in decreasing order of their length and, when they have the same length, sorted in ascending lexicographic order (letters and digits, more precisely sorted by codepoint). The different groups will be separated by `/`.
///
/// ## Examples
///
/// ```text id="t9a2ly"
///     s1 = "my&friend&Paul has heavy hats! &"
///     s2 = "my friend John has many many friends &"
///
///     mix(s1, s2) // --> "2:nnnnn/1:aaaa/1:hhh/2:mmm/2:yyy/2:dd/2:ff/2:ii/2:rr/=:ee/=:ss"
/// ```
///
/// ```text id="h4u7xb"
///     s1 = "mmmmm m nnnnn y&friend&Paul has heavy hats! &"
///     s2 = "my frie n d Joh n has ma n y ma n y frie n ds n&"
///
///     mix(s1, s2) // --> "1:mmmmmm/=:nnnnnn/1:aaaa/1:hhh/2:yyy/2:dd/2:ff/2:ii/2:rr/=:ee/=:ss"
/// ```
///
/// ```text id="r2m8dw"
///     s1 = "Are the kids at home? aaaaa fffff"
///     s2 = "Yes they are here! aaaaa fffff"
///
///     mix(s1, s2) // --> "=:aaaaaa/2:eeeee/=:fffff/1:tt/2:rr/=:hh"
/// ```
pub fn mix(s1: &str, s2: &str) -> String {
    let mut res = String::new();

    // only consider chars 'a' to 'z'
    let trim = |s: &str| {
        s.chars()
            .filter(char::is_ascii_lowercase)
            .collect::<String>()
    };

    let s1_trim = trim(s1);
    let s2_trim = trim(s2);

    // build the map
    // each entry has the character as key and infos on occurrences and source string as value
    let map = map_values(s1_trim, s2_trim);

    // filter out entries that appears only once
    let mut as_vec = map
        .into_iter()
        .filter(|(_, count)| count.count > 1)
        .collect::<Vec<_>>();

    // sort by occurrence descending, (or by source string if occurrence is the same)
    as_vec.sort_by(|(char_a, count_a), (char_b, count_b)| {
        let ord = count_b.count.cmp(&count_a.count);

        match ord {
            Less => ord,
            Greater => ord,
            Equal => {
                // compare the from strings
                let combined_a = format!("{}{}", count_a.from, char_a);
                let combined_b = format!("{}{}", count_b.from, char_b);
                combined_a.cmp(&combined_b)
            }
        }
    });

    // then paste the letter as many times as it appears in the relative string (s1 or s2)

    // for each key compare the value with the corresponding value in the other map, build the substring for that char
    as_vec.iter().for_each(|(char, count)| {
        res.push_str(&count.from);

        res.push_str(&char.to_string().repeat(count.count));
        res.push('/');
    });

    if res.len() > 1 {
        res.remove(res.len() - 1);
    }

    res
}

fn map_values(s1: String, s2: String) -> HashMap<char, Count> {
    let mut s1_map: HashMap<char, Count> = HashMap::new();
    let mut s2_map: HashMap<char, Count> = HashMap::new();

    let write_map = |s: &String, m: &mut HashMap<char, Count>, from: String| {
        s.chars().for_each(|c: char| {
            match m.get(&c) {
                None => m.insert(
                    c,
                    Count {
                        count: 1,
                        from: from.to_owned(),
                    },
                ),
                Some(v) => m.insert(
                    c,
                    Count {
                        count: v.count + 1,
                        from: from.to_owned(),
                    },
                ),
            };
        });
    };

    // iter over s1, write values to map
    write_map(&s1, &mut s1_map, String::from("1:"));
    // iter over s2, write values to map
    write_map(&s2, &mut s2_map, String::from("2:"));

    // merge maps and return
    // for each entry in s2, add it to s1, handle conflict or insert
    for (k, v2) in s2_map {
        s1_map
            .entry(k)
            .and_modify(|v1| {
                // compare v_1.count with v2_.count
                let cmp = v1.count.cmp(&v2.count);
                match cmp {
                    Less => *v1 = v2.clone(),
                    Greater => (),
                    Equal => {
                        *v1 = Count {
                            count: v1.count,
                            from: String::from("=:"),
                        }
                    }
                }
            })
            .or_insert(v2);
    }
    s1_map
}
