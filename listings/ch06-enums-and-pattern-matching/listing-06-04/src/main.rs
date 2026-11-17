// ANCHOR: here
#[derive(Debug)] // যাতে একটু পরেই অবস্থাটি দেখতে পারি
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// ANCHOR_END: here

fn main() {}
