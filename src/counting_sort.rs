
fn find_largest_element(arr: &mut [i32]) -> i32 {
    let mut largest_element: i32 = arr[0];

    for &value in arr.iter() {
        if largest_element < value {
            largest_element = value;
        }
    }
    largest_element
}
fn counting_sort(arr: &mut [i32]) {
    let length = arr.len();
    let largest_element = find_largest_element(arr);

    let max_size = (largest_element + 1) as usize;
    let mut count_array: [i32; 12] = [0; 12];
    let mut finished_arr: [i32; 12] = [0; 12];

    for &value in arr.iter() {
        count_array[value as usize] += 1;
    }

    for i in 1..max_size {
        count_array[i] += count_array[i - 1];
    }

    for i in (0..length).rev() {
        finished_arr[count_array[arr[i] as usize] as usize - 1] = arr[i];
        count_array[arr[i] as usize] -= 1;
    }

    for i in 0..length {
        arr[i] = finished_arr[i];
    }
}

#[test]
fn test_counting_sort() {
    let mut counting_sort_array: [i32; 10] = [1, 5, 6, 7, 8, 9, 2, 3, 6, 11];
    let expected: [i32; 10] = [1, 2, 3, 5, 6, 6, 7, 8, 9, 11];

    println!("Before quicksort: {:?}", counting_sort_array);

    counting_sort(&mut counting_sort_array);

    println!("After quicksort: {:?}", counting_sort_array);

    assert_eq!(counting_sort_array, expected)
}

fn main() {}