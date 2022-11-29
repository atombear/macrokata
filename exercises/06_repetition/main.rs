////////// DO NOT CHANGE BELOW HERE /////////
fn print_success() {
    println!("Yay, the if statement worked.");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `if_any!()` macro.

macro_rules! if_any {
    ($($b:expr),+; $success_action:block) => {
        if ($($b)||+) $success_action
        // {
        //     let mut bool_vec: Vec<bool> = vec![];
        //     $(bool_vec.push($b));+;
        //     if bool_vec.iter().fold(false, |x, y| x || *y) {
        //         $success_action;
        //     }
        // }
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    if_any!(false, 0 == 1, true; {
        print_success();
    })
}
