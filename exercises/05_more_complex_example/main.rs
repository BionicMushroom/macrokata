////////// DO NOT CHANGE BELOW HERE /////////
// This function should be called by the `show_output!()` macro
#[derive(Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn show(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

////////// DO NOT CHANGE ABOVE HERE /////////

// Create `for_2d!` macro here.
macro_rules! for_2d {
    ($row_i:ident <$row_t:ty> in $row_e:expr, $col_i:ident <$col_t:ty> in $col_e:expr, $code:block) => {
        for $row_i in $row_e {
            let $row_i: $row_t = $row_i;
            for $col_i in $col_e {
                let $col_i: $col_t = $col_i;
                $code
            }
        }
    };
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    for_2d!(row <i32> in 1..5, col <i32> in 2..7, {
        (Coordinate {x: col, y: row}).show()
    });

    let values = [1, 3, 5];

    for_2d!(x <u16> in values, y <u16> in values, {
        (Coordinate {x: x.into(), y: y.into()}).show()
    });
}
