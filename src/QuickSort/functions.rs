pub(crate) fn partitioning(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = arr[high];
    let mut i = low as isize - 1;

    for j in low..high {
        if arr[j] <= pivot {
            i += 1;
            arr.swap(i as usize, j);
        }
    }
    arr.swap((i + 1) as usize, high);
    (i + 1) as usize
}

pub(crate) fn quicksort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot_index = partitioning(arr, low, high);
        if pivot_index > 0 {
            quicksort(arr, low, pivot_index - 1);
        }
        quicksort(arr, pivot_index + 1, high);
    }
}