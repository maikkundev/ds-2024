#![allow(dead_code)]

pub fn bubblesort(arr: &mut Vec<i32>) -> Vec<i32> {
    // Iterate over each index
    for i in 0..(arr.len()) {
        for j in 0..(arr.len() - i - 1) {
            // If the next index is bigger; swap the two
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }

    arr.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test() {
        let mut arr = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let result = bubblesort(&mut arr);

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], result);
    }
}
