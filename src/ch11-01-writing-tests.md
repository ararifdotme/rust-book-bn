## test কীভাবে লিখতে হয় {#how-to-write-tests}

_Tests_ (test) হলো এমন Rust function, যা যাচাই করে যে test নয় এমন কোড প্রত্যাশিতভাবে কাজ করছে কি না। test function-এর
body সাধারণত এই তিনটি কাজ করে:

- প্রয়োজনীয় ডেটা বা অবস্থা প্রস্তুত করা।
- যে কোড test করতে চান তা চালানো।
- ফল যে প্রত্যাশামতো হয়েছে তা assert করা।

এই কাজগুলো করে এমন test লেখার জন্য Rust বিশেষভাবে যে বৈশিষ্ট্যগুলো দেয় সেগুলো দেখা যাক, যার মধ্যে আছে `test`
attribute, কয়েকটি macro, আর `should_panic` attribute।

<!-- Old headings. Do not remove or links may break. -->

<a id="the-anatomy-of-a-test-function"></a>

### test function-এর গঠন {#structuring-test-functions}

সবচেয়ে সরল রূপে, Rust-এ একটি test হলো `test` attribute (কোডের সঙ্গে জুড়ে দেওয়া বাড়তি নির্দেশ) দিয়ে annotate করা একটি
function। attribute হলো Rust কোডের বিভিন্ন অংশ সম্পর্কে মেটাডেটা; একটি উদাহরণ হলো অধ্যায় 5-এ struct-এর সঙ্গে ব্যবহার করা
`derive` attribute। একটি function-কে test function-এ পরিণত করতে `fn`-এর আগের লাইনে `#[test]` যোগ করুন। `cargo test`
command দিয়ে test চালালে Rust একটি test runner (test চালানোর প্রোগ্রাম) binary তৈরি করে, যা annotate করা function-গুলো চালায়
এবং প্রতিটি test function pass করল নাকি fail করল তা জানায়।

Cargo দিয়ে যখনই নতুন library project বানাই, তখনই আমাদের জন্য স্বয়ংক্রিয়ভাবে একটি test module তৈরি হয়, যাতে একটি test
function থাকে। এই module আপনাকে test লেখার একটি template (কাঠামো) দেয়, যাতে প্রতিবার নতুন project শুরু করার সময় সঠিক
গঠন ও syntax খুঁজে দেখতে না হয়। যত খুশি বাড়তি test function আর যত খুশি test module যোগ করতে পারেন!

কোনো কোড আসলে test করার আগে আমরা template test নিয়ে পরীক্ষা-নিরীক্ষা করে test কীভাবে কাজ করে তার কিছু দিক দেখব। তারপর
বাস্তব জীবনের কিছু test লিখব, যা আমাদের লেখা কোড call করে এবং তার আচরণ সঠিক কি না তা assert করে।

চলুন `adder` নামে একটি নতুন library project বানাই, যা দুটি সংখ্যা যোগ করবে:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

আপনার `adder` library-র _src/lib.rs_ ফাইলের বিষয়বস্তু দেখতে Listing 11-1-এর মতো হওয়ার কথা।

<Listing number="11-1" file-name="src/lib.rs" caption="`cargo new`-এর স্বয়ংক্রিয়ভাবে তৈরি করা কোড">

<!-- manual-regeneration
cd listings/ch11-writing-automated-tests
rm -rf listing-11-01
cargo new listing-11-01 --lib --name adder
cd listing-11-01
echo "$ cargo test" > output.txt
RUSTFLAGS="-A unused_variables -A dead_code" RUST_TEST_THREADS=1 cargo test >> output.txt 2>&1
git diff output.txt # commit any relevant changes; discard irrelevant ones
cd ../../..
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

</Listing>

ফাইলটি শুরু হয় একটি উদাহরণ `add` function দিয়ে, যাতে test করার মতো কিছু একটা থাকে।

আপাতত শুধু `it_works` function-এর দিকে মনোযোগ দিই। `#[test]` annotation খেয়াল করুন: এই attribute বোঝায় যে এটি একটি test
function, তাই test runner জানে এই function-কে test হিসেবে গণ্য করতে হবে। `tests` module-এ test নয় এমন function-ও থাকতে
পারে, যা সাধারণ পরিস্থিতি প্রস্তুত করতে বা সাধারণ কোনো কাজ করতে সাহায্য করে, তাই কোন function-গুলো test তা সবসময় বলে দিতে
হয়।

উদাহরণ function-এর body `assert_eq!` macro ব্যবহার করে assert করে যে `result`, যাতে 2 ও 2 দিয়ে `add` call করার ফল আছে, 4-এর
সমান। এই assertion একটি সাধারণ test-এর ধরনের উদাহরণ। চলুন এটি চালিয়ে দেখি test-টি pass করে কি না।

`cargo test` command আমাদের project-এর সব test চালায়, যেমনটি Listing 11-2-এ দেখানো হয়েছে।

<Listing number="11-2" caption="স্বয়ংক্রিয়ভাবে তৈরি test চালানোর output">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

</Listing>

Cargo test-টি compile করে চালিয়েছে। আমরা `running 1 test` লাইনটি দেখি। পরের লাইনে তৈরি হওয়া test function-এর নাম
`tests::it_works` দেখানো হয়েছে, আর দেখানো হয়েছে যে test-টি চালানোর ফল `ok`। সামগ্রিক সারসংক্ষেপ `test
result: ok.` মানে সব test pass করেছে, আর `1
passed; 0 failed` অংশটি কতগুলো test pass বা fail করেছে তার মোট হিসাব দেয়।

একটি test-কে ignored (উপেক্ষিত) হিসেবে চিহ্নিত করা যায়, যাতে নির্দিষ্ট কোনো ক্ষেত্রে সেটি না চলে; এটি আমরা এই অধ্যায়ের
পরে [“বিশেষভাবে না চাইলে test উপেক্ষা করা”][ignoring]<!-- ignore --> অংশে দেখব। এখানে আমরা তা করিনি বলে সারসংক্ষেপে
দেখাচ্ছে `0 ignored`। `cargo test` command-এ একটি argument দিয়ে শুধু সেই test-গুলো চালানো যায় যাদের নাম একটি string-এর সঙ্গে
মেলে; একে বলে _filtering_ (ছেঁকে নেওয়া), যা আমরা [“নাম দিয়ে test-এর একটি অংশ চালানো”][subset]<!-- ignore --> অংশে দেখব। এখানে
চালানো test-গুলো আমরা ছেঁকে নিইনি, তাই সারসংক্ষেপের শেষে দেখাচ্ছে `0 filtered out`।

`0 measured` পরিসংখ্যানটি performance মাপে এমন benchmark test-এর জন্য। এই লেখার সময় পর্যন্ত benchmark test শুধু nightly
Rust-এ পাওয়া যায়। আরও জানতে [benchmark test নিয়ে documentation][bench] দেখুন।

test output-এর পরের অংশ, যা `Doc-tests adder` থেকে শুরু, তা documentation test-এর ফলের জন্য। আমাদের এখনো কোনো documentation
test নেই, তবে আমাদের API documentation-এ থাকা যেকোনো কোডের উদাহরণ Rust compile করতে পারে। এই বৈশিষ্ট্য আপনার docs আর
কোডকে মিলিয়ে রাখতে সাহায্য করে! documentation test কীভাবে লিখতে হয় তা আমরা অধ্যায় 14-এর [“Documentation Comments as
Tests”][doc-comments]<!-- ignore --> অংশে আলোচনা করব। আপাতত `Doc-tests` output উপেক্ষা করব।

চলুন test-টিকে নিজেদের প্রয়োজন অনুযায়ী সাজাতে শুরু করি। প্রথমে `it_works` function-এর নাম বদলে অন্য কোনো নাম দিন, যেমন
`exploration`, এভাবে:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/src/lib.rs}}
```

তারপর আবার `cargo test` চালান। output এখন `it_works`-এর বদলে `exploration` দেখায়:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-01-changing-test-name/output.txt}}
```

এবার আমরা আরেকটি test যোগ করব, তবে এবার এমন একটি test বানাব যা fail করবে! test function-এর ভেতরে কোনো কিছু panic করলে
test fail করে। প্রতিটি test একটি নতুন thread-এ চলে, আর main thread যখন দেখে যে কোনো test thread মারা গেছে, তখন test-টিকে
failed হিসেবে চিহ্নিত করা হয়। অধ্যায় 9-এ আমরা বলেছিলাম যে panic করার সবচেয়ে সহজ উপায় হলো `panic!` macro call করা।
নতুন test-টি `another` নামের একটি function হিসেবে লিখুন, যাতে আপনার _src/lib.rs_ ফাইল দেখতে Listing 11-3-এর মতো হয়।

<Listing number="11-3" file-name="src/lib.rs" caption="দ্বিতীয় একটি test যোগ করা, যা fail করবে কারণ আমরা `panic!` macro call করি">

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

</Listing>

`cargo test` দিয়ে আবার test চালান। output দেখতে Listing 11-4-এর মতো হওয়ার কথা, যা দেখায় যে আমাদের `exploration` test pass
করেছে আর `another` fail করেছে।

<Listing number="11-4" caption="একটি test pass ও একটি test fail করলে test-এর ফল">

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

</Listing>

<!-- manual-regeneration
rg panicked listings/ch11-writing-automated-tests/listing-11-03/output.txt
check the line number of the panic matches the line number in the following paragraph
 -->

`ok`-এর বদলে `test tests::another` লাইনটি দেখায় `FAILED`। আলাদা আলাদা ফল আর সারসংক্ষেপের মাঝে দুটি নতুন অংশ দেখা যায়:
প্রথমটি প্রতিটি test fail করার বিস্তারিত কারণ দেখায়। এই ক্ষেত্রে আমরা বিস্তারিত পাই যে `tests::another` fail করেছে, কারণ
এটি _src/lib.rs_ ফাইলের 17 নম্বর লাইনে `Make
this test fail` message দিয়ে panic করেছে। পরের অংশে শুধু fail করা সব test-এর নামের তালিকা থাকে, যা অনেক test আর fail করা
test-এর অনেক বিস্তারিত output থাকলে কাজে লাগে। fail করা কোনো test-এর নাম ব্যবহার করে শুধু সেই test-টি চালিয়ে সহজে debug
করা যায়; test চালানোর উপায় নিয়ে আমরা [“test কীভাবে চলবে তা নিয়ন্ত্রণ
করা”][controlling-how-tests-are-run]<!-- ignore --> অংশে আরও কথা বলব।

শেষে সারসংক্ষেপের লাইনটি দেখা যায়: সামগ্রিকভাবে আমাদের test-এর ফল `FAILED`। আমাদের একটি test pass করেছে আর একটি fail
করেছে।

বিভিন্ন পরিস্থিতিতে test-এর ফল দেখতে কেমন তা দেখলেন, এবার `panic!` ছাড়া test-এ কাজে লাগে এমন আরও কিছু macro দেখা যাক।

<!-- Old headings. Do not remove or links may break. -->

<a id="checking-results-with-the-assert-macro"></a>

### `assert!` দিয়ে ফল যাচাই করা {#checking-results-with-assert}

standard library-র দেওয়া `assert!` macro কাজে লাগে যখন আপনি নিশ্চিত হতে চান যে test-এর কোনো শর্তের মান `true`। আমরা
`assert!` macro-কে এমন একটি argument দিই যার মান একটি Boolean। মান `true` হলে কিছুই ঘটে না এবং test pass করে। মান `false`
হলে `assert!` macro `panic!` call করে test-টিকে fail করায়। `assert!` macro ব্যবহার করে যাচাই করা যায় যে আমাদের কোড আমাদের
উদ্দেশ্য অনুযায়ী কাজ করছে।

অধ্যায় 5-এর Listing 5-15-এ আমরা একটি `Rectangle` struct আর একটি `can_hold` method ব্যবহার করেছিলাম, যা এখানে Listing 11-5-এ
আবার দেওয়া হলো। চলুন এই কোড _src/lib.rs_ ফাইলে রাখি, তারপর `assert!` macro ব্যবহার করে এর জন্য কিছু test লিখি।

<Listing number="11-5" file-name="src/lib.rs" caption="অধ্যায় 5-এর `Rectangle` struct ও এর `can_hold` method">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-05/src/lib.rs}}
```

</Listing>

`can_hold` method একটি Boolean return করে, অর্থাৎ এটি `assert!` macro-র জন্য একেবারে উপযুক্ত। Listing 11-6-এ আমরা এমন একটি
test লিখি যা `can_hold` method পরীক্ষা করে: 8 প্রস্থ ও 7 উচ্চতার একটি `Rectangle` instance তৈরি করে assert করে যে এটি 5
প্রস্থ ও 1 উচ্চতার আরেকটি `Rectangle` instance ধারণ করতে পারে।

<Listing number="11-6" file-name="src/lib.rs" caption="`can_hold`-এর একটি test, যা যাচাই করে বড় একটি আয়তক্ষেত্র সত্যিই ছোট একটি আয়তক্ষেত্র ধারণ করতে পারে কি না">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

</Listing>

`tests` module-এর ভেতরে `use super::*;` লাইনটি খেয়াল করুন। `tests` module একটি সাধারণ module, যা অধ্যায় 7-এর [“module
tree-র কোনো item উল্লেখ করার path”][paths-for-referring-to-an-item-in-the-module-tree]<!-- ignore -->
অংশে আলোচিত দৃশ্যমানতার সাধারণ নিয়ম মেনে চলে। `tests` module যেহেতু একটি ভেতরের module, তাই বাইরের module-এর যে কোড test
করা হচ্ছে তাকে ভেতরের module-এর scope-এ আনতে হবে। এখানে আমরা একটি glob ব্যবহার করি, ফলে বাইরের module-এ সংজ্ঞায়িত সবকিছু
এই `tests` module-এ পাওয়া যায়।

আমরা test-টির নাম দিয়েছি `larger_can_hold_smaller`, আর প্রয়োজনীয় দুটি `Rectangle` instance তৈরি করেছি। তারপর `assert!`
macro call করে তাকে `larger.can_hold(&smaller)` call করার ফল পাঠিয়েছি। এই expression-এর `true` return করার কথা, তাই আমাদের
test pass করার কথা। চলুন দেখি!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

সত্যিই pass করেছে! এবার আরেকটি test যোগ করি, এবার assert করব যে ছোট একটি আয়তক্ষেত্র বড় একটি আয়তক্ষেত্র ধারণ করতে পারে
না:

<span class="filename">Filename: src/lib.rs</span>

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/src/lib.rs:here}}
```

এই ক্ষেত্রে `can_hold` function-এর সঠিক ফল যেহেতু `false`, তাই `assert!` macro-তে পাঠানোর আগে ফলটিকে উল্টে (negate) দিতে
হবে। ফলে `can_hold` `false` return করলে আমাদের test pass করবে:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-02-adding-another-rectangle-test/output.txt}}
```

দুটি test pass করল! এবার দেখা যাক আমাদের কোডে একটি বাগ ঢোকালে test-এর ফলে কী ঘটে। `can_hold` method-এর implementation
বদলে প্রস্থ তুলনা করার সময় বড়-চিহ্ন (`>`)-এর জায়গায় ছোট-চিহ্ন (`<`) বসাব:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/src/lib.rs:here}}
```

এখন test চালালে এই ফল পাওয়া যায়:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-03-introducing-a-bug/output.txt}}
```

আমাদের test বাগটি ধরে ফেলেছে! `larger.width` হলো `8` আর `smaller.width` হলো `5`, তাই `can_hold`-এ প্রস্থের তুলনা এখন `false`
return করে: 8, 5-এর চেয়ে ছোট নয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="testing-equality-with-the-assert_eq-and-assert_ne-macros"></a>

### `assert_eq!` ও `assert_ne!` দিয়ে সমতা যাচাই করা {#testing-equality-with-assert_eq-and-assert_ne}

কার্যক্ষমতা যাচাইয়ের একটি প্রচলিত উপায় হলো test করা কোডের ফল আর কোডটি যে মান return করবে বলে আশা করছেন, এ দুটির সমতা
যাচাই করা। এটা `assert!` macro-তে `==` operator ব্যবহার করা একটি expression পাঠিয়ে করা যায়। কিন্তু এটি এত প্রচলিত test যে
standard library এই test আরও সুবিধাজনকভাবে করার জন্য এক জোড়া macro দেয়—`assert_eq!` ও `assert_ne!`। এই macro-গুলো দুটি
argument যথাক্রমে সমান নাকি অসমান তা তুলনা করে। assertion fail করলে এরা দুটি মানও print করে, ফলে test _কেন_ fail করল তা
বোঝা সহজ হয়; অন্যদিকে `assert!` macro শুধু জানায় যে `==` expression-এর জন্য এটি `false` মান পেয়েছে, কোন মানগুলোর কারণে
`false` হলো তা print করে না।

Listing 11-7-এ আমরা `add_two` নামের একটি function লিখি, যা তার parameter-এর সঙ্গে `2` যোগ করে, তারপর `assert_eq!` macro দিয়ে
এই function test করি।

<Listing number="11-7" file-name="src/lib.rs" caption="`assert_eq!` macro দিয়ে `add_two` function test করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

</Listing>

চলুন দেখি এটি pass করে কি না!

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-07/output.txt}}
```

আমরা `result` নামের একটি variable তৈরি করি, যাতে `add_two(2)` call করার ফল থাকে। তারপর `assert_eq!` macro-তে argument হিসেবে
`result` আর `4` পাঠাই। এই test-এর output লাইন হলো `test tests::it_adds_two
... ok`, আর `ok` লেখাটি বোঝায় আমাদের test pass করেছে!

`assert_eq!` fail করলে দেখতে কেমন হয় তা দেখতে চলুন কোডে একটি বাগ ঢোকাই। `add_two` function-এর implementation বদলে এর বদলে `3`
যোগ করান:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

আবার test চালান:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

আমাদের test বাগটি ধরে ফেলেছে! `tests::it_adds_two` test fail করেছে, আর message আমাদের জানাচ্ছে যে fail করা assertion-টি ছিল
`left == right`, এবং `left` ও `right`-এর মান কী। এই message debug শুরু করতে সাহায্য করে: `left` argument, যেখানে আমাদের
`add_two(2)` call করার ফল ছিল, তা ছিল `5`, কিন্তু `right` argument ছিল `4`। ভেবে দেখুন, অনেক test একসঙ্গে চললে এটি বিশেষভাবে
কাজে লাগবে।

খেয়াল করুন, কিছু ভাষা ও test framework-এ সমতা যাচাইয়ের assertion function-এর parameter-গুলোকে বলা হয় `expected` আর `actual`,
আর argument কোন ক্রমে দেওয়া হচ্ছে তা গুরুত্বপূর্ণ। কিন্তু Rust-এ এদের বলা হয় `left` আর `right`, এবং প্রত্যাশিত মান আর কোডের
তৈরি মান কোন ক্রমে দিচ্ছি তাতে কিছু যায় আসে না। এই test-এর assertion-টি আমরা `assert_eq!(4, result)` হিসেবেও লিখতে পারতাম,
তাতেও `` assertion `left == right` failed `` দেখানো একই failure message আসত।

`assert_ne!` macro pass করবে যদি আমাদের দেওয়া দুটি মান সমান না হয়, আর fail করবে যদি সমান হয়। এই macro সবচেয়ে কাজে লাগে
যখন আমরা নিশ্চিত নই একটি মান কী _হবে_, কিন্তু জানি মানটি নিশ্চিতভাবে কী _হওয়া উচিত নয়_। যেমন, যদি এমন একটি function
test করি যা নিশ্চিতভাবে তার input-কে কোনো না কোনোভাবে বদলায়, কিন্তু কীভাবে বদলায় তা নির্ভর করে সপ্তাহের কোন দিনে test
চালাচ্ছি তার ওপর, তাহলে সবচেয়ে ভালো assertion হতে পারে এটা যে function-এর output তার input-এর সমান নয়।

ভেতরে ভেতরে `assert_eq!` ও `assert_ne!` macro যথাক্রমে `==` ও `!=` operator ব্যবহার করে। assertion fail করলে এই macro-গুলো
debug formatting ব্যবহার করে তাদের argument print করে, অর্থাৎ তুলনা করা মানগুলোকে `PartialEq` ও `Debug` trait implement করতে
হবে। সব primitive type আর standard library-র বেশিরভাগ type এই trait-গুলো implement করে। নিজের সংজ্ঞায়িত struct ও enum-এর
সমতা assert করতে আপনাকে `PartialEq` implement করতে হবে। assertion fail করলে মানগুলো print করতে `Debug`-ও implement করতে
হবে। অধ্যায় 5-এর Listing 5-12-এ যেমন বলা হয়েছে, দুটি trait-ই derivable trait (স্বয়ংক্রিয়ভাবে যুক্ত করা যায় এমন trait),
তাই সাধারণত আপনার struct বা enum-এর সংজ্ঞায় `#[derive(PartialEq, Debug)]` annotation যোগ করলেই চলে। এগুলো ও অন্যান্য
derivable trait সম্পর্কে আরও বিস্তারিত জানতে Appendix C, [“Derivable Traits,”][derivable-traits]<!-- ignore --> দেখুন।

### নিজস্ব failure message যোগ করা {#adding-custom-failure-messages}

`assert!`, `assert_eq!` ও `assert_ne!` macro-তে ঐচ্ছিক argument হিসেবে একটি নিজস্ব message-ও যোগ করা যায়, যা failure
message-এর সঙ্গে print হবে। বাধ্যতামূলক argument-গুলোর পরে দেওয়া যেকোনো argument `format!` macro-তে পাঠানো হয় (অধ্যায় 8-এর
[“`+` বা `format!` দিয়ে জোড়া
লাগানো”][concatenating]<!--
ignore --> অংশে আলোচিত), তাই `{}` placeholder-যুক্ত একটি format string আর সেই placeholder-গুলোতে বসানোর মান পাঠাতে পারেন।
একটি assertion-এর মানে কী তা লিখে রাখতে নিজস্ব message কাজে লাগে; test fail করলে কোডের সমস্যাটা কী তা আরও ভালোভাবে
বুঝতে পারবেন।

যেমন ধরুন, আমাদের একটি function আছে যা মানুষকে নাম ধরে শুভেচ্ছা জানায়, আর আমরা test করতে চাই যে function-এ পাঠানো নামটি
output-এ আছে:

<span class="filename">Filename: src/lib.rs</span>
```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-05-greeter/src/lib.rs}}
```

এই প্রোগ্রামের চাহিদা এখনো চূড়ান্ত হয়নি, আর আমরা প্রায় নিশ্চিত যে শুভেচ্ছার শুরুতে থাকা `Hello` লেখাটি বদলাবে। আমরা ঠিক
করেছি চাহিদা বদলালে test হালনাগাদ করতে চাই না, তাই `greeting` function-এর return করা মানের সঙ্গে হুবহু সমতা যাচাই না করে
শুধু assert করব যে output-এ input parameter-এর লেখাটি আছে।

এবার `greeting`-কে বদলে `name` বাদ দিয়ে এই কোডে একটি বাগ ঢোকাই, যাতে দেখতে পারি default test failure দেখতে কেমন:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/src/lib.rs:here}}
```

এই test চালালে এই ফল পাওয়া যায়:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-06-greeter-with-bug/output.txt}}
```

এই ফল শুধু জানায় যে assertion fail করেছে এবং assertion-টি কোন লাইনে। আরও কাজের failure message হতো যদি `greeting` function
থেকে পাওয়া মানটি print করত। চলুন একটি নিজস্ব failure message যোগ করি, যা একটি format string দিয়ে তৈরি, যার placeholder-এ
বসবে `greeting` function থেকে আসলে পাওয়া মানটি:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

এখন test চালালে আরও তথ্যপূর্ণ error message পাব:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

test output-এ আসলে কোন মান পেয়েছি তা দেখতে পাচ্ছি, যা আমরা যা ঘটবে আশা করেছিলাম তার বদলে কী ঘটল তা debug করতে সাহায্য
করবে।

### `should_panic` দিয়ে panic যাচাই করা {#checking-for-panics-with-should_panic}

return মান যাচাইয়ের পাশাপাশি এটাও যাচাই করা গুরুত্বপূর্ণ যে আমাদের কোড error-এর পরিস্থিতিগুলো প্রত্যাশামতো সামলায়। যেমন,
অধ্যায় 9-এর Listing 9-13-এ তৈরি করা `Guess` type-টির কথা ভাবুন। `Guess` ব্যবহার করা অন্য কোড এই নিশ্চয়তার ওপর নির্ভর করে যে
`Guess` instance-এ শুধু 1 থেকে 100-এর মধ্যের মান থাকবে। আমরা এমন একটি test লিখতে পারি যা নিশ্চিত করে যে ওই সীমার বাইরের
মান দিয়ে `Guess` instance তৈরির চেষ্টা করলে panic হয়।

এটা করতে আমরা test function-এ `should_panic` attribute যোগ করি। function-এর ভেতরের কোড panic করলে test pass করে; function-এর
ভেতরের কোড panic না করলে test fail করে।

Listing 11-8-এ এমন একটি test দেখানো হয়েছে যা যাচাই করে যে `Guess::new`-এর error-এর পরিস্থিতিগুলো আমাদের প্রত্যাশিত সময়েই
ঘটে।

<Listing number="11-8" file-name="src/lib.rs" caption="একটি শর্ত যে `panic!` ঘটাবে তা test করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

</Listing>

আমরা `#[should_panic]` attribute বসাই `#[test]` attribute-এর পরে এবং যে test function-এর ওপর এটি প্রযোজ্য তার আগে। এই test pass
করলে ফল কেমন হয় দেখা যাক:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-08/output.txt}}
```

ভালোই দেখাচ্ছে! এবার মান 100-এর বেশি হলে `new` function panic করবে, এই শর্তটি সরিয়ে দিয়ে আমাদের কোডে একটি বাগ ঢোকাই:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/src/lib.rs:here}}
```

Listing 11-8-এর test চালালে এটি fail করবে:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-08-guess-with-bug/output.txt}}
```

এই ক্ষেত্রে আমরা খুব একটা কাজের message পাই না, তবে test function-টি দেখলে বুঝি যে এটি `#[should_panic]` দিয়ে annotate করা।
আমরা যে failure পেয়েছি তার মানে test function-এর কোড panic ঘটায়নি।

`should_panic` ব্যবহার করা test অস্পষ্ট হতে পারে। আমরা যে কারণে panic আশা করছিলাম তার বদলে অন্য কোনো কারণে panic করলেও
`should_panic` test pass করত। `should_panic` test-কে আরও নিখুঁত করতে `should_panic` attribute-এ একটি ঐচ্ছিক `expected`
parameter যোগ করা যায়। test harness (test চালানোর কাঠামো) নিশ্চিত করবে যে failure message-এ দেওয়া লেখাটি আছে। যেমন, Listing
11-9-এ `Guess`-এর পরিবর্তিত কোডটি দেখুন, যেখানে মান খুব ছোট নাকি খুব বড় তার ওপর নির্ভর করে `new` function ভিন্ন ভিন্ন message
দিয়ে panic করে।

<Listing number="11-9" file-name="src/lib.rs" caption="নির্দিষ্ট একটি substring আছে এমন panic message-সহ `panic!` test করা">

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

</Listing>

এই test pass করবে, কারণ `should_panic` attribute-এর `expected` parameter-এ আমরা যে মান দিয়েছি, তা `Guess::new` function যে
message দিয়ে panic করে তার একটি substring (string-এর অংশ)। আমরা প্রত্যাশিত পুরো panic message-টিও দিতে পারতাম, যা এই
ক্ষেত্রে হতো `Guess value must be less than or equal to
100, got 200`। কতটুকু উল্লেখ করবেন তা নির্ভর করে panic message-এর কতটুকু অনন্য বা পরিবর্তনশীল, আর আপনি test কতটা নিখুঁত
চান তার ওপর। এই ক্ষেত্রে panic message-এর একটি substring-ই যথেষ্ট এটা নিশ্চিত করতে যে test function-এর কোড
`else if value > 100` অংশটি চালায়।

`expected` message-যুক্ত একটি `should_panic` test fail করলে কী ঘটে তা দেখতে চলুন আবার কোডে একটি বাগ ঢোকাই, `if value < 1`
আর `else if value > 100` block-এর body অদলবদল করে:

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/src/lib.rs:here}}
```

এবার `should_panic` test চালালে এটি fail করবে:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-09-guess-with-panic-msg-bug/output.txt}}
```

failure message বোঝায় যে এই test আমাদের প্রত্যাশামতো সত্যিই panic করেছে, কিন্তু panic message-এ প্রত্যাশিত string `less than or equal
to 100` ছিল না। এই ক্ষেত্রে আমরা যে panic message পেয়েছি তা হলো `Guess value must
be greater than or equal to 1, got 200`। এখন আমরা খুঁজতে শুরু করতে পারি বাগটি কোথায়!

### test-এ `Result<T, E>` ব্যবহার করা {#using-resultt-e-in-tests}

এ পর্যন্ত আমাদের সব test fail করলে panic করে। আমরা `Result<T, E>` ব্যবহার করে এমন test-ও লিখতে পারি! Listing 11-1-এর test-টিকে
এখানে নতুন করে লেখা হয়েছে, যাতে এটি `Result<T,
E>` ব্যবহার করে এবং panic না করে একটি `Err` return করে:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

`it_works` function-এর return type এখন `Result<(), String>`। function-এর body-তে `assert_eq!` macro call না করে test pass
করলে আমরা `Ok(())` return করি, আর test fail করলে ভেতরে একটি `String`-সহ একটি `Err` return করি।

test-কে `Result<T, E>` return করার মতো করে লিখলে test-এর body-তে প্রশ্নবোধক চিহ্নের operator ব্যবহার করা যায়, যা এমন test
লেখার একটি সুবিধাজনক উপায় হতে পারে, যেগুলোর ভেতরের কোনো operation `Err` variant return করলে test fail করা উচিত।

`Result<T,
E>` ব্যবহার করা test-এ `#[should_panic]` annotation ব্যবহার করা যায় না। কোনো operation যে `Err` variant return করে তা assert করতে
`Result<T, E>` মানের ওপর প্রশ্নবোধক চিহ্নের operator ব্যবহার করবেন _না_। বরং `assert!(value.is_err())` ব্যবহার করুন।

test লেখার কয়েকটি উপায় জানলেন, এবার দেখা যাক test চালালে কী ঘটে, আর `cargo
test`-এর সঙ্গে কী কী বিকল্প ব্যবহার করা যায়।

{{#quiz ../quizzes/ch11-01-writing-tests.toml}}

[concatenating]: ch08-02-strings.html#concatenating-with--or-format
[bench]: https://doc.rust-lang.org/unstable-book/library-features/test.html
[ignoring]: ch11-02-running-tests.html#ignoring-tests-unless-specifically-requested
[subset]: ch11-02-running-tests.html#running-a-subset-of-tests-by-name
[controlling-how-tests-are-run]: ch11-02-running-tests.html#controlling-how-tests-are-run
[derivable-traits]: appendix-03-derivable-traits.html
[doc-comments]: ch14-02-publishing-to-crates-io.html#documentation-comments-as-tests
[paths-for-referring-to-an-item-in-the-module-tree]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
