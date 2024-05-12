use rand::distributions::Uniform;
use rand::prelude::*;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Instant;

fn merge(vec: &mut Vec<i32>, start: usize, mid: usize, end: usize) {
    // Create left and right slices
    let left = vec[start..=mid].to_owned();
    let right = vec[mid + 1..=end].to_owned();

    // Index of slices and main vec
    let mut i = 0;
    let mut j = 0;
    let mut k = start;

    // Pick larger among elements of left and right slices and place them in the correct position
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            vec[k] = left[i];
            i += 1;
        } else {
            vec[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    // Pick up the remaining elements when either left or right slices run out of elements
    while i < left.len() {
        vec[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        vec[k] = right[j];
        j += 1;
        k += 1;
    }
}

pub fn mergesort(vec: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
        let middle = start + (end - start) / 2;

        mergesort(vec, start, middle);
        mergesort(vec, middle + 1, end);

        // Merge sorted slices
        merge(vec, start, middle, end);
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
            let data: Vec<i32> = rng.clone().sample_iter(&dist).take(size).collect(); // Create a separate mutable copy of the data vector
            let start = Instant::now();
            mergesort(&mut data.clone(), 0, data.len() - 1); // Pass the mutable copy to mergesort
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
        let mut vec = vec![];
        let end = vec.len() - 1;
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        mergesort(&mut vec, 0, end);
        assert_eq!(vec, sorted_vec)
    }

    #[test]
    fn one_element() {
        let mut vec = vec![1];
        let end = vec.len() - 1;
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        mergesort(&mut vec, 0, end);
        assert_eq!(vec, sorted_vec)
    }

    #[test]
    fn ten_elements() {
        let mut vec = vec![1, 5, 9, 3, 4, 11, 18, 51, 103, 2];
        let end = vec.len() - 1;
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        mergesort(&mut vec, 0, end);
        assert_eq!(vec, sorted_vec)
    }

    #[test]
    fn ten_elements_with_duplicates() {
        let mut vec = vec![1, 5, 9, 3, 5, 11, 11, 51, 103, 1];
        let end = vec.len() - 1;
        let mut sorted_vec = vec.clone();

        sorted_vec.sort();
        mergesort(&mut vec, 0, end);
        assert_eq!(vec, sorted_vec)
    }
}
