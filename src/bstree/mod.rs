#[cfg(test)]
mod tests;

/// Binary search tree
pub struct BSTree<T>
    where T: PartialOrd + PartialEq
{
    value: T,
    left: Option<Box<BSTree<T>>>,
    right: Option<Box<BSTree<T>>>
}

impl<T> BSTree<T>
    where T: PartialEq + PartialOrd
{
    pub fn new(root_val: T) -> Self {
        BSTree {
            value: root_val,
            left: None,
            right: None
        }
    }

    pub fn insert(&mut self, value: T) {
        if value == self.value {
            // No duplicate values
            return
        } else if value < self.value {
            match &mut self.left {
                None => self.left = Some(Box::new(BSTree::new(value))),
                Some(tree) => tree.insert(value)
            }
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(BSTree::new(value))),
                Some(tree) => tree.insert(value)
            }
        }
    }

    pub fn get(&self, value: T) -> Option<&T> {
        if self.value == value {
            Some(&self.value)
        } else if value < self.value {
            match &self.left {
                None => None,
                Some(tree) => tree.get(value)
            }
        } else {
            match &self.right {
                None => None,
                Some(tree) => tree.get(value)
            }
        }
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
