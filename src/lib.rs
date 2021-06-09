enum BinaryTree {
    data(Free),
    root(Box<[BinaryTree; 2]>),
}
impl BinaryTree {
    pub fn get(&self, index: usize) -> Free {
        match self {
            Self::data(free) => {
                if index == 0 {
                    *free
                } else {
                    panic!("invalid index")
                }
            }
            Self::root(data) => data[index & 1].get(index >> 1),
        }
    }
    /// Gets first element with free root and sets it to used
    pub fn get_first_free(&mut self, traverse_levels: usize) -> Option<usize> {
        match self {
            Self::data(free) => {
                if traverse_levels == 0 {
                    if *free == Free::Free {
                        *free = Free::Used;
                        Some(0)
                    } else {
                        None
                    }
                } else {
                    match free {
                        Free::Free => {
                            *self = BinaryTree::root(Box::new([
                                BinaryTree::data(Free::Free),
                                BinaryTree::data(Free::Free),
                            ]));
                            match self {
                                BinaryTree::data(_) => panic!("impossible condition"),
                                BinaryTree::root(data) => {
                                    Some(data[0].get_first_free(traverse_levels - 1).unwrap() << 1)
                                }
                            }
                        }
                        Free::Used => None,
                    }
                }
            }
            Self::root(data) => {
                if traverse_levels > 0 {
                    if let Some(first_try) = data[0].get_first_free(traverse_levels - 1) {
                        Some(first_try << 1)
                    } else if let Some(second_try) = data[1].get_first_free(traverse_levels - 1) {
                        Some((second_try << 1) + 1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}
#[derive(Copy, Clone, PartialEq)]
enum Free {
    Used,
    Free,
}
pub struct BuddyAllocator {
    free_tree: BinaryTree,
    data: [u8; Self::BLOCK_LEVELS * Self::MIN_BLOCK_SIZE],
}
pub struct Allocation<'a> {
    data: &'a [u8],
    alloc_index: usize,
}
impl BuddyAllocator {
    const MIN_BLOCK_SIZE: usize = 1024 * 64;
    const BLOCK_LEVELS: usize = 4;
    pub fn new() -> Self {
        Self {
            free_tree: BinaryTree::data(Free::Free),
            data: [0; Self::BLOCK_LEVELS * Self::MIN_BLOCK_SIZE],
        }
    }
    pub fn alloc(&self, allocation_size: usize) -> Allocation {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build() {
        let _tree = BuddyAllocator::new();
    }
    #[test]
    fn allocate() {
        let mut tree = BuddyAllocator::new();
        let alloc = tree.alloc(10);
    }
}
