mod functions;
use functions::Calculator;

fn main() {
    let mut calc = Calculator::new(0.0, 0.0);

    calc.read_parse_string();
}
