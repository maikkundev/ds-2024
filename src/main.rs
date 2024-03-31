#![allow(unused_imports)]
mod quicksort;
mod bubble_sort;
mod merge_sort;

use bubble_sort::bubble_sort;
use quicksort:quicksort;
use merge_sort::merge_sort;

fn main() {}

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
    fn quicksort_no_elements() {
        let mut vec = vec![];
        assert_eq!(quicksort(&mut vec), [])
    }

    #[test]
    fn quicksort_one_element() {
        let mut vec = vec![1];
        assert_eq!(quicksort(&mut vec), [1])
    }

    #[test]
    fn quicksort_ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(quicksort(&mut vec), sorted_vec)
    }

    #[test]
    fn quicksort_ten_elements_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(quicksort(&mut vec), sorted_vec)
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
