use std::cmp::max;

fn dynamic_program(x: &[char], n : usize, y: &[char], m : usize) -> i32 {
    let mut s = vec![vec![0; m + 1]; n + 1];

    for i in 1..=n {
        for j in 1..=m {
            if(x[i - 1]) != y[j - 1] {
                s[i][j] = max(s[i][j - 1], s[i - 1][j]);
            } else {
                s[i][j] = s[i - 1][j - 1] + 1;
            }
        }
    }
    s[n][m]
}
#[test]
fn test_dynamic_program() {
    let x: [char; 11] = ['A', 'B', 'R', 'A', 'C', 'A', 'D', 'A', 'B', 'R', 'A'];
    let y: [char; 7] = ['B', 'A', 'R', 'N', 'A', 'B', 'A'];

    let x_length = x.len();
    let y_length = y.len();

    let longest_substring: i32 = dynamic_program(&x, x_length, &y, y_length);

    assert_eq!(longest_substring, 5)
}

fn main() {}