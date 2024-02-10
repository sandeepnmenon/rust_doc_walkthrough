// While bringing in enums and structs to scope it is best to specify the full path
// and use the name of the struct directly without mentioning the module
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::cmp::Ordering;
use std::io;
// can also be written as
// use std::{cmp::Ordering, io}

// Glob can also be used
use std::collections::*;
