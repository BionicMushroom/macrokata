////////// DO NOT CHANGE BELOW HERE /////////
fn print_success() {
    println!("Yay, the if statement worked.");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// create `if_any!()` macro.
macro_rules! if_any {
    ($($e:expr),+; $code:block) => {
        if $($e)||+ $code
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    if_any!(false, 0 == 1, true; {
        print_success();
    })
}
