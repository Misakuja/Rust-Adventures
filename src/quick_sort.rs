fn partitioning(arr: &mut [i32], low: usize, high: usize) -> usize {
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

fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot_index = partitioning(arr, low, high);
        if pivot_index > 0 {
            quick_sort(arr, low, pivot_index - 1);
        }
        quick_sort(arr, pivot_index + 1, high);
    }
}

#[test]
fn test_quick_sort() {
    let mut quicksort_array: [i32; 10] = [1, 5, 6, 7, 8, 9, 2, 3, 6, 11];
    let expected: [i32; 10] = [1, 2, 3, 5, 6, 6, 7, 8, 9, 11];

    println!("Before quicksort: {:?}", quicksort_array);

    let len = quicksort_array.len();
    quick_sort(&mut quicksort_array, 0, len - 1);

    println!("After quicksort: {:?}", quicksort_array);

    assert_eq!(quicksort_array, expected)
}

fn main() {}