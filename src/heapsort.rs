/// An implementation of a binary max-heap.
#[derive(Debug)]
pub struct Heap<T>
where
    T: Ord + Copy,
{
    data: Vec<T>,
}

impl<T> Heap<T>
where
    T: Ord + Copy,
{
    /// Creates an empty `Heap` instance.
    ///
    /// This will be a max-heap by default, and cannot be changed to a min-heap.
    ///
    /// ## Example
    ///
    /// ```
    /// let heap = Heap::new();
    ///
    /// // Prints: Heap []
    /// println!("{:?}", heap);
    /// ```
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Pushes a value into the heap.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut heap = Heap::new();
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    ///
    /// // Prints: Heap [5, 1, 2]
    /// println!("{:?}", heap);
    /// ```
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        let mut last_index = self.len() - 1;

        while last_index != 0 {
            let parent_index = self.parent(last_index);

            if self.data[last_index] > self.data[parent_index] {
                self.data.swap(last_index, parent_index)
            }

            last_index = parent_index;
        }
    }

    /// Pops the greatest value from the heap returns it wrapped in an `Option`.
    ///
    /// If the heap is empty, returns `None`.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut heap = Heap::new();
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    ///
    /// // Prints: Heap [5, 1, 2]
    /// println!("{:?}", heap);
    /// heap.pop();
    ///
    /// // Prints: Heap [2, 1]
    /// println!("{:?}", heap)
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        match self.len() {
            n if n == 0 => None,
            n if n == 1 => self.data.pop(),
            _ => {
                // Set last as the last index in the vec
                let last = self.data.len() - 1;
                // Swap last and root
                self.data.swap(0, last);
                // Set result as the last element
                let result = self.data.pop().unwrap();

                // Heapify
                self.heapify(0);

                // Return the result
                Some(result)
            }
        }
    }

    /// Returns the heap as a sorted `Vec`.
    ///
    /// ## Example
    ///
    /// ```
    /// let mut heap = Heap::new();
    ///
    /// heap.push(1);
    /// heap.push(5);
    /// heap.push(2);
    ///
    /// // Prints: Heap [5, 1, 2]
    /// println!("{:?}", heap);
    ///
    /// // Prints: [1, 2, 5]
    /// println!("{:?}", heap.into_sorted_vec());
    /// ```
    pub fn into_sorted_vec(&mut self) -> Vec<T> {
        let mut vec = vec![];
        // Pushed the popped elements of the heap to the vec, while the vec isn't empty
        while self.len() > 0 {
            vec.push(self.pop().unwrap());
        }
        // Reverses the vec, since the heap is by default a max-heap
        vec.reverse();
        // Returns the vec
        vec
    }

    /// Returns the length of the heap.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Takes a `Vec` and an index, and turns it into a `Heap`.
    fn heapify(&mut self, index: usize) {
        let mut current = index;

        loop {
            let left = self.left(current);
            let right = self.right(current);
            let size = self.len() - 1;

            if left > size || right > size {
                break;
            }

            let max = self.get_max_index(left, right, current);

            if max != current {
                self.data.swap(current, max);

                current = max;
            } else {
                break;
            }
        }
    }

    /// Returns the max index from three given indexes.
    fn get_max_index(&self, left_index: usize, right_index: usize, current_index: usize) -> usize {
        let max = self.data[left_index]
            .max(self.data[right_index])
            .max(self.data[current_index]);

        match max {
            i if i == self.data[current_index] => current_index,
            i if i == self.data[right_index] => right_index,
            _ => left_index,
        }
    }

    /// Returns the parent index.
    fn parent(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    /// Returns the left child.
    fn left(&self, index: usize) -> usize {
        2 * index + 1
    }

    /// Returns the right child.
    fn right(&self, index: usize) -> usize {
        2 * index + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BinaryHeap;

    #[test]
    fn test_push() {
        let vec = vec![5, 10, 8, 2, 2, 5, 100, 31, 36];
        let mut heap = Heap::new();
        let mut binary_heap = BinaryHeap::new();
        vec.iter().for_each(|x| {
            heap.push(x);
            binary_heap.push(x)
        });
        // assert_eq!(heap, binary_heap);
    }

    #[test]
    fn test_pop() {
        let vec = vec![5, 10, 8, 2, 2, 5, 100, 31, 36];
        let mut heap = Heap::new();
        let mut binary_heap = BinaryHeap::new();
        vec.iter().for_each(|x| {
            heap.push(x);
            binary_heap.push(x)
        });
        heap.pop();
        binary_heap.pop();
        // assert_eq!(heap, binary_heap);
    }

    #[test]
    fn test_into_sorted_vec() {
        let vec = vec![5, 10, 8, 2, 2, 5, 100, 31, 36];
        let mut heap = Heap::new();
        let mut binary_heap = BinaryHeap::new();
        vec.iter().for_each(|x| {
            heap.push(x);
            binary_heap.push(x)
        });
        // assert_eq!(heap.into_sorted_vec(), binary_heap.into_sorted_vec());
    }
}
