// How it works: https://en.wikipedia.org/wiki/Quicksort
// Rust Implementation: https://gist.github.com/lispandfound/96b9065bf240f94a0c0f
fn quick_sort(vec: &Vec<i32>) -> Vec<i32> {
    match vec.len() {
        0..=1 => vec.to_vec(),
        _ => {
            let result = vec![];

            result
        }
    }
}

fn main() {
    let vec = vec![1, 5, 10, 3, 5, 5, 6, 7, 4, 93, 12];

    println!("{:?}", quick_sort(&vec));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quicksort_no_elements() {
        let vec = vec![];
        assert_eq!(quick_sort(&vec), [])
    }

    #[test]
    fn quicksort_one_element() {
        let vec = vec![1];
        assert_eq!(quick_sort(&vec), [1])
    }

    #[test]
    fn quicksort_ten_elements() {
        let vec = vec![1, 5, 9, 3, 5, 11, 18, 51, 103, 2];
        let mut sorted_vec = vec.clone();
        sorted_vec.sort();
        assert_eq!(quick_sort(&vec), sorted_vec)
    }
}
