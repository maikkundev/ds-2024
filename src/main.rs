fn merge(arr: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    // create subarrays
    let left = arr[start..=mid].to_vec();
    let right = arr[mid + 1..=end].to_vec();

    // index of subarrays and main array
    let mut i = 0;
    let mut j = 0;
    let mut k = start;


    // pick larger among
    // elements L and M and place them in the correct position
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
        let m = start + (end - start) / 2;

        merge_sort(arr, start, m);
        merge_sort(arr, m + 1, end);

        merge(arr, start, m, end);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test() {
        let mut arr = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let arr_len = arr.len();
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        merge_sort(&mut arr, 0, arr_len - 1);
        assert_eq!(arr, sorted_arr)
    }
}

fn main() {}
