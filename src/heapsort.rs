use std::collections::BinaryHeap;

/// Represents the `Heap`. It is by default a max-heap and cannot be turned into a min-heap.
#[derive(Debug)]
pub struct Heap<T> {
    data: Vec<T>,
}

impl<T> Heap<T>
where
    T: PartialOrd,
{
    /// Creates a new `Heap` instance.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// To insert an element to a heap, we perform the following steps:
    /// 1. Add the element to the bottom level of the heap at the leftmost open space.
    /// 2. Compare the added element with its parent; if they are in the correct order, stop.
    /// 3. If not, swap the element with its parent and return to the previous step.
    pub fn push(&mut self, value: T) {
        let mut child_index = self.data.len() as isize;
        // usize / usize produces a floored usize
        let mut parent_index = (child_index - 1) / 2;
        self.data.push(value);
        while child_index != 0 && self.data[child_index as usize] > self.data[parent_index as usize]
        {
            self.data.swap(child_index as usize, parent_index as usize);
            child_index = parent_index;
            parent_index = (child_index - 1) / 2;
        }
    }

    /// The procedure for deleting the root from the heap (effectively extracting the maximum element in a max-heap or the minimum element in a min-heap) while retaining the heap property is as follows:
    /// 1. Replace the root of the heap with the last element on the last level.
    /// 2. Compare the new root with its children; if they are in the correct order, stop.
    /// 3. If not, swap the element with one of its children and return to the previous step. (Swap with its smaller child in a min-heap and its larger child in a max-heap.)
    pub fn pop(&mut self) -> Option<T> {
        // Return `None` is the vec is empty
        if self.data.is_empty() {
            return None;
        }
        // Get the index of the last element and swap it with the root element (1)
        let last_el = self.data.len() - 1;
        self.data.swap(0, last_el);
        // Get value to return
        let result = Some(self.data.remove(last_el));

        while 
        {
            self.data.swap(child_index as usize, parent_index as usize);
            child_index = parent_index;
            parent_index = (child_index - 1) / 2;
        }
        // Return the removed element
        result
    }

    pub fn sort(&mut self) -> Vec<T> {
        vec![]
    }
}

//
fn heapsort(vec: &mut Vec<i32>) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use std::collections::binary_heap;

    use super::*;

    #[test]
    fn test_push() {
        let vec = vec![5, 10, 8, 2, 2, 5, 100, 31, 36];
        let mut heap = Heap::new();
        let mut binary_heap = BinaryHeap::new();
        vec.iter().for_each(|x| {
            heap.push(x);
            binary_heap.push(x)
        });
        // println!("{:?}", heap);
        // println!("{:?}", binary_heap);
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
        println!("Before pop {:?}", heap);
        heap.pop();
        println!("After pop {:?}", heap);
        println!("Before pop {:?}", binary_heap);
        binary_heap.pop();
        println!("After pop {:?}", binary_heap);

        // [36, 10, 31, 2, 5, 8, 2, 5]
        // [36, 31, 10, 5, 2, 5, 8, 2]
    }
}
