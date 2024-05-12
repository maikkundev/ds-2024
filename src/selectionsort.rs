pub fn selectionsort(vec: &mut Vec<i32>) -> Vec<i32> {
    // Loop over the length of the vec
    for i in 0..vec.len() {
        // Find the smallest number in the vec
        let mut smallest = i;
        for j in (i + 1)..vec.len() {
            if vec[j] < vec[smallest] {
                smallest = j;
            }
        }
        // Swap the current number with smallest
        vec.swap(i, smallest);
    }
    // Return the sorted vec
    vec.to_owned()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_elements() {
        let mut vec = vec![];
        assert_eq!(selectionsort(&mut vec), [])
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        assert_eq!(selectionsort(&mut vec), [1])
    }

    #[test]
    fn ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(selectionsort(&mut vec), sorted_vec)
    }

    #[test]
    fn ten_elements_with_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(selectionsort(&mut vec), sorted_vec)
    }
}
