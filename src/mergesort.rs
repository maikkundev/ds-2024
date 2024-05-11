#![allow(dead_code)]

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_test() {
        let mut vec = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let length = vec.len();
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        mergesort(&mut vec, 0, length - 1);
        print!(
            "Array: {:?}, Sorted Vec with Merge Sort: {:?}",
            vec, sorted_vec
        );

        assert_eq!(vec, sorted_vec);
    }
}
