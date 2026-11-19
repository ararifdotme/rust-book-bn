## test কীভাবে চলবে তা নিয়ন্ত্রণ করা {#controlling-how-tests-are-run}

`cargo run` যেমন আপনার কোড compile করে তারপর তৈরি হওয়া binary চালায়, তেমনি `cargo test` আপনার কোড test mode-এ compile
করে এবং তৈরি হওয়া test binary চালায়। `cargo test`-এর তৈরি binary-র default আচরণ হলো সব test parallel-ভাবে (একসঙ্গে)
চালানো এবং test চলার সময় তৈরি হওয়া output ধরে রাখা (capture করা), যাতে সেই output দেখানো না হয় এবং test-এর ফল সংক্রান্ত
output পড়া সহজ হয়। তবে command line-এর বিকল্প দিয়ে এই default আচরণ বদলানো যায়।

কিছু command line বিকল্প যায় `cargo test`-এ, আর কিছু যায় তৈরি হওয়া test binary-তে। এই দুই ধরনের argument আলাদা করতে প্রথমে
`cargo test`-এর argument-গুলো লিখুন, তারপর বিভাজক `--`, তারপর test binary-র argument-গুলো। `cargo test --help` চালালে
`cargo test`-এর সঙ্গে ব্যবহারযোগ্য বিকল্পগুলো দেখা যায়, আর `cargo test -- --help` চালালে বিভাজকের পরে ব্যবহারযোগ্য বিকল্পগুলো
দেখা যায়। এই বিকল্পগুলোর বিবরণ [_The `rustc` Book_-এর “Tests”
অংশেও][tests] আছে।

[tests]: https://doc.rust-lang.org/rustc/tests/index.html

### test parallel-ভাবে বা পরপর চালানো {#running-tests-in-parallel-or-consecutively}

একাধিক test চালালে default-ভাবে সেগুলো thread ব্যবহার করে parallel-ভাবে চলে, অর্থাৎ দ্রুত শেষ হয় এবং আপনি তাড়াতাড়ি ফল
জানতে পারেন। test-গুলো যেহেতু একই সময়ে চলছে, তাই নিশ্চিত করতে হবে যে আপনার test-গুলো একে অপরের ওপর বা কোনো ভাগ করা
অবস্থার ওপর নির্ভর করে না, এমনকি বর্তমান working directory বা environment variable-এর মতো ভাগ করা পরিবেশের ওপরও না।

যেমন ধরুন, আপনার প্রতিটি test এমন কিছু কোড চালায় যা disk-এ _test-output.txt_ নামের একটি ফাইল তৈরি করে এবং তাতে কিছু ডেটা
লেখে। তারপর প্রতিটি test সেই ফাইলের ডেটা পড়ে এবং assert করে যে ফাইলে একটি নির্দিষ্ট মান আছে, যা প্রতিটি test-এ আলাদা।
test-গুলো একই সময়ে চলে বলে একটি test যখন ফাইলে লিখছে আর পড়ছে, তার মাঝখানে আরেকটি test ফাইলটি overwrite করে দিতে পারে।
তখন দ্বিতীয় test-টি fail করবে, কোড ভুল বলে নয়, বরং parallel-ভাবে চলার সময় test-গুলো একে অপরের কাজে হস্তক্ষেপ করেছে বলে।
একটি সমাধান হলো নিশ্চিত করা যে প্রতিটি test আলাদা ফাইলে লেখে; আরেকটি সমাধান হলো test-গুলো একটি একটি করে চালানো।

আপনি যদি test parallel-ভাবে চালাতে না চান, অথবা ব্যবহৃত thread-এর সংখ্যার ওপর আরও সূক্ষ্ম নিয়ন্ত্রণ চান, তাহলে test binary-তে
`--test-threads` flag এবং যতগুলো thread ব্যবহার করতে চান সেই সংখ্যা পাঠাতে পারেন। নিচের উদাহরণটি দেখুন:

```console
$ cargo test -- --test-threads=1
```

আমরা test thread-এর সংখ্যা `1` করে দিয়েছি, অর্থাৎ প্রোগ্রামকে বলছি কোনো parallelism ব্যবহার না করতে। একটি thread দিয়ে test
চালাতে parallel-ভাবে চালানোর চেয়ে বেশি সময় লাগবে, কিন্তু test-গুলো অবস্থা ভাগ করলেও একে অপরের কাজে হস্তক্ষেপ করবে না।

### function-এর output দেখানো {#showing-function-output}

default-ভাবে কোনো test pass করলে Rust-এর test library standard output-এ print হওয়া সবকিছু ধরে রাখে। যেমন, একটি test-এ
`println!` call করলে এবং test-টি pass করলে আমরা terminal-এ `println!`-এর output দেখব না; শুধু test pass করেছে এমন লাইনটি দেখব।
কোনো test fail করলে failure message-এর বাকি অংশের সঙ্গে standard output-এ যা print হয়েছে তা-ও দেখব।

উদাহরণ হিসেবে, Listing 11-10-এ একটি সাদামাটা function আছে যা তার parameter-এর মান print করে এবং 10 return করে, সঙ্গে আছে একটি
test যা pass করে আর একটি test যা fail করে।

<Listing number="11-10" file-name="src/lib.rs" caption="`println!` call করে এমন একটি function-এর test">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

</Listing>

`cargo test` দিয়ে এই test-গুলো চালালে আমরা এই output দেখব:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

খেয়াল করুন, এই output-এর কোথাও `I got the value 4` দেখা যায় না, যা pass করা test চলার সময় print হয়। ওই output ধরে রাখা
হয়েছে। fail করা test-এর output, `I got the value 8`, দেখা যায় test-এর সারসংক্ষেপের অংশে, যেখানে test fail করার কারণও দেখানো
হয়।

pass করা test-এর print করা মানও দেখতে চাইলে `--show-output` দিয়ে Rust-কে সফল test-এর output-ও দেখাতে বলতে পারি:

```console
$ cargo test -- --show-output
```

`--show-output` flag দিয়ে Listing 11-10-এর test-গুলো আবার চালালে এই output দেখি:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-01-show-output/output.txt}}
```

### নাম দিয়ে test-এর একটি অংশ চালানো {#running-a-subset-of-tests-by-name}

পুরো test suite (সব test-এর সংগ্রহ) চালাতে কখনো কখনো অনেক সময় লাগে। কোনো নির্দিষ্ট অংশের কোড নিয়ে কাজ করলে হয়তো শুধু সেই
কোড সংক্রান্ত test-গুলো চালাতে চাইবেন। যে test বা test-গুলো চালাতে চান তাদের নাম `cargo test`-এ argument হিসেবে পাঠিয়ে কোন
test চলবে তা বেছে নিতে পারেন।

test-এর একটি অংশ কীভাবে চালাতে হয় তা দেখাতে প্রথমে আমরা `add_two` function-এর জন্য তিনটি test তৈরি করব, যেমনটি Listing
11-11-এ দেখানো হয়েছে, তারপর বেছে নেব কোনগুলো চালাব।

<Listing number="11-11" file-name="src/lib.rs" caption="তিনটি ভিন্ন নামের তিনটি test">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

</Listing>

কোনো argument না দিয়ে test চালালে, আগে যেমন দেখেছি, সব test parallel-ভাবে চলবে:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### একটিমাত্র test চালানো {#running-single-tests}

যেকোনো test function-এর নাম `cargo test`-এ পাঠিয়ে শুধু সেই test-টি চালানো যায়:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

শুধু `one_hundred` নামের test-টি চলেছে; বাকি দুটি test ওই নামের সঙ্গে মেলেনি। test output শেষে `2 filtered out` দেখিয়ে জানিয়ে
দেয় যে আমাদের আরও test ছিল যেগুলো চলেনি।

এভাবে একাধিক test-এর নাম দেওয়া যায় না; `cargo test`-এ দেওয়া শুধু প্রথম মানটিই ব্যবহৃত হবে। তবে একাধিক test চালানোর একটি
উপায় আছে।

#### একাধিক test চালাতে filter করা {#filtering-to-run-multiple-tests}

আমরা test-এর নামের একটি অংশ দিতে পারি, আর যে test-এর নাম সেই মানের সঙ্গে মেলে সেটিই চলবে। যেমন, আমাদের দুটি test-এর নামে
`add` আছে, তাই `cargo test add` চালিয়ে ওই দুটি চালাতে পারি:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

এই command নামে `add` আছে এমন সব test চালিয়েছে এবং `one_hundred` নামের test-টি বাদ দিয়েছে। আরও খেয়াল করুন, যে module-এ একটি
test থাকে সেটি test-এর নামের অংশ হয়ে যায়, তাই module-এর নাম দিয়ে filter করে একটি module-এর সব test চালানো যায়।

<!-- Old headings. Do not remove or links may break. -->

<a id="ignoring-some-tests-unless-specifically-requested"></a>

### বিশেষভাবে না চাইলে test উপেক্ষা করা {#ignoring-tests-unless-specifically-requested}

কখনো কখনো নির্দিষ্ট কয়েকটি test চালাতে অনেক সময় লাগে, তাই `cargo test`-এর বেশিরভাগ চালানোর সময় হয়তো এগুলো বাদ দিতে চাইবেন।
যে test-গুলো চালাতে চান সেগুলোর সবকটি argument হিসেবে তালিকাভুক্ত না করে, বরং সময়সাপেক্ষ test-গুলোকে `ignore` attribute দিয়ে
annotate করে বাদ দিতে পারেন, যেমনটি এখানে দেখানো হয়েছে:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

যে test বাদ দিতে চাই, তাতে `#[test]`-এর পরে `#[ignore]` লাইনটি যোগ করি। এখন test চালালে `it_works` চলে, কিন্তু `expensive_test`
চলে না:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

`expensive_test` function-টি `ignored` হিসেবে তালিকায় দেখানো হয়। শুধু উপেক্ষিত test-গুলো চালাতে চাইলে `cargo test -- --ignored`
ব্যবহার করতে পারি:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

কোন test চলবে তা নিয়ন্ত্রণ করে আপনি নিশ্চিত করতে পারেন যে `cargo test`-এর ফল দ্রুত পাওয়া যাবে। যখন `ignored` test-গুলোর ফল
যাচাই করা যুক্তিসঙ্গত মনে হবে এবং ফলের জন্য অপেক্ষা করার সময় থাকবে, তখন এর বদলে `cargo test -- --ignored` চালাতে পারেন।
উপেক্ষিত হোক বা না হোক, সব test চালাতে চাইলে `cargo test -- --include-ignored` চালাতে পারেন।

{{#quiz ../quizzes/ch11-02-running-tests.toml}}
