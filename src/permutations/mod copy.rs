use std::{collections::HashMap, fmt::Display};

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
    // build a tree in which each node is a char of the sting
    // and has a child for each other letter in the remaining string.
    // This will build a tree containing all the possible strings going from the root node to each leaf.
    let mut root_node = Node {
        char: ' ',
        children: vec![],
    };

    build_subtree(s, );

    // tree is build, now explore the tree (DFS) and build strings

    // later: make it all in one go, build the strings instead of building the tree
    // and add them to the vector
    let mut res: Vec<String> = vec![];

    build_str(root_node, &mut res, None);

    res
}

fn build_subtree(rest: &str, vec: Vec<String>, set: &mut HashSet<&str>) {
    // while recursing we'll keep an array of all the possible permutations so far `vec`, a string representing the current permutation `acc`
    // the remaining part of the initial string `rest`, and a set of already calculated subtrees (identified by the remaining part of the string) `set`
    //
    // when entering a recursion subtree, we'll verify if we have already visited that subtree (`set` contains `rest`)
    // if so, return an empty vector (the possible permutation starting from `rest` are already included in `vec`)
    // otherwise, remove one char from `rest`, add it to `acc` and go on with the recursion, merging the returned value of the recursive call with `vec`.
    // return vec

    if map.contains_key(rest) {
        // do not compute subtree
        return;
    }

    if rest.is_empty() {
        // end of string, save
    }

    for (index, c) in rest.char_indices() {
        // build a node, attach to children
        // recurse with substr - c
        let mut child_node = Node {
            char: c,
            children: Vec::new(),
        };
        let substr = remove_at(rest, index);
        build_subtree(&substr, &mut child_node, map);
        node.children.push(child_node);
    }
}

// old attempt, works without the map, but it's inefficient
// fn build_subtree(rest: &str, node: &mut Node, map: &mut HashMap<&str, Node>) {
//     if map.contains_key(rest) {
//         // do not compute subtree
//         return;
//     }

//     if rest.is_empty() {
//         // end of string, save
//     }

//     for (index, c) in rest.char_indices() {
//         // build a node, attach to children
//         // recurse with substr - c
//         let mut child_node = Node {
//             char: c,
//             children: Vec::new(),
//         };
//         let substr = remove_at(rest, index);
//         build_subtree(&substr, &mut child_node, map);
//         node.children.push(child_node);
//     }
// }

fn build_str(node: Node, vec: &mut Vec<String>, acc: Option<String>) {
    let mut curr = acc.unwrap_or_default();
    curr.push(node.char);

    if node.children.is_empty() {
        vec.push(curr.trim().to_owned());
        return;
    }

    for child in node.children {
        build_str(child, vec, Some(curr.clone()));
    }
}

fn remove_at(s: &str, at: usize) -> String {
    s.char_indices()
        .filter(|(i, _)| *i != at)
        .map(|(_, c)| c)
        .collect::<String>()
}

#[derive(Debug)]
struct Node {
    char: char,
    children: Vec<Node>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node: {}\nN. of children: {}\nChildren: {:#?}",
            self.char,
            self.children.len(),
            self.children
        )
    }
}
