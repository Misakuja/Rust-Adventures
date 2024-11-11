fn merge(arr: &mut [i32], low: usize, mid: usize, high: usize) {
    let mut temp_arr: Vec<i32> = Vec::with_capacity(high - low + 1);

    let mut i: usize = low;
    let mut j: usize = mid + 1;

    while i <= mid && j <= high { //combine the two subarrays
        if arr[i] <= arr[j] {
            temp_arr.push(arr[i]);
            i += 1;
        } else {
            temp_arr.push(arr[j]);
            j += 1;
        }
    }

    while i <= mid { //leftover elements in first subarray
        temp_arr.push(arr[i]);
        i += 1;
    }

    while j <= high { //leftover elements in second subarray
        temp_arr.push(arr[j]);
        j += 1;
    }

    for (k, &val) in temp_arr.iter().enumerate() { //copy elements to original array
        arr[low + k] = val;
    }
}

fn mergesort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let mid: usize = (low + high) / 2;

        mergesort(arr, low, mid); // left half
        mergesort(arr, mid + 1, high); // right half
        merge(arr, low, mid, high); // merge
    }
}

#[test]
fn test_merge_sort() {
    let mut mergesort_array: [i32; 8] = [1, 2, 5, 6, 10, 22, 34, 12];
    let expected: [i32; 8] = [1, 2, 5, 6, 10, 12, 22, 34];

    println!("Before mergeSort: {:?}", mergesort_array);

    let len = mergesort_array.len();
    mergesort(&mut mergesort_array, 0, len - 1);

    println!("After mergeSort: {:?}", mergesort_array);

    assert_eq!(mergesort_array, expected)
}

fn main() {
}
