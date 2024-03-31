#![allow(unused_imports)]
mod bubble_sort;
mod merge_sort;
mod quicksort;

use bubble_sort::bubble_sort;
use merge_sort::merge_sort;
use quicksort::quicksort;

fn main() {
    let mut vec = vec![1, 3, 5, 7, 1, 9, 3, 3, 4, 1, 12, 3];
    let length = &vec.len();
    println!("{:?}", bubble_sort(&mut vec));
    println!("{:?}", quicksort(&mut vec));
    println!("{:?}", merge_sort(&mut vec, 0, length - 1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test() {
        let mut arr = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let result = bubble_sort(&mut arr);

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
        let length = arr.len();
        let mut sorted_arr = arr.clone();

        sorted_arr.sort();

        merge_sort(&mut arr, 0, length - 1);
        print!(
            "Array: {:?}, Sorted Array with Merge Sort: {:?}",
            arr, sorted_arr
        );

        assert_eq!(arr, sorted_arr);
    }
}
