#![allow(dead_code)]
pub fn shell_sort(arr: &mut Vec<i32>, size: usize) {
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
