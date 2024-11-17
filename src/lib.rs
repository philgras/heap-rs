use std::usize;

#[derive(Debug)]
pub struct MinHeap<T: PartialOrd + Copy> {
    elements: Vec<T>,
}

impl<T: PartialOrd + Copy> MinHeap<T> {
    pub fn new() -> MinHeap<T> {
        MinHeap { elements: vec![] }
    }

    pub fn from_seq(values: &Vec<T>) -> MinHeap<T> {
        let mut heap: MinHeap<T> = MinHeap::new();
        for i in values {
            heap.insert(*i);
        }
        heap
    }

    pub fn sort(mut values: Vec<T>) -> Vec<T> {
        let mut heap = MinHeap::from_seq(&values);
        for i in &mut values {
            *i = heap.extract_min();
        }
        values
    }

    pub fn insert(self: &mut Self, x: T) {
        self.elements.push(x);
        self.heapify_up(self.elements.len() - 1);
    }

    pub fn remove(self: &mut Self, idx: usize) {
        self.swap(idx, self.len() - 1);
        self.elements.pop();
        self.heapify_down(idx);
    }

    pub fn extract_min(self: &mut Self) -> T {
        let ret_val = self.elements[0];
        self.remove(0);
        ret_val
    }

    pub fn min(self: &Self) -> T {
        self.elements[0]
    }

    pub fn is_empty(self: &Self) -> bool {
        self.len() == 0
    }

    pub fn len(self: &Self) -> usize {
        self.elements.len()
    }

    fn heapify_up(self: &mut Self, idx: usize) {
        let mut idx = idx;
        loop {
            if let Some(parent_idx) = self.get_parent_idx(idx) {
                // break if in order
                if self.in_order(parent_idx, idx) {
                    break;
                }
                // swap if not
                self.swap(parent_idx, idx);
                idx = parent_idx;
            } else {
                // break if idx is root
                break;
            }
        }
    }

    fn heapify_down(self: &mut Self, idx: usize) {
        let mut idx = idx;
        let mut best_child_idx;

        loop {
            if let Some(left_idx) = self.get_left_child_idx(idx) {
                best_child_idx = left_idx;

                // change the value if a second child is available
                if let Some(right_idx) = self.get_right_child_idx(idx) {
                    if self.in_order(right_idx, left_idx) {
                        best_child_idx = right_idx;
                    }
                }

                if self.in_order(idx, best_child_idx) {
                    break;
                }

                self.swap(idx, best_child_idx);
                idx = best_child_idx;
            } else {
                break;
            }
        }
    }

    fn swap(self: &mut Self, idx1: usize, idx2: usize) {
        self.elements.swap(idx1, idx2);
    }

    fn in_order(self: &Self, idx1: usize, idx2: usize) -> bool {
        self.elements[idx1] < self.elements[idx2]
    }

    fn get_parent_idx(self: &Self, node_idx: usize) -> Option<usize> {
        if node_idx == 0 {
            return None;
        }
        let parent_idx;

        if node_idx % 2 == 0 {
            parent_idx = node_idx / 2 - 1;
        } else {
            parent_idx = (node_idx - 1) / 2;
        }
        return Some(parent_idx);
    }

    fn get_left_child_idx(self: &Self, node_idx: usize) -> Option<usize> {
        let left_idx = node_idx * 2 + 1;
        if left_idx >= self.elements.len() {
            return None;
        } else {
            return Some(left_idx);
        }
    }

    fn get_right_child_idx(self: &Self, node_idx: usize) -> Option<usize> {
        let right_idx = (node_idx + 1) * 2;
        if right_idx >= self.elements.len() {
            return None;
        } else {
            return Some(right_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn new() {
        let heap: MinHeap<i32> = MinHeap::new();
        assert!(heap.is_empty());
    }

    #[test]
    fn types() {
        const N: i32 = 10000;
        let int_values: Vec<i32> = (0..N).collect();
        let float_values: Vec<f32> = int_values.iter().map(|x| *x as f32).collect();
        let heap_float: MinHeap<f32> = MinHeap::from_seq(&float_values);
        let heap_int: MinHeap<i32> = MinHeap::from_seq(&int_values);

        for (i, f) in zip(heap_int.elements, heap_float.elements) {
            assert!(i == f as i32)
        }
    }

    #[test]
    fn min() {
        let mut heap: MinHeap<i32> = MinHeap::new();

        // create some random data
        const N: i32 = 100000;
        let values: Vec<i32> = (0..N).collect();
        let mut shuffled = values.clone();
        shuffled.shuffle(&mut thread_rng());

        for i in &shuffled {
            heap.insert(*i);
        }

        for i in &values {
            assert_eq!(heap.min(), *i);
            assert_eq!(heap.extract_min(), *i);
        }
    }

    #[test]
    fn sort() {
        const N: i32 = 100000;
        let values: Vec<i32> = (0..N).collect();
        let mut shuffled = values.clone();
        shuffled.shuffle(&mut thread_rng());

        let sorted = MinHeap::sort(shuffled);
        assert_eq!(&sorted[..], &values[..]);
    }
}
