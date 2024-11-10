fn binary_search(arr: &mut [i32], l: usize, h: usize, x: i32) -> i32 {
    if h >= l {
        let m: usize = (l + h) / 2;

        if arr[m] == x {
            return m as i32;
        }
        if arr[m] > x {
            return binary_search(arr, l, m - 1, x)
        }
        else if arr[m] < x {
            return binary_search(arr, m + 1, h, x)
        }
    }
    println!("No such element in array");
    -1
}

#[test]
fn test_binary_search() {
    let mut binary_search_array: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let x = 4;
    let len = binary_search_array.len();
    let expected: i32 = 3;


    let found_num: i32 = binary_search(&mut binary_search_array, 0, len - 1, x);

    println!("X value is at index: {:?}", found_num);

    assert_eq!(found_num, expected)
}

fn main() {}