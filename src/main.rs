fn merge(mut arr: Vec<i32>, mut p: usize, mut q: usize, mut r: usize) {
    // create subarrays
    let n1 = q - p + 1;
    let n2 = r - q;

    let mut l: Vec<i32> = Vec::with_capacity(n1);
    let mut m: Vec<i32> = Vec::with_capacity(n2);

    for i in 0..n1 {
        l.push(arr[p + i]);
    }
    for j in 0..n2 {
        m.push(arr[q + 1 + j]);
    }

    // index of subarrays and main array
    let mut i = 0;
    let mut j = 0;
    let mut k = p;

    // pick larger among
    // elements L and M and place them in the correct position
    while i < n1 - 1 && j < n2 - 1 {
        if l[i] <= m[j] {
            arr[k] = l[i];
            i += 1;
        } else {
            arr[k] = m[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 - 1 {
        arr[k] = l[i];
        i += 1;
        k += 1;
    }

    while j < n2 - 1 {
        arr[k] = m[j];
        j += 1;
        k += 1;
    }

    
}

fn merge_sort(arr: &mut Vec<i32>, mut l: usize, mut r: usize) {
    if l < r {
        let m = l + (r - 1) / 2;

        merge_sort(arr, l, m);
        merge_sort(arr, m + 1, r);

        let sorted = merge(arr, l, m, r);
    }
    
}

#[cfg(test)]
mod tests {
    use crate::merge_sort;

    #[test]
    fn bubble_test() {
        let arr = vec![5, 3, 8, 4, 2, 6, 9, 1, 7];
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        merge_sort(&mut arr, 0, arr.len() - 1);
        assert_eq!(arr, sorted_arr)
    }
}

fn main() {}
