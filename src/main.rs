// https://crates.io/crates/gcj-helper
// https://github.com/FaultyRAM/gcj-helper-rs
extern crate gcj_helper;

use gcj_helper::TestEngine;

fn main() {
    TestEngine::new("./foo.in", "./foo.out").run(
        |input| {
            // Your parser code goes here.
        },
        |data| {
            // Your solver code goes here.
        },
    );
}
