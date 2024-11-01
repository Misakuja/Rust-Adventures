
mod functions;
use crate::functions::quicksort;

fn main() {
    let mut quicksort_array: [i32; 10] = [1, 5, 6, 7, 8, 9, 2, 3, 6, 11];

    println!("Before quicksort: {:?}", quicksort_array);

    let len = quicksort_array.len();
    quicksort(&mut quicksort_array, 0, len - 1);

    println!("After quicksort: {:?}", quicksort_array);
}
