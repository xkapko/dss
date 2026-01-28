type Handle = Option<usize>;

#[derive(Debug, Clone)]
pub struct SparseSet<T> {
    sparse: Vec<Handle>,
    dense: Vec<(usize, T)>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self::with_capacity(32)
    }
}

impl<T> SparseSet<T> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            sparse: vec![None; cap],
            dense: Vec::with_capacity(cap),
        }
    }

    pub fn push_at(&mut self, item: T, index: usize) -> Option<T> {
        if self.sparse.len() <= index {
            self.dense.push((self.sparse.len(), item));
            self.sparse.insert(index, Some(self.dense.len() - 1));
            return None;
        }

        // If there is an item at that index, swap it with the new one and return the old one.
        if let Some(dense_index) = self.sparse[index] {
            let item = (index, item);
            let (_, out) = std::mem::replace(self.dense.get_mut(dense_index).unwrap(), item);
            return Some(out);
        }

        self.sparse[index] = Some(self.dense.len() - 1);

        None
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if let Some(Some(dense_index)) = self.sparse.get(index) {
            let (_, item) = self.dense.get(*dense_index).unwrap();
            return Some(item);
        }

        None
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if let Some(Some(dense_index)) = self.sparse.get(index) {
            let (_, item) = self.dense.get_mut(*dense_index).unwrap();
            return Some(item);
        }

        None
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if let Some(Some(dense_index)) = self.sparse.get(index) {
            let (i, item) = self.dense.swap_remove(*dense_index);
            self.sparse[i] = Some(self.dense.len() - 1);
            return Some(item);
        }

        None
    }
}
