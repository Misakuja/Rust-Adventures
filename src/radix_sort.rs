
fn find_largest_element(arr: &mut [i32]) -> i32 {
    let mut largest_element: i32 = arr[0];

    for &value in arr.iter() {
        if largest_element < value {
            largest_element = value;
        }
    }
    largest_element
}
fn counting_sort(arr: &mut [i32], digit_place: i32) {
    let length = arr.len();

    let mut output_array: [i32; 10] = [0; 10];
    let mut count_array: [i32; 10] = [0; 10];

    for &number in arr.iter() {
        let digit = ((number / digit_place) % 10) as usize;
        count_array[digit] += 1;
    }

    for i in 1..10 {
        count_array[i] += count_array[i - 1];
    }

    for i in (0..length).rev() {
        let digit = (arr[i]  / digit_place) % 10;
        output_array[count_array[digit as usize] as usize - 1]  = arr[i];
        count_array[digit as usize] -= 1;
    }

    for i in 0..length {
        arr[i] = output_array[i];
    }
}

fn radix_sort(arr: &mut [i32]) {
    let largest_element:i32 = find_largest_element(arr);
    let mut digit_place = 1;

    while(largest_element / digit_place) > 0 {
        counting_sort(arr, digit_place);
        digit_place *= 10;
    }
}

#[test]
fn test_radix_sort() {
    let mut radix_sort_array: [i32; 10] = [123, 513, 65, 72, 8, 91, 2123, 345, 63, 11];
    let expected: [i32; 10] = [8, 11, 63, 65, 72, 91, 123, 345, 513, 2123];

    println!("Before quicksort: {:?}", radix_sort_array);

    radix_sort(&mut radix_sort_array);

    println!("After quicksort: {:?}", radix_sort_array);

    assert_eq!(radix_sort_array, expected)
}

fn main() {}