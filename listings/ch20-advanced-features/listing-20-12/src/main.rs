// ANCHOR: here
unsafe trait Foo {
    // method-গুলো এখানে বসবে
}

unsafe impl Foo for i32 {
    // method-গুলোর implementation এখানে বসবে
}
// ANCHOR_END: here

fn main() {}
