use std::{
    cmp::Ordering::{Equal, Greater, Less},
    collections::HashSet,
};

/// A group of N golfers wants to play in groups of G players for D days
/// in such a way that no golfer plays more than once with any other golfer.
///
/// For example, for N=20, G=4, D=5, the solution at Wolfram MathWorld is
/// ```
///  Mon:    ABCD    EFGH    IJKL    MNOP    QRST
///  Tue:    AEIM    BJOQ    CHNT    DGLS    FKPR
///  Wed:    AGKO    BIPT    CFMS    DHJR    ELNQ
///  Thu:    AHLP    BKNS    CEOR    DFIQ    GJMT
///  Fri:    AFJN    BLMR    CGPQ    DEKT    HIOS
/// ```
///
/// Write a function that validates a proposed solution, a list of list of strings,
/// as being a solution to the social golfer problem.
/// Each character represents a golfer, and each string is a group of players. Rows represent days.
///
/// The solution above would be encoded as:
/// ```
///  [
///   ['ABCD', 'EFGH', 'IJKL', 'MNOP', 'QRST'],
///   ['AEIM', 'BJOQ', 'CHNT', 'DGLS', 'FKPR'],
///   ['AGKO', 'BIPT', 'CFMS', 'DHJR', 'ELNQ'],
///   ['AHLP', 'BKNS', 'CEOR', 'DFIQ', 'GJMT'],
///   ['AFJN', 'BLMR', 'CGPQ', 'DEKT', 'HIOS']
///  ]
/// ```
///
/// You need to make sure:
///  1. that each golfer plays exactly once every day,
///  2. that the number and size of the groups is the same every day
///  3. that each player plays with every other player at most once.
///
/// So although each player must play every day, there can be particular pairs of players that never play together.
///
/// It is not necessary to consider the case where the number of golfers is zero; no tests will check for that.
/// If you do wish to consider that case, note that you should accept as valid all possible solutions for zero golfers,
/// who (vacuously) can indeed play in an unlimited number of groups of zero.
pub fn valid(a: Vec<Vec<&str>>) -> bool {
    // iter through days and matches
    // maintain:
    // - a set of `char`s to keep track of who played (resets every day)
    // - a second set of `char`s to keep track of who played yesterday (to detect unknown players)
    // - a bool to check if the size of groups changes through the day (resets every day)
    // - a set of pairs to check if a re-match is happening (never resets)
    let mut who_played_today: HashSet<char> = HashSet::new();
    let mut who_played_yesterday: HashSet<char> = HashSet::new();
    let mut pairs_today: HashSet<(char, char)> = HashSet::new();
    let group_n = a[0].len();
    let group_size = a[0][0].len();

    for day in a {
        let this_group_n = day.len();
        who_played_today.clear();

        for this_match in day {
            let this_group_size = this_match.len();
            // if group size changes -> invalid
            if this_group_size != group_size || this_group_n != group_n {
                return false;
            }
            // we don't need to update the group size values, if they change we'll detect anyway

            // for each player, check if the player has already played today
            for player in this_match.chars() {
                // if it was already there -> return false as he played twice today
                if !who_played_today.insert(player) {
                    return false;
                }
            }

            // for each pair of players, check if this pair already played today
            let pairs = gen_pairs(this_match);

            for pair in pairs {
                // if it was already there -> return false as this pair played twice today
                if !pairs_today.insert(pair) {
                    return false;
                }
            }
        }

        // we should keep track of known players
        // this will be done by memoizing a copy of `who_played_yesterday`
        // and check that it has the same members of `who_played_today` at the end of each day
        let is_empty = who_played_yesterday.is_empty();
        if !is_empty
            && who_played_today
                .symmetric_difference(&who_played_yesterday)
                .next()
                .is_some()
        {
            return false;
        }
        
        if is_empty {
            who_played_yesterday = who_played_today.clone();
        }
    }

    true
}

fn gen_pairs(str: &str) -> Vec<(char, char)> {
    let mut pairs = vec![];
    for (p1, c1) in str.char_indices() {
        for (p2, c2) in str.char_indices() {
            // avoid double pairs
            if p2 <= p1 {
                continue;
            }
            // sort pair before pushing
            let sorted_pair = match c1.cmp(&c2) {
                Greater => (c1, c2),
                Less => (c2, c1),
                Equal => unreachable!(),
            };

            pairs.push(sorted_pair);
        }
    }
    pairs
}
