#[cfg(test)]
mod tests;

/// Binary search tree
#[derive(Clone)]
pub struct BSTree<T>
where
    T: PartialOrd + PartialEq + Copy + Clone,
{
    value: T,
    left: Option<Box<BSTree<T>>>,
    right: Option<Box<BSTree<T>>>,
}

impl<T> BSTree<T>
where
    T: PartialEq + PartialOrd + Copy + Clone,
{
    pub fn new(root_val: T) -> Self {
        BSTree {
            value: root_val,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        if value < self.value {
            match &mut self.left {
                None => self.left = Some(Box::new(BSTree::new(value))),
                Some(tree) => tree.insert(value),
            }
        } else if value > self.value {
            match &mut self.right {
                None => self.right = Some(Box::new(BSTree::new(value))),
                Some(tree) => tree.insert(value),
            }
        }
    }

    pub fn get(&self, value: T) -> Option<&T> {
        if self.value == value {
            Some(&self.value)
        } else if value < self.value {
            match &self.left {
                None => None,
                Some(tree) => tree.get(value),
            }
        } else {
            match &self.right {
                None => None,
                Some(tree) => tree.get(value),
            }
        }
    }

    pub fn delete(self, value: T) -> Option<Self> {
        Self::delete_node(Some(self), value)
    }

    fn delete_node(root: Option<BSTree<T>>, value: T) -> Option<Self> {
        root.as_ref()?;

        let mut root = root.unwrap();

        if value < root.value {
            root.left = {
                if let Some(node) = root.left {
                    let temp = Self::delete_node(Some(*node), value);
                    if let Some(node) = temp {
                        Some(Box::new(node))
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
        } else if value > root.value {
            root.right = {
                if let Some(node) = root.right {
                    let temp = Self::delete_node(Some(*node), value);
                    if let Some(node) = temp {
                        Some(Box::new(node))
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
        } else {
            if root.left.is_none() {
                let temp = root.left;
                return match temp {
                    None => None,
                    Some(node) => Some(*node),
                };
            } else if root.right.is_none() {
                let temp = root.right;
                return match temp {
                    None => None,
                    Some(node) => Some(*node),
                };
            }

            root.right = {
                if let Some(node) = root.right {
                    root.value = *node.min();
                    let temp = Self::delete_node(Some(*node), root.value);
                    if let Some(node) = temp {
                        Some(Box::new(node))
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
        }

        Some(root)
    }

    pub fn len(&self) -> usize {
        let mut length = 1;
        if let Some(tree) = &self.left {
            length += tree.len();
        }
        if let Some(tree) = &self.right {
            length += tree.len();
        }

        length
    }

    pub fn ordered_traversal(&self) -> Vec<&T> {
        let mut elements = Vec::with_capacity(self.len());

        if let Some(tree) = &self.left {
            elements.append(&mut tree.ordered_traversal());
        }
        elements.push(&self.value);
        if let Some(tree) = &self.right {
            elements.append(&mut tree.ordered_traversal());
        }

        elements
    }

    pub fn max(&self) -> &T {
        if let Some(tree) = &self.right {
            tree.max()
        } else {
            &self.value
        }
    }

    pub fn min(&self) -> &T {
        if let Some(tree) = &self.left {
            tree.min()
        } else {
            &self.value
        }
    }
}
