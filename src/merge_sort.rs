fn merge(arr: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    // Create subarrays
    let left = arr[start..=mid].to_vec();
    let right = arr[mid + 1..=end].to_vec();

    // Index of subarrays and main array
    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    // Pick larger among elements Vec left and right and place them in the correct position
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Pick up the remaining elements when either Vec left or right run out of elements
    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort(arr: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
        let middle = start + (end - start) / 2;

        merge_sort(arr, start, middle);
        merge_sort(arr, middle + 1, end);

        // Merge sorted subarrays
        merge(arr, start, middle, end);
    }
}

// CMD: Cargo test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_sort_test() {
        let mut arr = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let arr_len = arr.len();
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        merge_sort(&mut arr, 0, arr_len - 1);
        print!(
            "Array: {:?}, Sorted Array with Merge Sort: {:?}",
            arr, sorted_arr
        );
        assert_eq!(arr, sorted_arr)
    }
}

fn main() {}
