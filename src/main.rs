mod shell_sort;
use shell_sort::shell_sort;
// CMD: Cargo test
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
        shell_sort(&mut arr, arr_len);
        println!("Sorted Array with Shell Sort: {:?}", arr);
        println!("{:?}", sorted_arr);
        assert_eq!(arr, sorted_arr)
    }
}

fn main() {}
