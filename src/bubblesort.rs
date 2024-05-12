// ΡΑΤΣΙΚΑΣ ΑΘΑΝΑΣΙΟΣ ΜΑΡΙΟΣ - inf2021193 | ΑΒΑΓΙΑΝΟΣ ΜΙΧΑΗΛ - inf2021009
// Creation: March 17
// Update: May 12
// cargo run --bin bubblesort

use rand::distributions::Uniform;
use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

pub fn bubblesort<T: PartialOrd>(vec: &mut Vec<T>) {
    // Iterate over each index
    for i in 0..(vec.len()) {
        for j in 0..(vec.len() - i - 1) {
            // If the next index is bigger; swap the two
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
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
            bubblesort(&mut data);
            duration_vec.push(start.elapsed().as_micros());
        }
        let average = duration_vec.iter().sum::<u128>() / duration_vec.len() as u128;
        let output = format!(
            "[BUBBLESORT] Integers: {} - Average Time: {:?}μs",
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
        let mut vec: Vec<u64> = vec![];
        bubblesort(&mut vec);
        assert_eq!(vec, [])
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        bubblesort(&mut vec);
        assert_eq!(vec, [1]);
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
