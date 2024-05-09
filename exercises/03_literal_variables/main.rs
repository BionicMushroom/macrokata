////////// DO NOT CHANGE BELOW HERE /////////
fn print_result(num: i32) {
    println!("The result is {num}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// create `math!()` macro.
macro_rules! math {
    ($lhs:literal plus $rhs:literal) => {
        $lhs + $rhs
    };
    (square $arg:literal) => {
        $arg * $arg
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    print_result(math!(3 plus 5));
    print_result(math!(square 2));
}
