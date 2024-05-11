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
    fn no_elements() {
        let mut vec = vec![];
        let length = vec.len();
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        shellsort(&mut vec, length);
        assert_eq!(vec, sorted_vec)
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        let length = vec.len();
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        shellsort(&mut vec, length);
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let length = vec.len();
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        shellsort(&mut vec, length);
        assert_eq!(vec, sorted_vec);
    }

    #[test]
    fn ten_elements_with_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let length = vec.len();
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        shellsort(&mut vec, length);
        assert_eq!(vec, sorted_vec);
    }
}
