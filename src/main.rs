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
