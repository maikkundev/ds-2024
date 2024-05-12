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
    fn no_elements() {
        let mut vec = vec![];
        assert_eq!(bubblesort(&mut vec), [])
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        assert_eq!(bubblesort(&mut vec), [1])
    }

    #[test]
    fn ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(bubblesort(&mut vec), sorted_vec)
    }

    #[test]
    fn ten_elements_with_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(bubblesort(&mut vec), sorted_vec)
    }
}
