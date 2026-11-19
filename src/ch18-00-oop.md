# object-oriented programming-এর বৈশিষ্ট্য {#object-oriented-programming-features}

<!-- Old headings. Do not remove or links may break. -->

<a id="object-oriented-programming-features-of-rust"></a>

object-oriented programming (OOP, অবজেক্টভিত্তিক প্রোগ্রামিং) হলো প্রোগ্রাম সাজানোর একটি উপায়। প্রোগ্রামিংয়ের ধারণা হিসেবে object প্রথম আনা হয় 1960-এর দশকে Simula প্রোগ্রামিং ভাষায়।
সেই object-গুলো Alan Kay-এর প্রোগ্রামিং স্থাপত্যকে প্রভাবিত করেছিল, যেখানে object-গুলো একে অপরকে message পাঠায়। এই স্থাপত্য বর্ণনা করতে তিনি 1967 সালে _object-oriented programming_
পরিভাষাটি তৈরি করেন। OOP কী তা নিয়ে অনেক প্রতিদ্বন্দ্বী সংজ্ঞা আছে, আর এর কিছু সংজ্ঞা অনুযায়ী Rust object-oriented, আবার অন্যগুলো অনুযায়ী নয়। এই অধ্যায়ে আমরা এমন কিছু বৈশিষ্ট্য দেখব, যা
সাধারণত object-oriented বলে বিবেচিত, আর সেই বৈশিষ্ট্যগুলো idiomatic Rust-এ কীভাবে রূপ নেয়। তারপর দেখাব কীভাবে Rust-এ একটি object-oriented design pattern implement করা যায়, এবং আলোচনা
করব এটা করার সঙ্গে Rust-এর কিছু শক্তি কাজে লাগিয়ে সমাধান implement করার তুলনায় সুবিধা-অসুবিধা কী।
