#![allow(unused_imports)]

mod bubble_sort;
mod merge_sort;

use bubble_sort::bubble_sort;
use merge_sort::merge_sort;

fn main() {}

// cargo test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test() {
        let arr = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let result = bubble_sort(arr);

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], result);
    }

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

        assert_eq!(arr, sorted_arr);
    }
}
