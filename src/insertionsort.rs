// ΡΑΤΣΙΚΑΣ ΑΘΑΝΑΣΙΟΣ ΜΑΡΙΟΣ - inf2021193 | ΑΒΑΓΙΑΝΟΣ ΜΙΧΑΗΛ - inf2021009
// Creation: April 1
// Update: May 12
// cargo run --bin insertionsort

use rand::distributions::Uniform;
use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

pub fn insertionsort<T: PartialOrd>(vec: &mut Vec<T>) {
    // Loop over the vec
    for i in 1..vec.len() {
        let mut j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j, j - 1);
            j = j - 1;
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
            insertionsort(&mut data);
            duration_vec.push(start.elapsed().as_micros());
        }
        let average = duration_vec.iter().sum::<u128>() / duration_vec.len() as u128;
        let output = format!(
            "[INSERTIONSORT] Integers: {} - Average Time: {:?}μs",
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
        let mut vec: Vec<i32> = vec![];
        insertionsort(&mut vec);
        assert_eq!(vec, [])
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        insertionsort(&mut vec);
        assert_eq!(vec, [1])
    }

    #[test]
    fn ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let mut sorted_vec = vec.clone();

        insertionsort(&mut vec);
        sorted_vec.sort();
        assert_eq!(vec, sorted_vec)
    }

    #[test]
    fn ten_elements_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let mut sorted_vec = vec.clone();

        insertionsort(&mut vec);
        sorted_vec.sort();
        assert_eq!(vec, sorted_vec)
    }
}
