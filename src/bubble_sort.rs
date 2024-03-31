#![allow(dead_code)]

pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 0..(arr.len()) {
        for j in 0..(arr.len() - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    arr
}
