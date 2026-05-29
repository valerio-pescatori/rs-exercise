use std::collections::HashMap;

/// Write a function that takes a string input, and returns the first character that is not repeated anywhere in the string.
///
/// For example, if given the input "stress", the function should return 't',
/// since the letter t only occurs once in the string, and occurs first in the string.
///
/// As an added challenge, upper- and lowercase characters are considered the same character,
/// but the function should return the correct case for the initial character.
/// For example, the input "sTreSS" should return "T".
///
/// If a string contains only repeating characters, return None.
///
/// Note: despite its name in some languages, your function should handle any Unicode codepoint:
/// ```
///     "@#@@*"    --> "#"
///     "かか何"   --> "何"
///     "🐐🦊🐐" --> "🦊"
/// ```
pub fn first_non_repeating(s: &str) -> Option<char> {
    
    let chars: Vec<(usize, char)> = s.chars().enumerate().collect::<Vec<_>>();
    let mut map: HashMap<char, Count> = HashMap::new();

    // build a map with char -> occurrence using the first occurrence in the string for char (uppercase/lowercase)
    // then return the first key that has 1 occurrence and lowest index in the string

    chars.iter().for_each(|(idx, c)| {
        // check if map has c case insensitive
        let low_c = c.to_lowercase().next().unwrap();
        let lookup = map.get(&low_c);

        match lookup {
            Some(v) => map.insert(
                low_c,
                Count {
                    idx: v.idx,
                    count: v.count + 1,
                    is_lowercase: v.is_lowercase,
                },
            ),
            None => map.insert(
                low_c,
                Count {
                    idx: *idx,
                    count: 1,
                    is_lowercase: c.is_lowercase(),
                },
            ),
        };
    });

    // the map is built, iter over entries, filter element with 1 occurrence
    let once = map.iter().filter(|(_, v)| v.count == 1).collect::<Vec<_>>();

    match once.len() {
        0 => None,
        _ => {
            // we are sure there is at least one element with 1 occurrence
            let (c, count) = once.iter().min_by(|a, b| a.1.idx.cmp(&b.1.idx)).unwrap();

            let result: char = if count.is_lowercase {
                c.to_lowercase().next().unwrap()
            } else {
                c.to_uppercase().next().unwrap()
            };

            Some(result)
        }
    }
}

struct Count {
    /// position in the string
    idx: usize,
    /// number of occurrences
    count: usize,
    /// is lowercase
    is_lowercase: bool,
}
