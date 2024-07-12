mod functions;
use functions::Calculator;

fn main() {
    let calc = Calculator::new();

    calc.menu();
}