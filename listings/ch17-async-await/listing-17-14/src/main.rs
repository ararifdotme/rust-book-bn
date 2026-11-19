extern crate trpl; // required for mdbook test

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // পরে এখানে `slow` call করব
    });
}

// ANCHOR: slow
fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
// ANCHOR_END: slow
