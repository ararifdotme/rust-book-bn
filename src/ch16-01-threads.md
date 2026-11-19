## একসঙ্গে কোড চালাতে thread ব্যবহার {#using-threads-to-run-code-simultaneously}

বর্তমানের বেশিরভাগ operating system-এ চালানো একটি প্রোগ্রামের কোড একটি _process_ (চলমান প্রোগ্রাম)-এর ভেতরে চলে, আর operating system একসঙ্গে একাধিক process
সামলায়। একটি প্রোগ্রামের ভেতরেও এমন স্বাধীন অংশ থাকতে পারে যা একসঙ্গে চলে। এই স্বাধীন অংশগুলো চালানোর বৈশিষ্ট্যকে বলা হয় _threads_ (thread)। যেমন, একটি web
server-এর একাধিক thread থাকতে পারে, যাতে এটি একই সময়ে একাধিক request-এর উত্তর দিতে পারে।

একই সময়ে একাধিক কাজ চালাতে আপনার প্রোগ্রামের হিসাবকে একাধিক thread-এ ভাগ করলে performance বাড়তে পারে, তবে জটিলতাও বাড়ে। thread-গুলো একসঙ্গে চলতে পারে বলে
বিভিন্ন thread-এ থাকা আপনার কোডের অংশগুলো কোন ক্রমে চলবে তার কোনো স্বাভাবিক নিশ্চয়তা নেই। এতে নানা সমস্যা হতে পারে, যেমন:

- race condition, যেখানে thread-গুলো অসংগত ক্রমে ডেটা বা resource-এ প্রবেশ করে
- deadlock (অচলাবস্থা), যেখানে দুটি thread একে অপরের জন্য অপেক্ষা করে, ফলে দুটির কোনোটিই এগোতে পারে না
- এমন বাগ, যা শুধু নির্দিষ্ট পরিস্থিতিতে ঘটে এবং নির্ভরযোগ্যভাবে আবার ঘটানো ও ঠিক করা কঠিন

Rust thread ব্যবহারের নেতিবাচক প্রভাব কমানোর চেষ্টা করে, তবে multithreaded প্রেক্ষাপটে প্রোগ্রামিং করতে এখনো সতর্ক চিন্তা লাগে এবং একটি thread-এ চলা প্রোগ্রামের চেয়ে ভিন্ন
কোড-গঠন দরকার হয়।

প্রোগ্রামিং ভাষাগুলো কয়েকটি ভিন্ন উপায়ে thread implement করে, আর অনেক operating system এমন একটি API দেয় যা নতুন thread তৈরির জন্য প্রোগ্রামিং ভাষা call করতে পারে।
Rust standard library thread implementation-এর _1:1_ model ব্যবহার করে, যেখানে ভাষার প্রতিটি thread-এর জন্য প্রোগ্রাম operating system-এর একটি করে thread ব্যবহার করে।
এমন crate-ও আছে যা threading-এর অন্য model implement করে, যেগুলো 1:1 model-এর তুলনায় ভিন্ন আপস করে। (Rust-এর async system, যা পরের অধ্যায়ে দেখব, concurrency-র
আরেকটি পদ্ধতিও দেয়।)

### `spawn` দিয়ে নতুন thread তৈরি করা {#creating-a-new-thread-with-spawn}

নতুন thread তৈরি করতে আমরা `thread::spawn` function call করি এবং তাকে এমন একটি closure পাঠাই (closure নিয়ে অধ্যায় 13-এ কথা বলেছি), যাতে নতুন thread-এ চালাতে চাওয়া
কোড থাকে। Listing 16-1-এর উদাহরণ একটি main thread থেকে কিছু লেখা আর একটি নতুন thread থেকে অন্য লেখা print করে।

<Listing number="16-1" file-name="src/main.rs" caption="main thread যখন অন্য কিছু print করছে, তখন একটি জিনিস print করতে নতুন thread তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

</Listing>

খেয়াল করুন, একটি Rust প্রোগ্রামের main thread শেষ হলে চালু করা সব thread বন্ধ হয়ে যায়, সেগুলোর চলা শেষ হোক বা না হোক। এই প্রোগ্রামের output প্রতিবার একটু ভিন্ন হতে পারে,
তবে দেখতে নিচের মতো হবে:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

`thread::sleep`-এর call একটি thread-কে অল্প সময়ের জন্য চলা থামাতে বাধ্য করে, ফলে অন্য একটি thread চলতে পারে। thread-গুলো সম্ভবত পালা করে চলবে, তবে তার নিশ্চয়তা নেই:
এটি নির্ভর করে আপনার operating system thread-গুলোকে কীভাবে সময় বণ্টন করে তার ওপর। এই বার চালানোয় main thread আগে print করেছে, যদিও কোডে চালু করা thread-এর print
statement আগে আছে। আর চালু করা thread-কে `i` `9` না হওয়া পর্যন্ত print করতে বললেও main thread বন্ধ হওয়ার আগে এটি শুধু `5` পর্যন্ত পৌঁছেছে।

এই কোড চালিয়ে যদি শুধু main thread-এর output দেখেন, বা কোনো মিশ্রণ না দেখেন, তাহলে range-গুলোর সংখ্যা বাড়িয়ে দেখুন, যাতে operating system-এর thread বদলানোর আরও
সুযোগ তৈরি হয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="waiting-for-all-threads-to-finish-using-join-handles"></a>

### সব thread শেষ হওয়ার জন্য অপেক্ষা করা {#waiting-for-all-threads-to-finish}

Listing 16-1-এর কোড main thread শেষ হওয়ার কারণে বেশিরভাগ সময় চালু করা thread-কে শুধু অসময়ে থামিয়েই দেয় না, বরং thread-গুলো কোন ক্রমে চলবে তার কোনো নিশ্চয়তা না
থাকায় আমরা এটাও নিশ্চিত করতে পারি না যে চালু করা thread আদৌ চলবে!

`thread::spawn`-এর return মান একটি variable-এ রেখে চালু করা thread না চলা বা অসময়ে শেষ হওয়ার সমস্যা ঠিক করা যায়। `thread::spawn`-এর return type হলো `JoinHandle<T>`।
`JoinHandle<T>` একটি owned মান, যার ওপর `join` method call করলে এটি তার thread শেষ হওয়ার জন্য অপেক্ষা করবে। Listing 16-2-এ দেখানো হয়েছে Listing 16-1-এ তৈরি
thread-এর `JoinHandle<T>` কীভাবে ব্যবহার করতে হয়, আর `main` বেরিয়ে যাওয়ার আগে চালু করা thread যাতে শেষ হয় তা নিশ্চিত করতে কীভাবে `join` call করতে হয়।

<Listing number="16-2" file-name="src/main.rs" caption="thread যে শেষ পর্যন্ত চলবে তা নিশ্চিত করতে `thread::spawn` থেকে একটি `JoinHandle<T>` রেখে দেওয়া">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

</Listing>

handle-এর ওপর `join` call করলে বর্তমানে চলা thread block হয়ে থাকে, যতক্ষণ না handle যে thread বোঝায় সেটি শেষ হয়। একটি thread-কে _Blocking_ (block করা) মানে হলো
thread-টিকে কাজ করা বা বেরিয়ে যাওয়া থেকে আটকে রাখা। `join` call-টি main thread-এর `for` loop-এর পরে রেখেছি বলে Listing 16-2 চালালে এমন output আসার কথা:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

দুটি thread পালা করে চলতে থাকে, কিন্তু `handle.join()` call-এর কারণে main thread অপেক্ষা করে এবং চালু করা thread শেষ না হওয়া পর্যন্ত শেষ হয় না।

কিন্তু চলুন দেখি `handle.join()`-কে `main`-এর `for` loop-এর আগে সরিয়ে নিলে কী ঘটে, এভাবে:

<Listing file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

</Listing>

main thread চালু করা thread শেষ হওয়ার জন্য অপেক্ষা করবে, তারপর নিজের `for` loop চালাবে, তাই output আর মিশ্রিত থাকবে না, যেমনটি এখানে দেখানো হয়েছে:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```

`join` কোথায় call করা হচ্ছে, এমন ছোট খুঁটিনাটিও ঠিক করতে পারে আপনার thread-গুলো একই সময়ে চলবে কি না।

### thread-এর সঙ্গে `move` closure ব্যবহার {#using-move-closures-with-threads}

`thread::spawn`-এ পাঠানো closure-এর সঙ্গে আমরা প্রায়ই `move` keyword ব্যবহার করব, কারণ তখন closure environment থেকে যে মানগুলো ব্যবহার করে সেগুলোর ownership নিয়ে নেবে,
ফলে সেই মানগুলোর ownership এক thread থেকে আরেক thread-এ হস্তান্তর হবে। অধ্যায় 13-এর [“reference capture করা বা ownership সরিয়ে নেওয়া”][capture]<!-- ignore
-->-এ আমরা closure-এর প্রেক্ষাপটে `move` নিয়ে আলোচনা করেছি। এখন `move` আর `thread::spawn`-এর পারস্পরিক ক্রিয়ার দিকে বেশি মনোযোগ দেব।

Listing 16-1-এ খেয়াল করুন, `thread::spawn`-এ পাঠানো closure কোনো argument নেয় না: চালু করা thread-এর কোডে আমরা main thread-এর কোনো ডেটা ব্যবহার করছি না। চালু করা
thread-এ main thread-এর ডেটা ব্যবহার করতে চালু করা thread-এর closure-কে তার দরকারি মানগুলো capture করতে হবে। Listing 16-3-এ main thread-এ একটি vector তৈরি করে চালু করা
thread-এ সেটি ব্যবহারের একটি চেষ্টা দেখানো হয়েছে। তবে একটু পরেই দেখবেন, এটি এখনো কাজ করবে না।

<Listing number="16-3" file-name="src/main.rs" caption="main thread-এর তৈরি একটি vector অন্য thread-এ ব্যবহারের চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

</Listing>

closure-টি `v` ব্যবহার করে, তাই এটি `v` capture করবে এবং একে closure-এর environment-এর অংশ বানাবে। `thread::spawn` যেহেতু এই closure একটি নতুন thread-এ চালায়, তাই
সেই নতুন thread-এর ভেতরে `v` পাওয়ার কথা। কিন্তু এই উদাহরণ compile করলে এই error পাই:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

Rust `v` কীভাবে capture করবে তা _অনুমান করে_, আর `println!`-এর যেহেতু শুধু `v`-এর একটি reference দরকার, তাই closure `v` borrow করার চেষ্টা করে। কিন্তু একটি সমস্যা আছে:
চালু করা thread কতক্ষণ চলবে তা Rust বলতে পারে না, তাই এটি জানে না `v`-এর reference সবসময় বৈধ থাকবে কি না।

Listing 16-4-এ এমন একটি পরিস্থিতি দেওয়া হয়েছে, যেখানে `v`-এর reference অবৈধ হওয়ার সম্ভাবনা বেশি।

<Listing number="16-4" file-name="src/main.rs" caption="একটি thread, যার closure এমন main thread থেকে `v`-এর reference capture করার চেষ্টা করে, যে main thread `v` drop করে দেয়">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

</Listing>

Rust যদি আমাদের এই কোড চালাতে দিত, তাহলে এমন সম্ভাবনা আছে যে চালু করা thread একটুও না চলে সঙ্গে সঙ্গে পটভূমিতে চলে যেত। চালু করা thread-এর ভেতরে `v`-এর একটি
reference আছে, কিন্তু main thread অধ্যায় 15-এ আলোচিত `drop` function ব্যবহার করে সঙ্গে সঙ্গে `v` drop করে দেয়। তারপর চালু করা thread চলতে শুরু করলে `v` আর বৈধ থাকে
না, তাই এর reference-ও অবৈধ। হায় হায়!

Listing 16-3-এর compiler error ঠিক করতে আমরা error message-এর পরামর্শ ব্যবহার করতে পারি:

<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ++++
```

closure-এর আগে `move` keyword যোগ করে আমরা closure-কে তার ব্যবহৃত মানগুলোর ownership নিতে বাধ্য করি, Rust-কে মানগুলো borrow করা উচিত বলে অনুমান করতে না দিয়ে। Listing
16-5-এ দেখানো Listing 16-3-এর পরিবর্তিত রূপ compile হবে এবং আমাদের উদ্দেশ্য অনুযায়ী চলবে।

<Listing number="16-5" file-name="src/main.rs" caption="closure-কে তার ব্যবহৃত মানগুলোর ownership নিতে বাধ্য করতে `move` keyword ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

</Listing>

Listing 16-4-এর যে কোডে main thread `drop` call করেছিল, সেটি ঠিক করতে আমরা হয়তো একই কাজ, অর্থাৎ একটি `move` closure ব্যবহার করতে চাইব। কিন্তু এই সমাধান কাজ করবে না,
কারণ Listing 16-4 যা করার চেষ্টা করছে তা ভিন্ন একটি কারণে অনুমোদিত নয়। closure-এ `move` যোগ করলে আমরা `v`-কে closure-এর environment-এ সরিয়ে নিতাম, আর তখন main
thread-এ এর ওপর আর `drop` call করতে পারতাম না। এর বদলে এই compiler error পেতাম:

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

Rust-এর ownership-এর নিয়ম আবারও আমাদের বাঁচিয়ে দিল! Listing 16-3-এর কোড থেকে error পেয়েছিলাম, কারণ Rust রক্ষণশীল ছিল এবং thread-এর জন্য শুধু `v` borrow করছিল,
অর্থাৎ তাত্ত্বিকভাবে main thread চালু করা thread-এর reference অবৈধ করে দিতে পারত। Rust-কে `v`-এর ownership চালু করা thread-এ সরাতে বলে আমরা Rust-কে নিশ্চয়তা দিচ্ছি
যে main thread আর `v` ব্যবহার করবে না। Listing 16-4 একইভাবে বদলালে main thread-এ `v` ব্যবহারের চেষ্টা করার সময় আমরা ownership-এর নিয়ম ভঙ্গ করছি। `move` keyword Rust-এর
রক্ষণশীল default, অর্থাৎ borrow করাকে override করে; এটি আমাদের ownership-এর নিয়ম ভঙ্গ করতে দেয় না।

thread কী এবং thread API কোন method দেয় তা দেখা হলো, এবার এমন কিছু পরিস্থিতি দেখা যাক যেখানে thread ব্যবহার করা যায়।

{{#quiz ../quizzes/ch16-01-threads.toml}}

[capture]: ch13-01-closures.html#capturing-references-or-moving-ownership
