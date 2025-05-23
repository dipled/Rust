use std::fmt::{self, Display, Formatter};

struct Node<T> {
    left: Option<Box<Node<T>>>,
    element: T,
    right: Option<Box<Node<T>>>,
}

pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> Tree<T> {
    
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn insert(&mut self, element: T) {
        fn insert_recursive<T: Ord>(node: &mut Option<Box<Node<T>>>, element: T) {
            match node {
                None => {
                    *node = Some(Box::new(Node { element, left: None, right: None }));
                }
                Some(ref mut x) => {
                    if element <= x.element {
                        insert_recursive(&mut x.left, element);
                    } else {
                        insert_recursive(&mut x.right, element);
                    }
                }
            }
        }
        insert_recursive(&mut self.root, element);
    }

    pub fn pop_max(&mut self) -> Option<T> {
        fn pop_max_recursive<T: Ord>(node: &mut Option<Box<Node<T>>>) -> Option<T> {
            if let Some(ref mut x) = node {
                if x.right.is_some() {
                    return pop_max_recursive(&mut x.right);
                }
                let mut boxed = node.take().unwrap();
                *node = boxed.left.take();
                return Some(boxed.element);
            }
            None
        }
        pop_max_recursive(&mut self.root)
    }

    pub fn remove(&mut self, key: &T) -> bool
    where
        T: Clone,
    {
        fn remove_recursive<T: Ord + Clone>(node: &mut Option<Box<Node<T>>>, key: &T) -> bool {
            if let Some(ref mut x) = node {
                if key < &x.element {
                    return remove_recursive(&mut x.left, key);
                } else if key > &x.element {
                    return remove_recursive(&mut x.right, key);
                } else {
                    match (x.left.take(), x.right.take()) {
                        (None, None) => *node = None,
                        (Some(left), None) => *node = Some(left),
                        (None, Some(right)) => *node = Some(right),
                        (Some(left), Some(right_sub)) => {
                            let mut min_val = right_sub.element.clone();
                            {
                                let mut cur = &right_sub;
                                while let Some(ref l) = cur.left {
                                    min_val = l.element.clone();
                                    cur = l;
                                }
                            }
                            let mut right_opt = Some(right_sub);
                            remove_recursive(&mut right_opt, &min_val);

                            *node = Some(Box::new(Node {
                                element: min_val,
                                left: Some(left),
                                right: right_opt,
                            }));
                        }
                    }
                    return true;
                }
            }
            false
        }
        remove_recursive(&mut self.root, key)
    }

    pub fn inorder(&self) -> Vec<&T> {
        fn inorder_recursive<'a, T>(node: &'a Option<Box<Node<T>>>, acc: &mut Vec<&'a T>) {
            if let Some(x) = node {
                inorder_recursive(&x.left, acc);
                acc.push(&x.element);
                inorder_recursive(&x.right, acc);
            }
        }
        let mut vec = Vec::new();
        inorder_recursive(&self.root, &mut vec);
        vec
    }
}

impl<T: Display + Ord> Display for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn fmt_node<T: Display>(node: &Option<Box<Node<T>>>, f: &mut Formatter<'_>, depth: usize) -> fmt::Result {
            if let Some(x) = node {
                fmt_node(&x.right, f, depth + 1)?;
                for _ in 0..depth {
                    write!(f, "    ")?;
                }
                writeln!(f, "{}", x.element)?;
                fmt_node(&x.left, f, depth + 1)?;
            }
            Ok(())
        }
        fmt_node(&self.root, f, 0)
    }
}

fn main() {
   
}
