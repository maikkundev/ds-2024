use rand::distributions::Uniform;
use rand::prelude::*;
use std::time::Instant;

pub fn bubblesort(vec: &mut Vec<i32>) -> Vec<i32> {
    // Iterate over each index
    for i in 0..(vec.len()) {
        for j in 0..(vec.len() - i - 1) {
            // If the next index is bigger; swap the two
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
    // Return sorted vec
    vec.to_owned()
}

fn main() {
    let rng = rand::thread_rng();
    let mut vec: Vec<i32> = Vec::new();

    let dist = Uniform::from(0..=1e6 as i32);
    let data: Vec<Vec<i32>> = (100..=1000)
        .step_by(100)
        .map(|size| rng.clone().sample_iter(dist).take(size).collect())
        .collect();
    for data in data {
        let length = data.len();
        let start = Instant::now();
        bubblesort(&mut vec);
        let duration = start.elapsed().as_nanos();
        println!("[BUBBLESORT] Integers: {} - Time: {:?}ns", length, duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_elements() {
        let mut vec = vec![];
        assert_eq!(bubblesort(&mut vec), [])
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        assert_eq!(bubblesort(&mut vec), [1])
    }

    #[test]
    fn ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(bubblesort(&mut vec), sorted_vec)
    }

    #[test]
    fn ten_elements_with_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(bubblesort(&mut vec), sorted_vec)
    }
}
