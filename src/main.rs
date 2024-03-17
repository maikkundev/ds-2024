mod bubble_sort;
use bubble_sort::bubble_sort;

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
}
