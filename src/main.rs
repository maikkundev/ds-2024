mod quicksort;
mod selection_sort;
use quicksort::quicksort;

fn main() {
    let mut vec = vec![1, 5, 10, 3, 5, 5, 6, 7, 4, 93, 12];

    println!("{:?}", quicksort(&mut vec));
}
