fn main() {
    // ANCHOR: here
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // লক্ষ করুন, s1 এখানে move হয়ে গেছে এবং আর ব্যবহার করা যাবে না
    // ANCHOR_END: here
}
