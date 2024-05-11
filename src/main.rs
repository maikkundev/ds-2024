mod bubblesort;
mod mergesort;
mod quicksort;
mod selectionsort;
mod shellsort;

use bubblesort::bubblesort;
use mergesort::mergesort;
use quicksort::quicksort;
use selectionsort::selectionsort;
use shellsort::shellsort;

fn main() {
    let mut vec = vec![1, 3, 5, 7, 1, 9, 3, 3, 4, 1, 12, 3];
    let length = vec.len();
    println!("{:?}", bubblesort(&mut vec));
    println!("{:?}", quicksort(&mut vec));
    println!("{:?}", mergesort(&mut vec, 0, length - 1));
    println!("{:?}", selectionsort(&mut vec));
    println!("{:?}", shellsort(&mut vec, length));
}
