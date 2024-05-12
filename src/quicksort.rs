// ΡΑΤΣΙΚΑΣ ΑΘΑΝΑΣΙΟΣ ΜΑΡΙΟΣ - inf2021193 | ΑΒΑΓΙΑΝΟΣ ΜΙΧΑΗΛ - inf2021009
// Creation: March 9
// Update: May 12
// cargo run --bin ds-2024

use rand::distributions::Uniform;
use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

fn partition(vec: &mut Vec<i32>) -> Vec<i32> {
    // If vec length is 0 or 1, return the vec
    if vec.len() <= 1 {
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
    // Partitioned left and right slices
    let mut part_left = partition(&mut left);
    let mut part_right = partition(&mut right);
    part_left.append(&mut part_right);
    part_left
}

pub fn quicksort(vec: &mut Vec<i32>) -> Vec<i32> {
    partition(vec)
}

fn main() {
    let rng = rand::thread_rng();

    let dist = Uniform::from(0..=1e6 as i32);

    let mut file = OpenOptions::new()
        .append(true) // Open the file in append mode
        .create(true) // Create the file if it does not exist
        .open("results.txt")
        .unwrap();

    for size in (100..=1000).step_by(100) {
        let mut duration_vec = vec![];
        for _ in 0..100 {
            let mut data: Vec<i32> = rng.clone().sample_iter(&dist).take(size).collect();
            let start = Instant::now();
            quicksort(&mut data);
            duration_vec.push(start.elapsed().as_micros());
        }
        let average = duration_vec.iter().sum::<u128>() / duration_vec.len() as u128;
        let output = format!(
            "[QUICKSORT] Integers: {} - Average Time: {:?}μs",
            size, average
        );
        println!("{}", output);

        if let Err(e) = writeln!(file, "{}", output) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_elements() {
        let mut vec = vec![];
        assert_eq!(quicksort(&mut vec), [])
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        assert_eq!(quicksort(&mut vec), [1])
    }

    #[test]
    fn ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(quicksort(&mut vec), sorted_vec)
    }

    #[test]
    fn ten_elements_with_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(quicksort(&mut vec), sorted_vec)
    }
}
