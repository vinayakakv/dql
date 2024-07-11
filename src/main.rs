mod dql;
mod env;

use dql::parse_dql;
use std::{fs, io::Read};

fn main() {
    let mut buffer = String::new();
    let _ = fs::File::open("test.dql")
        .unwrap()
        .read_to_string(&mut buffer);
    let result = parse_dql(&buffer);
    dbg!(result);
}
