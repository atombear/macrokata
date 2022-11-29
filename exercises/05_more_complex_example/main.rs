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

// TODO: Create `for_2d!` macro here.

macro_rules! for_2d {
    (
        $row:ident <$row_t:ty> in $range_row:expr,
        $col:ident <$col_t:ty> in $range_col:expr,
        $loop_body:expr) => {
        for $row in $range_row {
            let $row: $row_t = $row;
            for $col in $range_col {
                let $col: $col_t = $col;
                $loop_body;
            }
        }
    }
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
