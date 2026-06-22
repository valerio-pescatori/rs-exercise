use std::{collections::HashSet};

/// In this kata, your task is to create all permutations of a non-empty input string and remove duplicates, if present.
///
/// Create as many "shufflings" as you can!
///
/// Examples:
///
/// With input 'a':
/// Your function should return: ['a']
///
/// With input 'ab':
/// Your function should return ['ab', 'ba']
///
/// With input 'abc':
/// Your function should return ['abc','acb','bac','bca','cab','cba']
///
/// With input 'aabb':
/// Your function should return ['aabb', 'abab', 'abba', 'baab', 'baba', 'bbaa']
/// Note: The order of the permutations doesn't matter.
pub fn permutations(s: &str) -> Vec<String> {
    let mut vec: Vec<String> = vec![];

    build_subtree(s, String::new(), &mut vec, &mut HashSet::new());

    vec
}

fn build_subtree(
    rest: &str,
    acc: String,
    vec: &mut Vec<String>,
    set: &mut HashSet<(String, String)>,
) -> Option<String> {
    // while recursing we'll keep an array of all the possible permutations so far `vec`, a string representing the current permutation `acc`
    // the remaining part of the initial string `rest`, and a set of already calculated subtrees (identified by the remaining part of the string) `set`
    //
    // when entering a recursion subtree, we'll verify if we have already visited that subtree (`set` contains `rest`)
    // if so, return an empty vector (the possible permutation starting from `rest` are already included in `vec`)
    // otherwise, remove one char from `rest`, add it to `acc` and go on with the recursion, adding the returned value of the recursive call into `vec`.
    // return vec

    if set.contains(&(acc.clone(), rest.to_string())) {
        // do not compute subtree
        return None;
    }

    if rest.is_empty() {
        return Some(acc);
    }

    for (index, c) in rest.char_indices() {
        // build next `rest`
        let substr = remove_at(rest, index);
        // add `c` to `acc`
        let mut new_acc = acc.clone();
        new_acc.push(c);

        let res = build_subtree(&substr, new_acc, vec, set);

        if let Some(v) = res {
            vec.push(v)
        };
    };

    // mark substr as visited
    set.insert((acc.clone(), rest.to_string()));

    None
}

fn remove_at(s: &str, at: usize) -> String {
    s.char_indices()
        .filter(|(i, _)| *i != at)
        .map(|(_, c)| c)
        .collect::<String>()
}
