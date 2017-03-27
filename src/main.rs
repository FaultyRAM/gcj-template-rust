extern crate gcj_helper;

use gcj_helper::TestEngine;
use std::io::{Read, Write};

fn main() {
    TestEngine::new("./foo.in", "./foo.out").run(|input, output| {
        // Your code goes here.
    });
}
