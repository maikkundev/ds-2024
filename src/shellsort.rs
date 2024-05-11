pub fn shellsort(vec: &mut Vec<i32>, length: usize) {
    let mut interval = length / 2;
    while interval > 0 {
        for i in interval..length {
            let temp = vec[i];
            let mut j = i;
            while j >= interval && vec[j - interval] > temp {
                vec[j] = vec[j - interval];
                j -= interval;
            }
            vec[j] = temp;
        }
        interval /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shell_sort_test() {
        let mut vec = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let length = vec.len();
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        println!("Original Vec: {:?}", vec);
        shellsort(&mut vec, length);
        println!("Sorted Vec with Shell Sort: {:?}", vec);
        println!("{:?}", sorted_vec);

        assert_eq!(vec, sorted_vec);
    }
}
