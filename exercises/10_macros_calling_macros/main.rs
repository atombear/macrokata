////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;

fn print_pair<K: std::fmt::Debug, V: std::fmt::Debug>(pair: (K, V)) {
    println!("{pair:#?}");
}
fn print_hashmap<K: std::fmt::Debug, V: std::fmt::Debug>(hashmap: &HashMap<K, V>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: Create a `pair!()` macro.

macro_rules! pair {
    ($k:expr => $v:expr) => {
        ($k, $v)
    }
}

// TODO: Create a `hashmap!()` macro that uses the `pair!()` macro.
macro_rules! hashmap {
    ($($k:expr => $v:expr),+ $(,)?) => {
        HashMap::from([$(pair!($k => $v)),+])
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let pair = pair!('a' => 1);

    print_pair(pair);

    let value = "value";

    let my_hashmap = hashmap!(
        "hash" => "map",
        "Key" => value,
    );

    print_hashmap(&my_hashmap);
}
