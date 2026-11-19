## একটি ফাইল পড়া {#reading-a-file}

এবার আমরা `file_path` argument-এ উল্লেখ করা ফাইলটি পড়ার কার্যক্ষমতা যোগ করব। প্রথমে এটি test করার জন্য একটি নমুনা ফাইল
দরকার: আমরা এমন একটি ফাইল ব্যবহার করব যাতে কয়েক লাইনে অল্প কিছু লেখা আছে এবং কিছু শব্দ বারবার এসেছে। Listing 12-3-এ
Emily Dickinson-এর একটি কবিতা আছে, যা এ কাজে বেশ ভালো চলবে! আপনার project-এর মূল স্তরে _poem.txt_ নামে একটি ফাইল তৈরি করুন,
আর তাতে “I’m Nobody!
Who are you?” কবিতাটি লিখুন।

<Listing number="12-3" file-name="poem.txt" caption="Emily Dickinson-এর একটি কবিতা ভালো test case হয়।">

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

</Listing>

লেখাটি জায়গামতো রেখে _src/main.rs_ সম্পাদনা করুন এবং ফাইল পড়ার কোড যোগ করুন, যেমনটি Listing 12-4-এ দেখানো হয়েছে।

<Listing number="12-4" file-name="src/main.rs" caption="দ্বিতীয় argument-এ উল্লেখ করা ফাইলের বিষয়বস্তু পড়া">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

</Listing>

প্রথমে একটি `use` statement দিয়ে standard library-র প্রাসঙ্গিক অংশ আনি: ফাইল সামলাতে আমাদের `std::fs` দরকার।

`main`-এ নতুন statement `fs::read_to_string` `file_path` নেয়, সেই ফাইলটি খোলে, এবং `std::io::Result<String>` type-এর একটি মান
return করে, যাতে ফাইলের বিষয়বস্তু থাকে।

এরপর আবার একটি সাময়িক `println!` statement যোগ করি, যা ফাইল পড়ার পর `contents`-এর মান print করে, যাতে যাচাই করতে পারি যে
প্রোগ্রাম এ পর্যন্ত কাজ করছে।

চলুন প্রথম command line argument হিসেবে যেকোনো string (কারণ খোঁজার অংশটি এখনো implement করিনি) আর দ্বিতীয় argument হিসেবে
_poem.txt_ ফাইল দিয়ে এই কোড চালাই:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

চমৎকার! কোডটি ফাইলের বিষয়বস্তু পড়ে print করেছে। কিন্তু কোডে কয়েকটি ত্রুটি আছে। এই মুহূর্তে `main` function-এর একাধিক
দায়িত্ব: সাধারণত প্রতিটি function শুধু একটি ধারণার দায়িত্বে থাকলে function আরও পরিষ্কার হয় এবং রক্ষণাবেক্ষণ সহজ হয়। অন্য
সমস্যা হলো আমরা যতটা ভালোভাবে পারতাম ততটা ভালোভাবে error সামলাচ্ছি না। প্রোগ্রামটি এখনো ছোট, তাই এই ত্রুটিগুলো বড় সমস্যা
নয়, কিন্তু প্রোগ্রাম বড় হতে থাকলে এগুলো পরিচ্ছন্নভাবে ঠিক করা কঠিন হবে। প্রোগ্রাম তৈরির শুরুর দিকেই refactoring (আচরণ না
বদলে কোডের গঠন উন্নত করা) শুরু করা ভালো অভ্যাস, কারণ অল্প কোড refactor করা অনেক সহজ। এরপর আমরা সেটাই করব।
