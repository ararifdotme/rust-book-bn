fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // এই পর্যায়ে field_name আর field_value অবৈধ, সেগুলো ব্যবহারের চেষ্টা করে
    // দেখুন কী compiler error পান!
    // ANCHOR_END: here
}
