mod quicksort;
use quicksort::quicksort;

fn main() {
    let mut vec = vec![1, 5, 10, 3, 5, 5, 6, 7, 4, 93, 12];

    println!("{:?}", quicksort(&mut vec));
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
