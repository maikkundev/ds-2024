pub fn bubblesort(vec: &mut Vec<i32>) -> Vec<i32> {
    // Iterate over each index
    for i in 0..(vec.len()) {
        for j in 0..(vec.len() - i - 1) {
            // If the next index is bigger; swap the two
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
    // Return sorted vec
    vec.to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test() {
        let mut vec = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let result = bubblesort(&mut vec);

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], result);
    }
}
