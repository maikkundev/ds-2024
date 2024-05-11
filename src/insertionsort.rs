fn insertionsort(vec: &mut Vec<i32>) -> Vec<i32> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertionsort_no_elements() {
        let mut vec = vec![];
        assert_eq!(insertionsort(&mut vec), [])
    }

    #[test]
    fn insertionsort_one_element() {
        let mut vec = vec![1];
        assert_eq!(insertionsort(&mut vec), [1])
    }

    #[test]
    fn insertionsort_ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(insertionsort(&mut vec), sorted_vec)
    }

    #[test]
    fn insertionsort_ten_elements_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(insertionsort(&mut vec), sorted_vec)
    }
}
