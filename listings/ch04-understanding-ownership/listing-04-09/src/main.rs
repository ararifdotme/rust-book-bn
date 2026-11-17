// ANCHOR: here
fn first_word(s: &str) -> &str {
    // ANCHOR_END: here
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: usage
fn main() {
    let my_string = String::from("hello world");

    // `first_word` `String`-এর slice-এর ওপর কাজ করে, আংশিক হোক বা পুরো।
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` `String`-এর reference-এর ওপরও কাজ করে, যা `String`-এর
    // পুরো slice-এর সমতুল্য।
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` string literal-এর slice-এর ওপর কাজ করে, আংশিক হোক বা
    // পুরো।
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // string literal যেহেতু আগে থেকেই string slice, তাই
    // slice syntax ছাড়াও এটি কাজ করে!
    let word = first_word(my_string_literal);
}
// ANCHOR_END: usage
