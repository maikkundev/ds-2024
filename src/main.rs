mod bubblesort;
mod heapsort;
mod mergesort;
mod quicksort;
mod selectionsort;
mod shellsort;

use bubblesort::bubblesort;
use mergesort::mergesort;
use quicksort::quicksort;
use selectionsort::selectionsort;
use shellsort::shellsort;

use crate::heapsort::Heap;

fn main() {
    let mut vec = vec![1, 3, 5, 7, 1, 9, 3, 3, 4, 1, 12, 3];
    let length = vec.len();
    let mut merge_vec = vec.clone();
    let mut shell_vec = vec.clone();
    mergesort(&mut merge_vec, 0, length - 1);
    shellsort(&mut shell_vec, length);

    let mut heap: Heap<i32> = Heap::new();
    vec.iter().for_each(|x| heap.push(*x));

    println!("{:?}", bubblesort(&mut vec));
    println!("{:?}", quicksort(&mut vec));
    println!("{:?}", merge_vec);
    println!("{:?}", selectionsort(&mut vec));
    println!("{:?}", shell_vec);
    println!("{:?}", heap.into_sorted_vec());
}
