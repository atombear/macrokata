////////// DO NOT CHANGE BELOW HERE /////////
use std::collections::HashMap;

fn print_hashmap(hashmap: &HashMap<&str, &str>) {
    println!("{hashmap:#?}");
}
////////// DO NOT CHANGE ABOVE HERE /////////

// TODO: create `hashmap!()` macro.

// optional trailing comma with $(,)?
macro_rules! hashmap {
    ($($key:expr => $value:expr),*$(,)?) => {
        {
            let mut ret_dict = HashMap::new();
            $(ret_dict.insert($key, $value));+;
            ret_dict
        }
    }
}

////////// DO NOT CHANGE BELOW HERE /////////

fn main() {
    let value = "my_string";
    let my_hashmap = hashmap!(
        "hash" => "map",
        "Key" => value,
    );

    print_hashmap(&my_hashmap);
}
