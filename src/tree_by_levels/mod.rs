use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    value: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: u32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn left(mut self, left: Self) -> Self {
        self.left = Some(Box::new(left));
        self
    }

    pub fn right(mut self, right: Self) -> Self {
        self.right = Some(Box::new(right));
        self
    }
}

/// Your task is to return the list with elements from tree sorted by levels, which means the root element goes first,
/// then root children (from left to right) are second and third, and so on.
///
/// Inputs will always contain at least one node.
///
/// Example 1 - following tree:
/// ```
///                   1
///             2              3
///           4     5       6      7
///         8  9  10 11   12 13  14 15
/// ```
/// Should return following list:
///
/// ```
///     [1,2,3,4,5,6,7,8,9,...]
/// ```
pub fn tree_by_levels(root: &Node) -> Vec<u32> {
    if root.left.is_none() && root.right.is_none() {
        return vec![root.value];
    };

    // build a map whose keys are levels
    // and value values are lists of vec with elements from that level
    // then iter throug entries and concat all vecs

    // initialize with self - level 0
    let mut map: HashMap<usize, Vec<u32>> = HashMap::from([(0, vec![root.value])]);

    // recurse call for level 1
    if let Some(l) = &root.left {
        recursive_tree(l, &mut map, 1);
    }
    if let Some(r) = &root.right {
        recursive_tree(r, &mut map, 1);
    }

    let mut res: Vec<u32> = vec![];
    let mut sorted = map.iter_mut().collect::<Vec<_>>();
    sorted.sort_by_key(|e| e.0);

    for (_,  vec) in sorted {
        res.append(vec);
    }

    res
}

pub fn recursive_tree(root: &Node, map: &mut HashMap<usize, Vec<u32>>, level: usize) {
    // add root.value to the vec associated with the current level

    map.entry(level).and_modify(|vec| {
        vec.push(root.value);
    }).or_insert(vec![root.value]);

    if let Some(l) = &root.left {
        recursive_tree(l, map, level + 1);
    }
    if let Some(r) = &root.right {
        recursive_tree(r, map, level + 1);
    }
}
