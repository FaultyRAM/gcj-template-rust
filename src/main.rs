extern crate gcj_helper;

use gcj_helper::TestEngine;

fn main() {
    TestEngine::new("./foo.in", "./foo.out").run(|io_helper| {
        // Your code goes here.
    });
}
