#[derive(Debug)]
struct BinaryTree<T> {
    value: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T: PartialEq + PartialOrd> BinaryTree<T> {
    fn new(value: T) -> BinaryTree<T> {
        BinaryTree { value, left: None, right: None }
    }

    fn add(&mut self, value: T) {
        if self.value == value { return; };
        let update =
            if value > self.value { &mut self.right } else { &mut self.left };
        match update {
            Some(update) => update.add(value),
            None => *update = Some(Box::new(BinaryTree::new(value)))
        }
    }

    fn search(&self, target: T) -> Option<T> {
        if target == self.value { Some(target) }
        else if target < self.value { self.left.as_ref()?.search(target) }
        else if target > self.value { self.right.as_ref()?.search(target) }
        else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut root = BinaryTree::new(5);
        root.add(7);
        root.add(4);
        root.add(3);
        root.add(6);
        root.add(8);

        assert_eq!(root.search(4), Some(4));
    }
}