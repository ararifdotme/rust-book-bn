fn main() {
    // ANCHOR: here
    let data = "initial contents";

    let s = data.to_string();

    // method-টি সরাসরি একটি literal-এর ওপরও কাজ করে:
    let s = "initial contents".to_string();
    // ANCHOR_END: here
}
