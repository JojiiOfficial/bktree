mod distance;

use std::{collections::VecDeque, iter::FromIterator, ops::Sub};

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Node<T> {
    word: T,
    children: Vec<(usize, Node<T>)>,
}

/// A BK-tree datastructure
///
#[derive(Serialize, Deserialize)]
pub struct BkTree<T: AsRef<str>> {
    root: Option<Box<Node<T>>>,
}

impl<T: AsRef<str>> BkTree<T> {
    /// Create a new BK-tree with a given distance function
    #[inline]
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Insert every element from a given iterator in the BK-tree
    pub fn insert_all<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.insert(i);
        }
    }

    /// Insert a new element in the BK-tree
    pub fn insert(&mut self, val: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(Node {
                    word: val,
                    children: Vec::new(),
                }))
            }
            Some(ref mut root_node) => {
                let mut u = &mut **root_node;
                loop {
                    let k = distance::levenshtein_distance(&u.word, &val);
                    if k == 0 {
                        return;
                    }

                    let v = u.children.iter().position(|(dist, _)| *dist == k);
                    match v {
                        None => {
                            u.children.push((
                                k,
                                Node {
                                    word: val,
                                    children: Vec::new(),
                                },
                            ));
                            return;
                        }
                        Some(pos) => {
                            let (_, ref mut vnode) = u.children[pos];
                            u = vnode;
                        }
                    }
                }
            }
        }
    }
    /// Find the closest elements to a given value present in the BK-tree
    /// Returns pairs of element references and distances
    pub fn find(&self, val: &T, max_dist: usize) -> Vec<(&T, usize)> {
        if self.root.is_none() {
            return vec![];
        }

        let mut found = Vec::with_capacity(5);

        let mut candidates: VecDeque<&Node<T>> = VecDeque::with_capacity(511);
        candidates.push_back(self.root.as_ref().unwrap());

        while let Some(n) = candidates.pop_front() {
            let distance = distance::levenshtein_distance(&n.word, &val);
            if distance <= max_dist {
                found.push((&n.word, distance));
            }

            candidates.extend(n.children.iter().filter_map(|(arc, node)| {
                (abs_difference(*arc, distance) <= max_dist).then(|| node)
            }));
        }

        found
    }

    /// Convert the BK-tree into an iterator over its elements, in no particular order
    #[inline]
    pub fn into_iter(self) -> IntoIter<T> {
        let mut queue = Vec::with_capacity(1);
        if let Some(root) = self.root {
            queue.push(*root);
        }
        IntoIter { queue }
    }

    /// Create an iterator over references of BK-tree elements, in no particular order
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        let mut queue = Vec::with_capacity(1);
        if let Some(ref root) = self.root {
            queue.push(&**root);
        }
        Iter { queue }
    }
}

impl<T: AsRef<str>> FromIterator<T> for BkTree<T> {
    #[inline]
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        let mut bk = BkTree::new();
        bk.insert_all(iter);
        bk
    }
}

#[inline]
fn abs_difference<T: Sub<Output = T> + Ord>(x: T, y: T) -> T {
    if x < y {
        y - x
    } else {
        x - y
    }
}

impl<T: AsRef<str>> IntoIterator for BkTree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

/// Iterator over BK-tree elements
pub struct IntoIter<T: AsRef<str>> {
    queue: Vec<Node<T>>,
}

impl<T: AsRef<str>> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop().map(|node| {
            self.queue.extend(node.children.into_iter().map(|(_, n)| n));
            node.word
        })
    }
}

/// Iterator over BK-tree elements, by reference
pub struct Iter<'a, T: AsRef<str>> {
    queue: Vec<&'a Node<T>>,
}

impl<'a, T: AsRef<str>> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop().map(|node| {
            self.queue.extend(node.children.iter().map(|(_, n)| n));
            &node.word
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::BkTree;
    #[test]
    fn levenshtein_distance_test() {
        let mut bk = BkTree::new();
        bk.insert_all(vec![
            "book", "books", "boo", "boon", "cook", "cake", "cape", "cart",
        ]);
        let (words, dists): (Vec<&str>, Vec<isize>) = bk.find("bo", 2).into_iter().unzip();
        assert_eq!(words, ["book", "boo", "boon"]);
        assert_eq!(dists, [2, 1, 2]);
    }
}
