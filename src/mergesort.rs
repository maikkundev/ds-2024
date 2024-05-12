fn merge(vec: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    // Create left and right slices
    let left = vec[start..=mid].to_owned();
    let right = vec[mid + 1..=end].to_owned();

    // Index of slices and main vec
    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    // Pick larger among elements of left and right slices and place them in the correct position
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            vec[k] = left[i];
            i += 1;
        } else {
            vec[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Pick up the remaining elements when either left or right slices run out of elements
    while i < left.len() {
        vec[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        vec[k] = right[j];
        j += 1;
        k += 1;
    }
}

pub fn mergesort(vec: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
        let middle = start + (end - start) / 2;

        mergesort(vec, start, middle);
        mergesort(vec, middle + 1, end);

        // Merge sorted slices
        merge(vec, start, middle, end);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_elements() {
        let mut vec = vec![];
        let end = vec.len() - 1;
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        mergesort(&mut vec, 0, end);
        assert_eq!(vec, sorted_vec)
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        let end = vec.len() - 1;
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        mergesort(&mut vec, 0, end);
        assert_eq!(vec, sorted_vec)
    }

    #[test]
    fn ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let end = vec.len() - 1;
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        mergesort(&mut vec, 0, end);
        assert_eq!(vec, sorted_vec)
    }

    #[test]
    fn ten_elements_with_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let end = vec.len() - 1;
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        mergesort(&mut vec, 0, end);
        assert_eq!(vec, sorted_vec)
    }
}
