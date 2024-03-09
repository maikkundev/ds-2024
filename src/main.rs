fn merge(arr: Vec<i32>, mut start: usize, mut mid: usize, mut end: usize) {
    // create subarrays
    let mut left = vec![&mut arr[0..=mid].to_vec()];
    let right = vec![&mut arr[mid..=end].to_vec()];

    // index of subarrays and main array
    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    let left_last_index = left.len() - 1;
    let right_last_index = right.len() - 1;

    // pick larger among
    // elements L and M and place them in the correct position
    while i < left_last_index && j < right_last_index {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left - 1 {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right - 1 {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort(arr: &mut Vec<i32>, mut start: usize, mut end: usize) {
    if start < end {
        let m = start + (end - 1) / 2;

        merge_sort(arr, start, m);
        merge_sort(arr, m + 1, end);

        let sorted = merge(arr, start, m, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test() {
        let arr = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        merge_sort(&mut arr, 0, arr.len() - 1);
        assert_eq!(arr, sorted_arr)
    }
}

fn main() {}
