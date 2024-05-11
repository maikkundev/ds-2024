pub fn shellsort(arr: &mut Vec<i32>, size: usize) {
    let mut interval = size / 2;
    while interval > 0 {
        for i in interval..size {
            let temp = arr[i];
            let mut j = i;
            while j >= interval && arr[j - interval] > temp {
                arr[j] = arr[j - interval];
                j -= interval;
            }
            arr[j] = temp;
        }
        interval /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_sort_test() {
        let mut arr = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let arr_len = arr.len();
        let mut sorted_arr = arr.clone();

        sorted_arr.sort();
        println!("Original Array: {:?}", arr);
        shellsort(&mut arr, arr_len);
        println!("Sorted Array with Shell Sort: {:?}", arr);
        println!("{:?}", sorted_arr);

        assert_eq!(arr, sorted_arr);
    }
}
