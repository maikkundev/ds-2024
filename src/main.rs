// Buble sort fucntion
fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 0..(arr.len()) {
        for j in 0..(arr.len() - i - 1) {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }

    arr
}

#[cfg(test)]
mod tests {
    use crate::bubble_sort;

    #[test]
    fn bubble_test() {
        let arr = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let result = bubble_sort(arr);
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}

fn main() {}
