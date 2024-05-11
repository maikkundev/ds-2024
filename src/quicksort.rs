// How it works: https://en.wikipedia.org/wiki/Quicksort
// Rust Implementation: https://gist.github.com/lispandfound/96b9065bf240f94a0c0f
fn partition(vec: &mut Vec<i32>) -> Vec<i32> {
    // If arr length is 0 or 1, return the vec
    if vec.len() <= 1 {
        println!("vec length {}", vec.len());
        return vec.to_owned();
    }
    // Create the two sub-arrays
    // Do so by comparing the value of the pivot to the one an index before - see if the pivot is a higher value than the
    // other index's value.
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    // Loop through the vec and
    for el in &vec[1..] {
        if *el <= vec[0] {
            left.push(*el)
        } else {
            right.push(*el)
        }
    }
    // Push pivot into right slice
    right.push(vec[0]);
    println!("left slice {:?}", left);
    println!("right slice {:?}", right);
    // Partitioned left and right slices
    let mut part_left = partition(&mut left);
    let mut part_right = partition(&mut right);
    part_left.append(&mut part_right);
    part_left
}

pub fn quicksort(vec: &mut Vec<i32>) -> Vec<i32> {
    partition(vec)
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
