// ΡΑΤΣΙΚΑΣ ΑΘΑΝΑΣΙΟΣ ΜΑΡΙΟΣ - inf2021193 | ΑΒΑΓΙΑΝΟΣ ΜΙΧΑΗΛ - inf2021009
// Creation: March 30
// Update: May 12
// cargo run --bin shellsort

mod record;

use rand::distributions::Uniform;
use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

use crate::record::Record;

pub fn shellsort<T: PartialOrd + Clone>(vec: &mut Vec<T>, length: usize) {
    let mut interval = length / 2;
    while interval > 0 {
        for i in interval..length {
            let temp = vec[i].clone();
            let mut j = i;
            while j >= interval && vec[j - interval] > temp {
                vec[j] = vec[j - interval].clone();
                j -= interval;
            }
            vec[j] = temp;
        }
        interval /= 2;
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
            let data: Vec<i32> = rng.clone().sample_iter(&dist).take(size).collect();
            let start = Instant::now();
            shellsort(&mut data.clone(), data.len());
            duration_vec.push(start.elapsed().as_micros());
        }
        let average = duration_vec.iter().sum::<u128>() / duration_vec.len() as u128;
        let output = format!(
            "[SHELLSORT] Integers: {} - Average Time: {:?}μs",
            size, average
        );
        println!("{}", output);

        if let Err(e) = writeln!(file, "{}", output) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }

    for size in (100..=1000).step_by(100) {
        let mut duration_vec = vec![];
        for _ in 0..100 {
            let data = Record::create_rand_vec(size);
            let start = Instant::now();
            shellsort(&mut data.clone(), data.len());
            duration_vec.push(start.elapsed().as_micros());
        }
        let average = duration_vec.iter().sum::<u128>() / duration_vec.len() as u128;
        let output = format!(
            "[SHELLSORT] Records: {} - Average Time: {:?}μs",
            size, average
        );
        println!("{}", output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_elements() {
        let mut vec: Vec<i32> = vec![];
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
