## shared-state concurrency {#shared-state-concurrency}

message passing concurrency সামলানোর একটি ভালো উপায়, তবে একমাত্র উপায় নয়। আরেকটি পদ্ধতি হলো একাধিক thread একই ভাগ করা ডেটায় প্রবেশ করা। Go ভাষার documentation-এর
স্লোগানের এই অংশটি আবার ভাবুন: “মেমোরি ভাগ করে যোগাযোগ করো না।”

মেমোরি ভাগ করে যোগাযোগ দেখতে কেমন হতো? তা ছাড়া message passing-এর অনুরাগীরা কেন মেমোরি ভাগ না করার ব্যাপারে সতর্ক করেন?

এক অর্থে যেকোনো প্রোগ্রামিং ভাষার channel একক ownership-এর মতো, কারণ একবার channel দিয়ে একটি মান পাঠিয়ে দিলে সেই মান আর ব্যবহার করা উচিত নয়। shared-memory concurrency
একাধিক ownership-এর মতো: একাধিক thread একই সময়ে একই মেমোরির জায়গায় প্রবেশ করতে পারে। অধ্যায় 15-এ যেমন দেখেছেন, যেখানে smart pointer একাধিক ownership সম্ভব করেছিল,
একাধিক ownership জটিলতা বাড়াতে পারে, কারণ এই ভিন্ন ভিন্ন owner-দের সামলাতে হয়। এই ব্যবস্থাপনা সঠিকভাবে করতে Rust-এর type system আর ownership-এর নিয়ম অনেক সাহায্য করে।
উদাহরণ হিসেবে চলুন mutex দেখি, যা ভাগ করা মেমোরির জন্য সবচেয়ে প্রচলিত concurrency primitive (concurrency-র মৌলিক গঠন)-গুলোর একটি।

<!-- Old headings. Do not remove or links may break. -->

<a id="using-mutexes-to-allow-access-to-data-from-one-thread-at-a-time"></a>

### mutex দিয়ে প্রবেশাধিকার নিয়ন্ত্রণ {#controlling-access-with-mutexes}

_Mutex_ হলো _mutual exclusion_ (পারস্পরিক বর্জন)-এর সংক্ষেপ, অর্থাৎ একটি mutex যেকোনো সময়ে শুধু একটি thread-কে কিছু ডেটায় প্রবেশ করতে দেয়। একটি mutex-এর ডেটায় প্রবেশ করতে
একটি thread-কে প্রথমে mutex-এর lock নিতে চেয়ে জানাতে হয় যে এটি প্রবেশ করতে চায়। _lock_ হলো mutex-এর অংশ এমন একটি data structure, যা হিসাব রাখে এই মুহূর্তে ডেটায়
একচেটিয়া প্রবেশাধিকার কার কাছে আছে। তাই বলা হয় mutex locking system-এর মাধ্যমে তার রাখা ডেটা _পাহারা দেয়_।

mutex ব্যবহার করা কঠিন বলে এর দুর্নাম আছে, কারণ আপনাকে দুটি নিয়ম মনে রাখতে হয়:

1. ডেটা ব্যবহারের আগে lock নেওয়ার চেষ্টা করতেই হবে।
2. mutex যে ডেটা পাহারা দেয় তা দিয়ে কাজ শেষ হলে ডেটা unlock করতেই হবে, যাতে অন্য thread-গুলো lock নিতে পারে।

mutex-এর একটি বাস্তব উপমা হিসেবে একটি সম্মেলনের প্যানেল আলোচনা কল্পনা করুন, যেখানে মাইক্রোফোন মাত্র একটি। একজন প্যানেলিস্ট কথা বলার আগে তাকে মাইক্রোফোন ব্যবহার করতে
চেয়ে অনুরোধ করতে বা ইঙ্গিত দিতে হয়। মাইক্রোফোন পেলে তিনি যতক্ষণ খুশি কথা বলতে পারেন, তারপর কথা বলতে চাওয়া পরের প্যানেলিস্টের হাতে মাইক্রোফোন তুলে দেন। কোনো প্যানেলিস্ট
কথা শেষ করে মাইক্রোফোন হস্তান্তর করতে ভুলে গেলে আর কেউ কথা বলতে পারে না। ভাগ করা মাইক্রোফোনের ব্যবস্থাপনায় গোলমাল হলে প্যানেল পরিকল্পনামতো চলবে না!

mutex-এর ব্যবস্থাপনা ঠিকঠাক করা অবিশ্বাস্য রকম কঠিন হতে পারে, এ কারণেই এত মানুষ channel নিয়ে উৎসাহী। তবে Rust-এর type system আর ownership-এর নিয়মের কল্যাণে আপনি
lock আর unlock করতে ভুল করতে পারবেন না।

#### `Mutex<T>`-এর API {#the-api-of-mutext}

mutex কীভাবে ব্যবহার করতে হয় তার উদাহরণ হিসেবে চলুন একটি single-threaded প্রেক্ষাপটে mutex ব্যবহার দিয়ে শুরু করি, যেমনটি Listing 16-12-এ দেখানো হয়েছে।

<Listing number="16-12" file-name="src/main.rs" caption="সরলতার জন্য একটি single-threaded প্রেক্ষাপটে `Mutex<T>`-এর API দেখা">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

</Listing>

অনেক type-এর মতোই আমরা associated function `new` দিয়ে একটি `Mutex<T>` তৈরি করি। mutex-এর ভেতরের ডেটায় প্রবেশ করতে `lock` method দিয়ে lock নিই। এই call বর্তমান thread
block করবে, ফলে lock পাওয়ার পালা না আসা পর্যন্ত এটি কোনো কাজ করতে পারবে না।

lock ধরে রাখা অন্য কোনো thread panic করলে `lock` call ব্যর্থ হতো। সেই ক্ষেত্রে কেউই আর কখনো lock পেত না, তাই আমরা `unwrap` করার পথ বেছে নিয়েছি, যাতে সেই পরিস্থিতিতে এই
thread panic করে।

lock নেওয়ার পর return মানটিকে, এই ক্ষেত্রে যার নাম `num`, ভেতরের ডেটার একটি mutable reference হিসেবে ব্যবহার করতে পারি। type system নিশ্চিত করে যে `m`-এর মান ব্যবহারের
আগে আমরা lock নিই। `m`-এর type হলো `Mutex<i32>`, `i32` নয়, তাই `i32` মান ব্যবহার করতে আমাদের `lock` call করতেই _হবে_। ভুলে যাওয়ার উপায় নেই; নইলে type system আমাদের
ভেতরের `i32`-এ প্রবেশ করতে দেবে না।

`lock` call `MutexGuard` নামের একটি type return করে, যা একটি `LockResult`-এ মোড়ানো, যেটি আমরা `unwrap` call দিয়ে সামলেছি। `MutexGuard` type আমাদের ভেতরের ডেটাকে নির্দেশ
করতে `Deref` implement করে; এই type-এর একটি `Drop` implementation-ও আছে, যা একটি `MutexGuard` scope-এর বাইরে গেলে স্বয়ংক্রিয়ভাবে lock ছেড়ে দেয়, যা ঘটে ভেতরের
scope-এর শেষে। ফলে lock ছাড়তে ভুলে গিয়ে অন্য thread-কে mutex ব্যবহারে বাধা দেওয়ার ঝুঁকি থাকে না, কারণ lock ছাড়া স্বয়ংক্রিয়ভাবে ঘটে।

lock drop করার পর আমরা mutex-এর মান print করে দেখতে পারি যে ভেতরের `i32`-কে `6`-এ বদলাতে পেরেছি।

<!-- Old headings. Do not remove or links may break. -->

<a id="sharing-a-mutext-between-multiple-threads"></a>

#### `Mutex<T>`-এ ভাগ করা প্রবেশাধিকার {#shared-access-to-mutext}

এবার `Mutex<T>` ব্যবহার করে একাধিক thread-এর মধ্যে একটি মান ভাগ করার চেষ্টা করি। আমরা 10টি thread চালু করব এবং প্রত্যেককে দিয়ে একটি counter-এর মান 1 করে বাড়াব, ফলে
counter 0 থেকে 10 হবে। Listing 16-13-এর উদাহরণে একটি compiler error থাকবে, আর সেই error ব্যবহার করে আমরা `Mutex<T>` ব্যবহার সম্পর্কে এবং Rust কীভাবে একে সঠিকভাবে
ব্যবহারে সাহায্য করে সে সম্পর্কে আরও শিখব।

<Listing number="16-13" file-name="src/main.rs" caption="দশটি thread, যার প্রত্যেকটি একটি `Mutex<T>`-এর পাহারায় থাকা counter বাড়ায়">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

</Listing>

Listing 16-12-এর মতোই আমরা একটি `Mutex<T>`-এর ভেতরে একটি `i32` রাখতে `counter` variable তৈরি করি। এরপর সংখ্যার একটি range-এর ওপর দিয়ে iterate করে 10টি thread তৈরি করি।
আমরা `thread::spawn` ব্যবহার করে সব thread-কে একই closure দিই: যা counter-কে thread-এ সরিয়ে নেয়, `lock` method call করে `Mutex<T>`-এর lock নেয়, তারপর mutex-এর মানের
সঙ্গে 1 যোগ করে। একটি thread তার closure চালানো শেষ করলে `num` scope-এর বাইরে যাবে এবং lock ছেড়ে দেবে, যাতে অন্য thread সেটি নিতে পারে।

main thread-এ আমরা সব join handle জড়ো করি। তারপর Listing
16-2-এর মতো প্রতিটি handle-এর ওপর `join` call করে নিশ্চিত হই যে সব thread শেষ হয়েছে। সেই মুহূর্তে main thread lock নেবে এবং এই প্রোগ্রামের ফল print করবে।

আমরা ইঙ্গিত দিয়েছিলাম যে এই উদাহরণ compile হবে না। চলুন দেখি কেন!

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

error message বলছে যে loop-এর আগের পুনরাবৃত্তিতে `counter` মানটি সরে গেছে। Rust আমাদের বলছে যে lock `counter`-এর ownership একাধিক thread-এ সরানো যায় না। অধ্যায় 15-এ
আলোচিত একাধিক ownership-এর পদ্ধতি দিয়ে compiler error-টি ঠিক করি।

#### একাধিক thread-এ একাধিক ownership {#multiple-ownership-with-multiple-threads}

অধ্যায় 15-এ আমরা smart pointer `Rc<T>` দিয়ে একটি reference-counted মান তৈরি করে একটি মানকে একাধিক owner দিয়েছিলাম। এখানেও তা-ই করে দেখি কী ঘটে। Listing 16-14-এ আমরা
`Mutex<T>`-কে `Rc<T>`-এ মুড়ে দেব এবং thread-এ ownership সরানোর আগে `Rc<T>` clone করব।

<Listing number="16-14" file-name="src/main.rs" caption="একাধিক thread-কে `Mutex<T>`-এর মালিক হতে দিতে `Rc<T>` ব্যবহারের চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

</Listing>

আবারও compile করে পাই... ভিন্ন error! compiler আমাদের অনেক কিছু শেখাচ্ছে:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

বাপরে, error message-টি খুবই শব্দবহুল! মনোযোগ দেওয়ার মতো গুরুত্বপূর্ণ অংশ এটি:
`` `Rc<Mutex<i32>>` cannot be sent between threads safely ``। compiler কারণটিও বলছে: `` the trait `Send` is not implemented for
`Rc<Mutex<i32>>` ``। `Send` নিয়ে পরের অংশে কথা বলব: এটি সেই trait-গুলোর একটি, যা নিশ্চিত করে যে thread-এর সঙ্গে আমরা যে type ব্যবহার করি তা concurrent পরিস্থিতিতে
ব্যবহারের উপযোগী।

দুর্ভাগ্যবশত, `Rc<T>` thread-এর মধ্যে ভাগ করা নিরাপদ নয়। `Rc<T>` যখন reference count সামলায়, তখন `clone`-এর প্রতিটি call-এ count বাড়ায় এবং প্রতিটি clone drop হলে count
কমায়। কিন্তু count-এর পরিবর্তন যাতে অন্য thread বাধাগ্রস্ত করতে না পারে, তা নিশ্চিত করতে এটি কোনো concurrency primitive ব্যবহার করে না। এতে count ভুল হতে পারে—এমন সূক্ষ্ম
বাগ, যা থেকে memory leak হতে পারে বা কাজ শেষ হওয়ার আগেই একটি মান drop হয়ে যেতে পারে। আমাদের দরকার এমন একটি type, যা হুবহু `Rc<T>`-এর মতো, কিন্তু reference count-এর
পরিবর্তন thread-safe উপায়ে করে।

#### `Arc<T>` দিয়ে atomic reference counting {#atomic-reference-counting-with-arct}

সৌভাগ্যক্রমে `Arc<T>` _সত্যিই_ `Rc<T>`-এর মতো এমন একটি type, যা concurrent পরিস্থিতিতে ব্যবহার করা নিরাপদ। _a_ মানে _atomic_ (অবিভাজ্যভাবে সম্পন্ন), অর্থাৎ এটি একটি
_atomically reference-counted_ type। atomic হলো আরেক ধরনের concurrency primitive, যা এখানে বিস্তারিত আলোচনা করব না: আরও বিস্তারিত জানতে standard library-র
[`std::sync::atomic`][atomic]<!-- ignore --> documentation দেখুন। এই মুহূর্তে শুধু জানলেই চলবে যে atomic primitive type-এর মতো কাজ করে, কিন্তু thread-এর মধ্যে ভাগ করা নিরাপদ।

তাহলে হয়তো ভাবছেন, সব primitive type কেন atomic নয়, আর standard library-র type-গুলো কেন default-ভাবে `Arc<T>` ব্যবহার করার মতো করে implement করা নয়। কারণ thread
safety-র জন্য performance-এর মাশুল দিতে হয়, যা আপনি শুধু সত্যিই দরকার হলে দিতে চাইবেন। আপনি যদি শুধু একটি thread-এর ভেতরে মানের ওপর operation চালান, তাহলে atomic-এর দেওয়া
নিশ্চয়তা প্রয়োগ করতে না হলে আপনার কোড আরও দ্রুত চলতে পারে।

চলুন আমাদের উদাহরণে ফিরি: `Arc<T>` আর `Rc<T>`-এর API একই, তাই `use` লাইন, `new` call আর `clone` call বদলে আমরা প্রোগ্রামটি ঠিক করি। Listing 16-15-এর কোড অবশেষে compile
হয়ে চলবে।

<Listing number="16-15" file-name="src/main.rs" caption="একাধিক thread-এর মধ্যে ownership ভাগ করতে পারার জন্য `Mutex<T>`-কে একটি `Arc<T>`-এ মোড়ানো">

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

</Listing>

এই কোড এই output print করবে:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Result: 10
```

পেরেছি! আমরা 0 থেকে 10 পর্যন্ত গুনেছি, যা হয়তো খুব একটা চমকপ্রদ মনে হবে না, তবে এটি আমাদের `Mutex<T>` আর thread safety সম্পর্কে অনেক কিছু শিখিয়েছে। শুধু একটি counter
বাড়ানোর চেয়ে আরও জটিল operation করতেও এই প্রোগ্রামের গঠন ব্যবহার করতে পারেন। এই কৌশল ব্যবহার করে একটি হিসাবকে স্বাধীন অংশে ভাগ করে, সেই অংশগুলো বিভিন্ন thread-এ বণ্টন
করে, তারপর একটি `Mutex<T>` ব্যবহার করে প্রতিটি thread-কে দিয়ে চূড়ান্ত ফলে নিজের অংশটি হালনাগাদ করাতে পারেন।

খেয়াল করুন, সাধারণ সংখ্যাগত operation করলে [standard library-র `std::sync::atomic`
module][atomic]<!-- ignore -->-এ `Mutex<T>` type-এর চেয়ে সরল type আছে। এই type-গুলো primitive type-এ নিরাপদ, concurrent, atomic প্রবেশাধিকার দেয়। এই উদাহরণে আমরা একটি
primitive type-সহ `Mutex<T>` ব্যবহার করেছি, যাতে `Mutex<T>` কীভাবে কাজ করে তার দিকে মনোযোগ দিতে পারি।

<!-- Old headings. Do not remove or links may break. -->

<a id="similarities-between-refcelltrct-and-mutextarct"></a>

### `RefCell<T>`/`Rc<T>` আর `Mutex<T>`/`Arc<T>`-এর তুলনা {#comparing-refcelltrct-and-mutextarct}

হয়তো খেয়াল করেছেন যে `counter` immutable, কিন্তু আমরা এর ভেতরের মানের একটি mutable reference পেতে পেরেছি; এর মানে `Cell` পরিবারের মতোই `Mutex<T>` interior mutability দেয়।
অধ্যায় 15-এ যেভাবে একটি `Rc<T>`-এর ভেতরের বিষয়বস্তু পরিবর্তন করতে `RefCell<T>` ব্যবহার করেছিলাম, সেভাবেই একটি `Arc<T>`-এর ভেতরের বিষয়বস্তু পরিবর্তন করতে `Mutex<T>` ব্যবহার
করি।

আরেকটি খেয়াল করার মতো বিষয় হলো, `Mutex<T>` ব্যবহারের সময় Rust আপনাকে সব ধরনের logic error থেকে রক্ষা করতে পারে না। অধ্যায় 15 থেকে মনে করুন, `Rc<T>` ব্যবহারে reference
cycle তৈরির ঝুঁকি ছিল, যেখানে দুটি `Rc<T>` মান একে অপরকে নির্দেশ করে memory leak ঘটায়। একইভাবে `Mutex<T>`-এ _deadlocks_ (deadlock) তৈরির ঝুঁকি আছে। এগুলো ঘটে যখন একটি
operation-এর দুটি resource lock করা দরকার এবং দুটি thread প্রত্যেকে একটি করে lock নিয়ে ফেলেছে, ফলে তারা চিরকাল একে অপরের জন্য অপেক্ষা করে। deadlock নিয়ে আগ্রহী হলে
deadlock আছে এমন একটি Rust প্রোগ্রাম তৈরি করে দেখুন; তারপর যেকোনো ভাষায় mutex-এর deadlock এড়ানোর কৌশল খুঁজে পড়ে Rust-এ সেগুলো implement করার চেষ্টা করুন। `Mutex<T>` আর
`MutexGuard`-এর standard library API documentation-এ কাজের তথ্য আছে।

`Send` আর `Sync` trait নিয়ে এবং নিজস্ব type-এর সঙ্গে এগুলো কীভাবে ব্যবহার করা যায় তা নিয়ে কথা বলে আমরা এই অধ্যায় শেষ করব।

{{#quiz ../quizzes/ch16-03-shared-state.toml}}

[atomic]: https://doc.rust-lang.org/std/sync/atomic/index.html
