# একটি সংখ্যা আন্দাজের খেলা প্রোগ্রাম করা {#programming-a-guessing-game}

চলুন, একসঙ্গে হাতে-কলমে একটি প্রজেক্ট করতে করতে Rust-এ ঝাঁপিয়ে পড়ি! এই
অধ্যায়ে একটি বাস্তব প্রোগ্রামে ব্যবহার করে দেখিয়ে Rust-এর কয়েকটি সাধারণ ধারণার
সঙ্গে আপনাকে পরিচয় করিয়ে দেওয়া হবে। আপনি শিখবেন `let`, `match`, method
(টাইপের সঙ্গে যুক্ত function), associated function (কোনো type-এর সঙ্গে যুক্ত
function), বাইরের crate (Rust-এর কোডের একক) এবং আরও অনেক কিছু! পরের অধ্যায়গুলোতে
এই ধারণাগুলো আমরা আরও বিস্তারিত দেখব। এই অধ্যায়ে আপনি শুধু মূল বিষয়গুলো
অনুশীলন করবেন।

আমরা নতুন প্রোগ্রামারদের একটি চিরচেনা সমস্যা সমাধান করব: একটি সংখ্যা আন্দাজের
খেলা। খেলাটি এভাবে চলে: প্রোগ্রামটি 1 থেকে 100-এর মধ্যে একটি র‍্যান্ডম integer
(পূর্ণসংখ্যা) তৈরি করবে। তারপর খেলোয়াড়কে একটি সংখ্যা আন্দাজ করে লিখতে বলবে।
আন্দাজ লেখার পর প্রোগ্রাম জানাবে আন্দাজটি খুব কম হয়েছে নাকি খুব বেশি। আন্দাজ
সঠিক হলে খেলাটি একটি অভিনন্দন-বার্তা প্রিন্ট করে বন্ধ হয়ে যাবে।

> **দ্রষ্টব্য:** এই অধ্যায়ে কোনো কুইজ নেই, কারণ এর উদ্দেশ্য শুধু ভাষাটির সঙ্গে আপনাকে একটু পরিচিত করে তোলা।

## নতুন প্রজেক্ট তৈরি {#setting-up-a-new-project}

নতুন প্রজেক্ট তৈরি করতে অধ্যায় 1-এ বানানো _projects_ ডিরেক্টরিতে যান, আর Cargo
দিয়ে একটি নতুন প্রজেক্ট বানান, এভাবে:

```console
$ cargo new guessing_game
$ cd guessing_game
```

প্রথম কমান্ড `cargo new` প্রথম argument (function-এ পাঠানো আসল মান) হিসেবে
প্রজেক্টের নাম (`guessing_game`) নেয়। দ্বিতীয় কমান্ডটি নতুন প্রজেক্টের
ডিরেক্টরিতে নিয়ে যায়।

তৈরি হওয়া _Cargo.toml_ ফাইলটি দেখুন:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial
rm -rf no-listing-01-cargo-new
cargo new no-listing-01-cargo-new --name guessing_game
cd no-listing-01-cargo-new
cargo run > output.txt 2>&1
cd ../../..
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

অধ্যায় 1-এ যেমন দেখেছেন, `cargo new` আপনার জন্য একটি “Hello, world!” প্রোগ্রাম
তৈরি করে দেয়। _src/main.rs_ ফাইলটি দেখুন:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

এবার `cargo run` কমান্ড দিয়ে এক ধাপেই এই “Hello, world!” প্রোগ্রামটি compile
(সংকলন) করে চালাই:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

কোনো প্রজেক্টে দ্রুত বারবার ছোট পরিবর্তন করে দেখার দরকার হলে `run` কমান্ডটি
খুব কাজে আসে। এই খেলায় আমরা ঠিক তা-ই করব: প্রতিটি ধাপ দ্রুত পরীক্ষা করে তারপর
পরের ধাপে যাব।

_src/main.rs_ ফাইলটি আবার খুলুন। সব কোড আপনি এই ফাইলেই লিখবেন।

## আন্দাজ গ্রহণ করা {#processing-a-guess}

সংখ্যা আন্দাজের খেলার প্রথম অংশটি ব্যবহারকারীর কাছে ইনপুট চাইবে, সেই ইনপুট
প্রক্রিয়া করবে, আর দেখবে ইনপুটটি প্রত্যাশিত আকারে আছে কি না। শুরুতে আমরা
খেলোয়াড়কে একটি আন্দাজ লিখতে দেব। Listing 2-1-এর কোডটি _src/main.rs_-এ লিখুন।

<Listing number="2-1" file-name="src/main.rs" caption="ব্যবহারকারীর কাছ থেকে একটি আন্দাজ নিয়ে তা প্রিন্ট করে এমন কোড">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

</Listing>

এই কোডে অনেক তথ্য আছে, তাই চলুন লাইন ধরে ধরে দেখি। ব্যবহারকারীর ইনপুট নিয়ে
তারপর ফলাফল আউটপুট হিসেবে প্রিন্ট করতে আমাদের `io` ইনপুট/আউটপুট library
(লাইব্রেরি)-টি scope (কোডের যে অংশে একটি নাম ব্যবহার করা যায়)-এ আনতে হবে।
`io` library আসে standard library (আদর্শ লাইব্রেরি) থেকে, যার নাম `std`:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

ডিফল্টভাবে standard library-তে সংজ্ঞায়িত কিছু জিনিসের একটি সেট Rust প্রতিটি
প্রোগ্রামের scope-এ নিয়ে আসে। এই সেটটিকে বলা হয় _prelude_ (প্রতিটি প্রোগ্রামে
আপনা-আপনি আসা জিনিসের সেট), আর এতে কী কী আছে, তা দেখতে পারেন
[standard library-র ডকুমেন্টেশনে][prelude]।

আপনি যে type (ডেটার ধরন) ব্যবহার করতে চান, তা prelude-এ না থাকলে একটি `use`
statement (কাজ করে কিন্তু মান তৈরি করে না এমন কোড-নির্দেশ) দিয়ে সেই type-টি
স্পষ্টভাবে scope-এ আনতে হবে। `std::io` library ব্যবহার করলে অনেক কাজের সুবিধা
পাওয়া যায়, তার মধ্যে একটি হলো ব্যবহারকারীর ইনপুট নেওয়ার ক্ষমতা।

অধ্যায় 1-এ যেমন দেখেছেন, `main` function (ফাংশন) হলো প্রোগ্রাম শুরুর জায়গা:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

`fn` syntax (ভাষার লিখনরীতি) একটি নতুন function declare (ঘোষণা) করে; বন্ধনী
`()` বোঝায় যে কোনো parameter (function-এ পাঠানো মানের জন্য রাখা নাম) নেই; আর
কার্লি ব্র্যাকেট `{` function-এর body শুরু করে।

অধ্যায় 1-এ আরও শিখেছেন যে `println!` একটি macro (কোড তৈরি করে এমন কোড), যা
স্ক্রিনে একটি string (লেখা) প্রিন্ট করে:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

এই কোডটি একটি বার্তা প্রিন্ট করছে, যা জানায় খেলাটি কী, আর ব্যবহারকারীর কাছে
ইনপুট চায়।

### variable-এ মান রাখা {#storing-values-with-variables}

এরপর আমরা ব্যবহারকারীর ইনপুট রাখার জন্য একটি _variable_ (মান রাখার নাম) তৈরি
করব, এভাবে:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

এবার প্রোগ্রামটি মজার হয়ে উঠছে! এই ছোট্ট লাইনে অনেক কিছু ঘটছে। variable তৈরি
করতে আমরা `let` statement ব্যবহার করি। আরেকটি উদাহরণ দেখুন:

```rust,ignore
let apples = 5;
```

এই লাইনটি `apples` নামে একটি নতুন variable তৈরি করে এবং সেটিকে `5` মানের সঙ্গে
bind (কোনো নামের সঙ্গে মান জুড়ে দেওয়া) করে। Rust-এ variable ডিফল্টভাবে
immutable (অপরিবর্তনীয়), অর্থাৎ variable-টিকে একবার একটি মান দিলে সেই মান আর
বদলাবে না। এই ধারণা নিয়ে আমরা অধ্যায় 3-এর
[“variable ও mutability”][variables-and-mutability]<!-- ignore -->
অংশে বিস্তারিত আলোচনা করব। কোনো variable-কে mutable (পরিবর্তনযোগ্য) করতে
variable-এর নামের আগে `mut` যোগ করি:

```rust,ignore
let apples = 5; // অপরিবর্তনীয়
let mut bananas = 5; // পরিবর্তনযোগ্য
```

> Note: `//` syntax একটি কমেন্ট শুরু করে, যা লাইনের শেষ পর্যন্ত চলে। কমেন্টের
> ভেতরের সবকিছু Rust উপেক্ষা করে। কমেন্ট নিয়ে আমরা [অধ্যায় 3][comments]<!-- ignore -->-এ
> আরও বিস্তারিত আলোচনা করব।

আবার সংখ্যা আন্দাজের প্রোগ্রামে ফিরে আসি। এখন আপনি জানেন যে `let mut guess`
`guess` নামে একটি mutable variable তৈরি করবে। সমান চিহ্ন (`=`) Rust-কে বলে যে
আমরা এখন variable-টির সঙ্গে কিছু একটা bind করতে চাই। সমান চিহ্নের ডানে আছে সেই
মান, যার সঙ্গে `guess` bind হয়েছে; মানটি হলো `String::new` call করার ফলাফল।
এটি এমন একটি function, যা একটি `String`-এর একটি নতুন instance (কোনো
type-এর একটি নির্দিষ্ট মান) return (ফেরত দেওয়া) করে।
[`String`][string]<!-- ignore --> হলো standard library-র দেওয়া একটি string
type, যা UTF-8 এনকোডিংয়ের এমন একটি লেখা, যার আকার বাড়তে পারে।

`::new` লাইনের `::` syntax বোঝায় যে `new` হলো `String` type-এর একটি associated
function। _associated function_ হলো এমন একটি function, যা কোনো type-এর ওপর
implement (বাস্তবায়ন) করা হয়েছে, এখানে `String`-এর ওপর। এই `new` function একটি
নতুন, ফাঁকা string তৈরি করে। অনেক type-এই `new` function পাবেন, কারণ কোনো
ধরনের নতুন মান তৈরি করে এমন function-এর জন্য এটি একটি প্রচলিত নাম।

সব মিলিয়ে, `let mut guess = String::new();` লাইনটি এমন একটি mutable variable
তৈরি করেছে, যা এই মুহূর্তে একটি `String`-এর নতুন, ফাঁকা instance-এর সঙ্গে bind
হয়ে আছে। উফ!

### ব্যবহারকারীর ইনপুট নেওয়া {#receiving-user-input}

মনে আছে, প্রোগ্রামের প্রথম লাইনে `use std::io;` লিখে আমরা standard library থেকে
ইনপুট/আউটপুটের সুবিধা যোগ করেছিলাম। এবার `io` module (কোডের অংশ ভাগ করার একক)
থেকে `stdin` function call করব, যা দিয়ে আমরা ব্যবহারকারীর ইনপুট সামলাতে পারব:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

প্রোগ্রামের শুরুতে `use std::io;` লিখে `io` module import না করলেও আমরা
`std::io::stdin` লিখে এই function-টি call করতে পারতাম। `stdin`
function [`std::io::Stdin`][iostdin]<!-- ignore -->-এর একটি instance return
করে; এটি এমন একটি type, যা আপনার টার্মিনালের standard input (কিবোর্ড থেকে আসা
ইনপুটের ধারা)-এর একটি handle (কোনো resource ব্যবহারের মাধ্যম) প্রকাশ করে।

এরপর `.read_line(&mut guess)` লাইনটি ব্যবহারকারীর কাছ থেকে ইনপুট নিতে standard
input handle-এর ওপর [`read_line`][read_line]<!--
ignore --> method call করে। ব্যবহারকারীর ইনপুট কোন string-এ রাখতে হবে, তা
জানাতে আমরা `read_line`-কে argument হিসেবে `&mut guess`-ও দিচ্ছি। `read_line`-এর
পুরো কাজ হলো, ব্যবহারকারী standard input-এ যা-ই টাইপ করুক, সেটি নিয়ে একটি
string-এর শেষে জুড়ে দেওয়া (string-এর আগের বিষয়বস্তু না মুছে); তাই আমরা সেই
string-টি argument হিসেবে দিই। method-টি যাতে string-এর বিষয়বস্তু বদলাতে পারে,
সেজন্য string argument-টিকে mutable হতে হবে।

`&` বোঝায় যে এই argument-টি একটি _reference_ (ডেটা কপি না করে তার দিকে নির্দেশ
করার উপায়)। reference-এর মাধ্যমে আপনার কোডের একাধিক অংশ একই ডেটা ব্যবহার করতে
পারে, সেই ডেটা মেমোরিতে বারবার কপি না করেই। reference একটি জটিল বৈশিষ্ট্য, আর
Rust-এর একটি বড় সুবিধা হলো reference ব্যবহার করা কতটা নিরাপদ ও সহজ। এই প্রোগ্রাম
শেষ করতে এসবের খুঁটিনাটি খুব বেশি জানার দরকার নেই। আপাতত শুধু এটুকু জানলেই চলবে
যে variable-এর মতো reference-ও ডিফল্টভাবে immutable। তাই reference-টিকে mutable
করতে `&guess` না লিখে `&mut guess` লিখতে হয়। (অধ্যায় 4-এ reference আরও
বিস্তারিতভাবে ব্যাখ্যা করা হবে।)

<!-- Old headings. Do not remove or links may break. -->

<a id="handling-potential-failure-with-the-result-type"></a>

### `Result` দিয়ে সম্ভাব্য ব্যর্থতা সামলানো {#handling-potential-failure-with-result}

আমরা এখনো কোডের এই লাইনটি নিয়েই কাজ করছি। এখন আমরা লেখার তৃতীয় লাইনটি নিয়ে
কথা বলছি, তবে মনে রাখবেন, এটি এখনো কোডের একটিমাত্র যৌক্তিক লাইনের অংশ। পরের
অংশটি হলো এই method:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

এই কোডটি আমরা এভাবেও লিখতে পারতাম:

```rust,ignore
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

তবে একটি লম্বা লাইন পড়া কঠিন, তাই এটিকে ভেঙে লেখাই ভালো। `.method_name()`
syntax দিয়ে method call করার সময় লম্বা লাইন ভাঙতে নতুন লাইন আর অন্যান্য ফাঁকা
অক্ষর ব্যবহার করা প্রায়ই বুদ্ধিমানের কাজ। এবার দেখা যাক এই লাইনটি কী করে।

আগেই বলা হয়েছে, ব্যবহারকারী যা লেখে, `read_line` তা আমাদের দেওয়া string-এ রাখে,
কিন্তু সেই সঙ্গে এটি একটি `Result` মানও return করে।
[`Result`][result]<!--
ignore --> হলো একটি [_enumeration_][enums]<!-- ignore --> (enum-এর পূর্ণ নাম),
যাকে প্রায়ই _enum_ (কয়েকটি সম্ভাব্য রূপের একটি নিতে পারে এমন টাইপ) বলা হয়।
enum হলো এমন একটি type, যা একাধিক সম্ভাব্য অবস্থার যেকোনো একটিতে থাকতে পারে।
প্রতিটি সম্ভাব্য অবস্থাকে আমরা বলি একটি _variant_ (enum-এর সম্ভাব্য একটি রূপ)।

[অধ্যায় 6][enums]<!-- ignore -->-এ enum নিয়ে আরও বিস্তারিত আলোচনা করা হবে। এই
`Result` type-গুলোর উদ্দেশ্য হলো error handling (ত্রুটি সামলানো)-সংক্রান্ত তথ্য
প্রকাশ করা।

`Result`-এর variant হলো `Ok` আর `Err`। `Ok` variant বোঝায় যে কাজটি সফল হয়েছে,
এবং এর ভেতরে থাকে সফলভাবে তৈরি হওয়া মানটি। `Err` variant মানে কাজটি ব্যর্থ
হয়েছে, এবং এর ভেতরে থাকে কাজটি কীভাবে বা কেন ব্যর্থ হলো, সেই তথ্য।

অন্য যেকোনো type-এর মানের মতোই `Result` type-এর মানগুলোর ওপরও method
সংজ্ঞায়িত আছে। `Result`-এর একটি instance-এ একটি [`expect` method][expect]<!-- ignore -->
আছে, যা আপনি call করতে পারেন। `Result`-এর এই instance যদি একটি `Err` মান হয়,
তাহলে `expect` প্রোগ্রামটিকে ক্র্যাশ করাবে এবং আপনি `expect`-কে argument হিসেবে যে
বার্তা দিয়েছেন, সেটি দেখাবে। `read_line` method যদি `Err` return করে, তাহলে সেটি
সম্ভবত নিচের স্তরের অপারেটিং সিস্টেম থেকে আসা কোনো error (ত্রুটি)-এর ফল। আর
`Result`-এর এই instance যদি একটি `Ok` মান হয়, তাহলে `Ok`-এর ভেতরে থাকা return
মানটি নিয়ে `expect` শুধু সেই মানটিই আপনাকে return করবে, যাতে আপনি সেটি ব্যবহার
করতে পারেন। এই ক্ষেত্রে মানটি হলো ব্যবহারকারীর ইনপুটে থাকা byte-এর সংখ্যা।

আপনি `expect` call না করলে প্রোগ্রামটি compile হবে, কিন্তু একটি সতর্কবার্তা পাবেন:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust সতর্ক করছে যে `read_line` থেকে return হওয়া `Result` মানটি আপনি ব্যবহার
করেননি, অর্থাৎ প্রোগ্রামটি একটি সম্ভাব্য error সামলায়নি।

সতর্কবার্তাটি বন্ধ করার সঠিক উপায় হলো সত্যিই error handling-এর কোড লেখা, কিন্তু
আমাদের ক্ষেত্রে আমরা চাই কোনো সমস্যা হলে প্রোগ্রামটি শুধু ক্র্যাশ করুক, তাই
`expect` ব্যবহার করতে পারি। error থেকে সামলে ওঠার উপায় শিখবেন [অধ্যায়
9][recover]<!-- ignore -->-এ।

### `println!` placeholder দিয়ে মান প্রিন্ট করা {#printing-values-with-println-placeholders}

শেষের কার্লি ব্র্যাকেটটি ছাড়া এ পর্যন্ত লেখা কোডে আলোচনার জন্য আর মাত্র একটি
লাইন বাকি:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

এই লাইনটি সেই string প্রিন্ট করে, যাতে এখন ব্যবহারকারীর ইনপুট আছে। `{}`
কার্লি ব্র্যাকেট জোড়াটি একটি placeholder (মান বসানোর জন্য রাখা জায়গা): `{}`-কে
কাঁকড়ার ছোট্ট দাঁড়ার মতো ভাবুন, যা একটি মানকে জায়গামতো ধরে রাখে। কোনো
variable-এর মান প্রিন্ট করার সময় variable-এর নামটি কার্লি ব্র্যাকেটের ভেতরে
বসানো যায়। আর কোনো expression (মান তৈরি করে এমন কোড-অংশ) evaluate (মান নির্ণয়)
করার ফলাফল প্রিন্ট করতে চাইলে format string (placeholder-সহ string)-এ ফাঁকা
কার্লি ব্র্যাকেট বসান, তারপর format string-এর পরে কমা দিয়ে আলাদা করা
expression-এর একটি তালিকা দিন; প্রতিটি ফাঁকা কার্লি ব্র্যাকেট placeholder-এ একই
ক্রমে একেকটি expression প্রিন্ট হবে। `println!`-এর একটি call-এ একটি variable আর
একটি expression-এর ফলাফল প্রিন্ট করলে তা দেখতে এরকম হবে:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

এই কোড প্রিন্ট করবে `x = 5 and y + 2 = 12`।

### প্রথম অংশটি পরীক্ষা করা {#testing-the-first-part}

চলুন সংখ্যা আন্দাজের খেলার প্রথম অংশটি পরীক্ষা করি। `cargo run` দিয়ে এটি চালান:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

এই পর্যায়ে খেলার প্রথম অংশটি তৈরি: আমরা কিবোর্ড থেকে ইনপুট নিচ্ছি আর তারপর
সেটি প্রিন্ট করছি।

## একটি গোপন সংখ্যা তৈরি করা {#generating-a-secret-number}

এরপর আমাদের একটি গোপন সংখ্যা তৈরি করতে হবে, যা ব্যবহারকারী আন্দাজ করার চেষ্টা
করবেন। গোপন সংখ্যাটি প্রতিবার আলাদা হওয়া উচিত, যাতে খেলাটি একাধিকবার খেলেও মজা
পাওয়া যায়। খেলাটি যাতে খুব কঠিন না হয়, সেজন্য আমরা 1 থেকে 100-এর মধ্যে একটি
র‍্যান্ডম সংখ্যা ব্যবহার করব। Rust-এর standard library-তে এখনো র‍্যান্ডম সংখ্যা
তৈরির সুবিধা নেই। তবে Rust টিম এই সুবিধাসহ একটি [`rand`
crate][randcrate] দেয়।

<!-- Old headings. Do not remove or links may break. -->
<a id="using-a-crate-to-get-more-functionality"></a>

### crate দিয়ে সুবিধা বাড়ানো {#increasing-functionality-with-a-crate}

মনে রাখবেন, crate হলো Rust সোর্স কোড ফাইলের একটি সংগ্রহ। আমরা যে প্রজেক্টটি
বানাচ্ছি, সেটি একটি binary crate (চালানো যায় এমন crate), অর্থাৎ একটি executable।
`rand` crate হলো একটি library crate (অন্য প্রোগ্রামে ব্যবহারের জন্য লেখা crate);
এতে এমন কোড থাকে, যা অন্য প্রোগ্রামে ব্যবহারের জন্য লেখা, এবং যা নিজে নিজে চালানো
যায় না।

বাইরের crate-গুলো সমন্বয় করার কাজেই Cargo সত্যিকারের ঝলক দেখায়। `rand` ব্যবহার
করে কোড লেখার আগে আমাদের _Cargo.toml_ ফাইলে `rand` crate-কে একটি dependency
(নির্ভরতা, অর্থাৎ যে লাইব্রেরির ওপর কোড নির্ভর করে) হিসেবে যোগ করতে হবে। ফাইলটি
এখন খুলুন, আর Cargo আপনার জন্য যে `[dependencies]` অংশের শিরোনাম তৈরি করেছে,
তার নিচে, একেবারে শেষে নিচের লাইনটি যোগ করুন। `rand` ঠিক এখানে যেভাবে আছে,
এই সংস্করণ নম্বরসহ হুবহু সেভাবেই লিখবেন, নইলে এই টিউটোরিয়ালের কোডের উদাহরণগুলো
কাজ না-ও করতে পারে:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Filename: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:8:}}
```

_Cargo.toml_ ফাইলে কোনো শিরোনামের পরে যা কিছু আসে, তা সেই অংশেরই অংশ, এবং অংশটি
চলতে থাকে পরের অংশ শুরু হওয়া পর্যন্ত। `[dependencies]`-এ আপনি Cargo-কে জানান
আপনার প্রজেক্ট কোন কোন বাইরের crate-এর ওপর নির্ভর করে, আর সেই crate-গুলোর কোন
সংস্করণ আপনার দরকার। এখানে আমরা `rand` crate-এর জন্য semantic version specifier
(সংস্করণ নির্ধারক) `0.8.5` দিয়েছি। Cargo [Semantic
Versioning][semver]<!-- ignore --> (কখনো কখনো একে _SemVer_ বলা হয়) বোঝে, যা
সংস্করণ নম্বর লেখার একটি মানদণ্ড। `0.8.5` specifier-টি আসলে `^0.8.5`-এর সংক্ষিপ্ত
রূপ, যার মানে হলো এমন যেকোনো সংস্করণ, যা অন্তত 0.8.5 কিন্তু 0.9.0-এর নিচে।

Cargo ধরে নেয় যে এই সংস্করণগুলোর public API (প্রোগ্রামিং ইন্টারফেস) 0.8.5
সংস্করণের সঙ্গে সামঞ্জস্যপূর্ণ, আর এই নির্দেশনা নিশ্চিত করে যে আপনি সর্বশেষ
patch release (ছোটখাটো সংশোধনী সংস্করণ) পাবেন, যা এখনো এই অধ্যায়ের কোডের সঙ্গে
compile হবে। 0.9.0 বা তার বেশি কোনো সংস্করণের API যে পরের উদাহরণগুলোতে ব্যবহৃত
API-র মতোই হবে, তার কোনো নিশ্চয়তা নেই।

এবার কোনো কোড না বদলে প্রজেক্টটি build (কোড compile করে চালানোর উপযোগী ফাইল
বানানো) করি, Listing 2-2-এ যেমন দেখানো হয়েছে।

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
rm Cargo.lock
cargo clean
cargo build -->

<Listing number="2-2" caption="`rand` crate-কে dependency হিসেবে যোগ করার পর `cargo build` চালানোর আউটপুট">

```console
$ cargo build
  Updating crates.io index
   Locking 15 packages to latest Rust 1.85.0 compatible versions
    Adding rand v0.8.5 (available: v0.9.0)
 Compiling proc-macro2 v1.0.93
 Compiling unicode-ident v1.0.17
 Compiling libc v0.2.170
 Compiling cfg-if v1.0.0
 Compiling byteorder v1.5.0
 Compiling getrandom v0.2.15
 Compiling rand_core v0.6.4
 Compiling quote v1.0.38
 Compiling syn v2.0.98
 Compiling zerocopy-derive v0.7.35
 Compiling zerocopy v0.7.35
 Compiling ppv-lite86 v0.2.20
 Compiling rand_chacha v0.3.1
 Compiling rand v0.8.5
 Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
  Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.48s
```

</Listing>

আপনি হয়তো আলাদা সংস্করণ নম্বর দেখবেন (তবে SemVer-এর কল্যাণে সবগুলোই কোডের সঙ্গে
সামঞ্জস্যপূর্ণ হবে!), আলাদা লাইন দেখবেন (অপারেটিং সিস্টেমের ওপর নির্ভর করে), আর
লাইনগুলোর ক্রমও আলাদা হতে পারে।

আমরা যখন কোনো বাইরের dependency যোগ করি, Cargo তখন সেই dependency-র দরকারি
সবকিছুর সর্বশেষ সংস্করণ _registry_ (crate-এর তথ্যভান্ডার) থেকে নিয়ে আসে।
registry হলো [Crates.io][cratesio]-এর ডেটার একটি কপি। Crates.io হলো সেই জায়গা,
যেখানে Rust ইকোসিস্টেমের মানুষজন নিজেদের ওপেন সোর্স Rust প্রজেক্ট অন্যদের
ব্যবহারের জন্য প্রকাশ করেন।

registry আপডেট করার পর Cargo `[dependencies]` অংশটি দেখে নেয়, আর তালিকায় থাকা
যেসব crate আগে ডাউনলোড হয়নি, সেগুলো ডাউনলোড করে। এখানে আমরা শুধু `rand`-কে
dependency হিসেবে লিখলেও `rand` কাজ করার জন্য যেসব crate-এর ওপর নির্ভর করে,
Cargo সেগুলোও নিয়ে এসেছে। crate-গুলো ডাউনলোড করার পর Rust সেগুলো compile করে,
তারপর dependency-গুলো হাতের কাছে রেখে প্রজেক্টটি compile করে।

কোনো পরিবর্তন না করে সঙ্গে সঙ্গে আবার `cargo build` চালালে `Finished` লাইনটি
ছাড়া আর কোনো আউটপুট পাবেন না। Cargo জানে যে dependency-গুলো সে আগেই ডাউনলোড ও
compile করেছে, আর আপনার _Cargo.toml_ ফাইলে আপনি সেগুলো নিয়ে কিছুই বদলাননি।
Cargo এটাও জানে যে আপনি আপনার কোডে কিছু বদলাননি, তাই সেটিও আবার compile করে
না। করার মতো কিছু না থাকায় সে শুধু বেরিয়ে যায়।

_src/main.rs_ ফাইলটি খুলে ছোট্ট একটা পরিবর্তন করে সেভ করুন, তারপর আবার build
করলে আউটপুটের মাত্র দুটি লাইন দেখবেন:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
```

এই লাইনগুলো দেখায় যে Cargo শুধু _src/main.rs_ ফাইলে আপনার ছোট্ট পরিবর্তনটুকু
দিয়ে build আপডেট করেছে। আপনার dependency-গুলো বদলায়নি, তাই Cargo জানে যে
সেগুলোর জন্য যা আগেই ডাউনলোড ও compile করা আছে, তা-ই আবার ব্যবহার করা যাবে।

<!-- Old headings. Do not remove or links may break. -->
<a id="ensuring-reproducible-builds-with-the-cargo-lock-file"></a>

#### একই build বারবার পাওয়া নিশ্চিত করা {#ensuring-reproducible-builds}

Cargo-র একটি ব্যবস্থা আছে, যা নিশ্চিত করে যে আপনি বা অন্য কেউ যতবারই আপনার কোড
build করুক, প্রতিবার একই artifact (build-এর ফলে তৈরি ফাইল) তৈরি হবে: আপনি অন্য
কিছু না বলা পর্যন্ত Cargo শুধু আপনার নির্দিষ্ট করা dependency-র সংস্করণগুলোই
ব্যবহার করবে। যেমন, ধরুন আগামী সপ্তাহে `rand` crate-এর 0.8.6 সংস্করণ বের হলো,
আর সেই সংস্করণে একটি গুরুত্বপূর্ণ বাগ ঠিক করা হয়েছে, কিন্তু সেই সঙ্গে এমন একটি
regression (আগে কাজ করা কিছু নতুন সংস্করণে ভেঙে যাওয়া)-ও আছে, যা আপনার কোড
ভেঙে দেবে। এই পরিস্থিতি সামলাতে প্রথমবার `cargo build` চালানোর সময় Rust
_Cargo.lock_ ফাইলটি তৈরি করে, তাই এখন _guessing_game_ ডিরেক্টরিতে আমাদের এই
ফাইলটি আছে।

আপনি যখন কোনো প্রজেক্ট প্রথমবার build করেন, Cargo তখন শর্ত পূরণ করে এমন সব
dependency-র সংস্করণ খুঁজে বের করে সেগুলো _Cargo.lock_ ফাইলে লিখে রাখে। পরে যখন
প্রজেক্টটি build করবেন, Cargo দেখবে যে _Cargo.lock_ ফাইলটি আছে, আর আবার
সংস্করণ খোঁজার পুরো কাজটি না করে সেখানে লেখা সংস্করণগুলোই ব্যবহার করবে। এর ফলে
আপনি আপনা-আপনিই একটি reproducible build (প্রতিবার হুবহু একই ফল দেয় এমন build)
পান। অন্যভাবে বললে, _Cargo.lock_ ফাইলের কল্যাণে আপনি নিজে স্পষ্টভাবে আপগ্রেড না
করা পর্যন্ত আপনার প্রজেক্ট 0.8.5-এই থাকবে। reproducible build-এর জন্য
_Cargo.lock_ ফাইলটি গুরুত্বপূর্ণ, তাই প্রায়ই প্রজেক্টের বাকি কোডের সঙ্গে এটিও
version control system-এ রাখা হয়।

#### নতুন সংস্করণ পেতে crate আপডেট করা {#updating-a-crate-to-get-a-new-version}

আপনি যখন সত্যিই কোনো crate আপডেট _করতে_ চান, তখনের জন্য Cargo `update` কমান্ডটি
দেয়, যা _Cargo.lock_ ফাইলটি উপেক্ষা করে _Cargo.toml_-এ আপনার শর্তের সঙ্গে মেলে
এমন সব সর্বশেষ সংস্করণ খুঁজে বের করে। তারপর Cargo সেই সংস্করণগুলো _Cargo.lock_
ফাইলে লিখে রাখে। তবে ডিফল্টভাবে Cargo শুধু 0.8.5-এর চেয়ে বড় এবং 0.9.0-এর চেয়ে
ছোট সংস্করণগুলোই খুঁজবে। `rand` crate যদি 0.8.6 আর 0.999.0 নামে দুটি নতুন
সংস্করণ প্রকাশ করে থাকে, তাহলে `cargo update` চালালে নিচের আউটপুট দেখবেন:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
assuming there is a new 0.8.x version of rand; otherwise use another update
as a guide to creating the hypothetical output shown here -->

```console
$ cargo update
    Updating crates.io index
     Locking 1 package to latest Rust 1.85.0 compatible version
    Updating rand v0.8.5 -> v0.8.6 (available: v0.999.0)
```

Cargo 0.999.0 সংস্করণটি উপেক্ষা করে। এই পর্যায়ে আপনি _Cargo.lock_ ফাইলেও একটি
পরিবর্তন লক্ষ করবেন, যেখানে লেখা থাকবে যে আপনি এখন `rand` crate-এর 0.8.6
সংস্করণ ব্যবহার করছেন। `rand`-এর 0.999.0 সংস্করণ বা 0.999._x_ সিরিজের যেকোনো
সংস্করণ ব্যবহার করতে চাইলে আপনাকে _Cargo.toml_ ফাইলটি এভাবে বদলাতে হবে (তবে
পরিবর্তনটি আসলে করবেন না, কারণ পরের উদাহরণগুলো ধরে নেয় যে আপনি `rand` 0.8
ব্যবহার করছেন):

```toml
[dependencies]
rand = "0.999.0"
```

পরের বার `cargo build` চালালে Cargo পাওয়া যায় এমন crate-গুলোর registry আপডেট
করবে, আর আপনার নির্দিষ্ট করা নতুন সংস্করণ অনুযায়ী `rand`-এর প্রয়োজনীয়তা আবার
মূল্যায়ন করবে।

[Cargo][doccargo]<!-- ignore --> আর [এর
ইকোসিস্টেম][doccratesio]<!-- ignore --> নিয়ে বলার আছে আরও অনেক কিছু, যা নিয়ে আমরা
অধ্যায় 14-এ আলোচনা করব, তবে আপাতত এটুকু জানলেই চলবে। Cargo দিয়ে library আবার
ব্যবহার করা খুব সহজ, তাই Rustacean (Rust প্রোগ্রামার)-রা অনেকগুলো package জুড়ে
তৈরি ছোট ছোট প্রজেক্ট লিখতে পারেন।

### একটি র‍্যান্ডম সংখ্যা তৈরি করা {#generating-a-random-number}

চলুন, আন্দাজ করার মতো একটি সংখ্যা তৈরি করতে `rand` ব্যবহার শুরু করি। পরের ধাপ
হলো _src/main.rs_ আপডেট করা, Listing 2-3-এ যেমন দেখানো হয়েছে।

<Listing number="2-3" file-name="src/main.rs" caption="একটি র‍্যান্ডম সংখ্যা তৈরির কোড যোগ করা">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

</Listing>

প্রথমে আমরা `use rand::Rng;` লাইনটি যোগ করি। `Rng` trait (টাইপের সাধারণ আচরণের
সংজ্ঞা) এমন কিছু method সংজ্ঞায়িত করে, যেগুলো random number generator (র‍্যান্ডম
সংখ্যা তৈরির ব্যবস্থা) implement করে; আর সেই method-গুলো ব্যবহার করতে হলে এই
trait-টিকে scope-এ থাকতে হবে। অধ্যায় 10-এ trait নিয়ে বিস্তারিত আলোচনা করা হবে।

এরপর আমরা মাঝখানে দুটি লাইন যোগ করছি। প্রথম লাইনে আমরা `rand::thread_rng`
function call করি, যা আমাদের সেই নির্দিষ্ট random number generator-টি দেয়, যা
আমরা ব্যবহার করব: এটি বর্তমান execution thread (প্রোগ্রামের ভেতরের আলাদা কাজের
ধারা)-এর নিজস্ব, আর অপারেটিং সিস্টেম একে seed (র‍্যান্ডম সংখ্যার শুরুর মান দেওয়া)
করে। তারপর আমরা random number generator-এর ওপর `gen_range` method call করি। এই
method-টি `Rng` trait-এ সংজ্ঞায়িত, যা আমরা `use rand::Rng;` statement দিয়ে
scope-এ এনেছি। `gen_range` method argument হিসেবে একটি range (একটি সীমার ভেতরের
মানগুলো) expression নেয় এবং সেই range-এর মধ্যে একটি র‍্যান্ডম সংখ্যা তৈরি করে।
এখানে আমরা যে ধরনের range expression ব্যবহার করছি, তা `start..=end` আকারের এবং
নিচের ও ওপরের দুই সীমাই এর অন্তর্ভুক্ত, তাই 1 থেকে 100-এর মধ্যে একটি সংখ্যা চাইতে
আমাদের `1..=100` লিখতে হবে।

> Note: কোনো crate থেকে কোন trait ব্যবহার করতে হবে, আর কোন method ও function
> call করতে হবে, তা আপনি এমনি এমনি জানবেন না; তাই প্রতিটি crate-এর সঙ্গে
> ব্যবহারের নির্দেশনাসহ ডকুমেন্টেশন থাকে। Cargo-র আরেকটি চমৎকার সুবিধা হলো,
> `cargo doc
> --open` কমান্ড চালালে আপনার সব dependency-র দেওয়া ডকুমেন্টেশন আপনার কম্পিউটারেই
> build হয়ে ব্রাউজারে খুলে যায়। যেমন, `rand` crate-এর অন্য সুবিধাগুলো জানতে
> আগ্রহী হলে `cargo doc --open` চালিয়ে বাঁ দিকের সাইডবারে `rand`-এ ক্লিক করুন।

নতুন দ্বিতীয় লাইনটি গোপন সংখ্যাটি প্রিন্ট করে। প্রোগ্রাম তৈরির সময় পরীক্ষা করার
জন্য এটি কাজে লাগে, তবে চূড়ান্ত সংস্করণ থেকে আমরা এটি মুছে দেব। শুরু হতেই যদি
প্রোগ্রাম উত্তরটা বলে দেয়, তাহলে আর খেলার মজা থাকে না!

কয়েকবার প্রোগ্রামটি চালিয়ে দেখুন:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 7
Please input your guess.
4
You guessed: 4

$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 83
Please input your guess.
5
You guessed: 5
```

আপনার প্রতিবার আলাদা র‍্যান্ডম সংখ্যা পাওয়ার কথা, আর সবগুলোই 1 থেকে 100-এর মধ্যে
হওয়ার কথা। দারুণ!

## আন্দাজের সঙ্গে গোপন সংখ্যা মেলানো {#comparing-the-guess-to-the-secret-number}

এখন আমাদের কাছে ব্যবহারকারীর ইনপুট আর একটি র‍্যান্ডম সংখ্যা আছে, তাই দুটি তুলনা
করতে পারি। সেই ধাপটি Listing 2-4-এ দেখানো হয়েছে। লক্ষ করুন, এই কোডটি এখনই
compile হবে না; কেন, তা আমরা ব্যাখ্যা করব।

<Listing number="2-4" file-name="src/main.rs" caption="দুটি সংখ্যা তুলনার সম্ভাব্য return মানগুলো সামলানো">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

</Listing>

প্রথমে আমরা আরেকটি `use` statement যোগ করি, যা standard library থেকে
`std::cmp::Ordering` নামের একটি type-কে scope-এ আনে। `Ordering` type-টিও একটি
enum, আর এর variant হলো `Less`, `Greater` ও `Equal`। দুটি মান তুলনা করলে এই
তিনটিই সম্ভাব্য ফলাফল।

তারপর আমরা নিচে পাঁচটি নতুন লাইন যোগ করি, যেগুলো `Ordering` type ব্যবহার করে।
`cmp` method দুটি মান তুলনা করে, আর তুলনা করা যায় এমন যেকোনো কিছুর ওপর এটি call
করা যায়। আপনি যার সঙ্গে তুলনা করতে চান, এটি তার একটি reference নেয়: এখানে এটি
`guess`-কে `secret_number`-এর সঙ্গে তুলনা করছে। তারপর এটি `Ordering` enum-এর একটি
variant return করে, যা আমরা `use` statement দিয়ে scope-এ এনেছি। `guess` আর
`secret_number`-এর মান দিয়ে `cmp` call করার পর `Ordering`-এর কোন variant return
হলো, তার ভিত্তিতে এরপর কী করা হবে, তা ঠিক করতে আমরা একটি
[`match`][match]<!-- ignore --> expression ব্যবহার করি।

একটি `match` expression কয়েকটি _arm_ (`match`-এর একেকটি শাখা) দিয়ে তৈরি। একটি
arm-এ থাকে মেলানোর জন্য একটি _pattern_ (মানের গঠনের নকশা), আর সেই কোড, যা চলবে
যদি `match`-কে দেওয়া মানটি ওই arm-এর pattern-এর সঙ্গে মেলে। Rust `match`-কে দেওয়া
মানটি নিয়ে একে একে প্রতিটি arm-এর pattern দেখে। pattern আর `match` গঠন Rust-এর
শক্তিশালী বৈশিষ্ট্য: এগুলো দিয়ে আপনার কোড যেসব নানা পরিস্থিতির মুখোমুখি হতে পারে,
তা প্রকাশ করা যায়, আর এগুলো নিশ্চিত করে যে আপনি সবগুলো পরিস্থিতিই সামলাচ্ছেন। এই
বৈশিষ্ট্যগুলো যথাক্রমে অধ্যায় 6 আর অধ্যায় 19-এ বিস্তারিত আলোচনা করা হবে।

এখানে ব্যবহৃত `match` expression দিয়ে চলুন একটি উদাহরণ ধাপে ধাপে দেখি। ধরুন,
ব্যবহারকারী 50 আন্দাজ করেছেন, আর এবার র‍্যান্ডমভাবে তৈরি গোপন সংখ্যাটি
38।

কোড যখন 50-কে 38-এর সঙ্গে তুলনা করে, তখন `cmp` method
`Ordering::Greater` return করবে, কারণ 50, 38-এর চেয়ে বড়। `match` expression
`Ordering::Greater` মানটি পায় এবং প্রতিটি arm-এর pattern পরীক্ষা করা শুরু করে।
এটি প্রথম arm-এর pattern `Ordering::Less` দেখে, এবং দেখে যে `Ordering::Greater`
মানটি `Ordering::Less`-এর সঙ্গে মেলে না, তাই ওই arm-এর কোড উপেক্ষা করে পরের
arm-এ যায়। পরের arm-এর pattern হলো `Ordering::Greater`, যা `Ordering::Greater`-এর
সঙ্গে _সত্যিই_ মেলে! ওই arm-এর কোড চলবে এবং স্ক্রিনে `Too big!` প্রিন্ট করবে।
প্রথম সফল মিলের পরই `match` expression শেষ হয়ে যায়, তাই এই ক্ষেত্রে এটি শেষ
arm-টি আর দেখবে না।

তবে Listing 2-4-এর কোড এখনো compile হবে না। চলুন চেষ্টা করে দেখি:

<!--
The error numbers in this output should be that of the code **WITHOUT** the
anchor or snip comments
-->

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

error-এর মূল কথা হলো, এখানে _mismatched types_ আছে, অর্থাৎ type মিলছে না। Rust-এর
একটি শক্তিশালী static type system (compile-এর সময়েই type নির্দিষ্ট হয় এমন
ব্যবস্থা) আছে। তবে এতে type inference (type অনুমান করে নেওয়া)-ও আছে। আমরা যখন
`let mut guess = String::new()` লিখেছিলাম, Rust তখন নিজেই infer করতে পেরেছিল যে
`guess` একটি `String` হবে, তাই আমাদের type লিখতে হয়নি। অন্যদিকে `secret_number`
একটি সংখ্যার type। Rust-এর কয়েকটি সংখ্যার type 1 থেকে 100-এর মধ্যের মান রাখতে
পারে: `i32`, একটি 32-bit সংখ্যা; `u32`, একটি unsigned (চিহ্নহীন, ঋণাত্মক হতে পারে
না) 32-bit সংখ্যা; `i64`, একটি 64-bit সংখ্যা; এবং আরও কিছু। অন্য কিছু বলে না
দিলে Rust ডিফল্টভাবে `i32` ধরে নেয়; অন্য কোথাও এমন type-এর তথ্য যোগ না করলে, যার
কারণে Rust অন্য কোনো সংখ্যার type infer করে, `secret_number`-এর type এটিই হবে। error-টির কারণ হলো, Rust একটি string আর একটি সংখ্যার type তুলনা করতে পারে না।

শেষ পর্যন্ত আমরা চাই, প্রোগ্রাম ইনপুট হিসেবে যে `String` পড়ে, তাকে একটি সংখ্যার
type-এ রূপান্তর করতে, যাতে গোপন সংখ্যার সঙ্গে সংখ্যা হিসেবে তুলনা করা যায়। এজন্য
`main` function-এর body-তে এই লাইনটি যোগ করি:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

লাইনটি হলো:

```rust,ignore
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

আমরা `guess` নামে একটি variable তৈরি করি। কিন্তু দাঁড়ান, প্রোগ্রামে কি আগে থেকেই
`guess` নামে একটি variable নেই? আছে, তবে সুবিধার ব্যাপার হলো Rust আমাদের `guess`-এর
আগের মানকে একটি নতুন মান দিয়ে shadow করতে দেয়। _shadowing_ (একই নামে নতুন variable
দিয়ে আগেরটিকে আড়াল করা) আমাদের `guess` নামটিই আবার ব্যবহার করতে দেয়, যেমন
`guess_str` আর `guess`-এর মতো দুটি আলাদা variable তৈরি করতে বাধ্য করে না। এ নিয়ে
[অধ্যায় 3][shadowing]<!-- ignore -->-এ আরও বিস্তারিত আলোচনা করব, তবে আপাতত জেনে
রাখুন, কোনো মানকে এক type থেকে আরেক type-এ রূপান্তর করতে চাইলে এই বৈশিষ্ট্যটি
প্রায়ই ব্যবহার করা হয়।

আমরা এই নতুন variable-টিকে `guess.trim().parse()` expression-এর সঙ্গে bind করি।
expression-এর `guess` বোঝায় মূল `guess` variable-টিকে, যাতে ইনপুটটি string
হিসেবে ছিল। একটি `String` instance-এর ওপর `trim` method শুরু ও শেষের সব ফাঁকা
অক্ষর মুছে দেয়। string-টিকে `u32`-তে রূপান্তর করার আগে এটি করতেই হবে, কারণ এই type
শুধু সংখ্যার ডেটা রাখতে পারে। `read_line`-কে সন্তুষ্ট করে আন্দাজটি লিখতে
ব্যবহারকারীকে <kbd>enter</kbd> চাপতে হয়, যা string-এ একটি newline (নতুন লাইনের
অক্ষর) যোগ করে। যেমন, ব্যবহারকারী <kbd>5</kbd> টাইপ করে <kbd>enter</kbd> চাপলে
`guess` দেখতে হয় এরকম: `5\n`। `\n` বোঝায় “newline”। (Windows-এ <kbd>enter</kbd>
চাপলে একটি carriage return (লাইনের শুরুতে ফেরার অক্ষর) আর একটি newline যোগ হয়:
`\r\n`।) `trim` method `\n` বা `\r\n` মুছে দেয়, ফলে থাকে শুধু `5`।

[string-এর ওপর `parse` method][parse]<!-- ignore --> একটি string-কে অন্য
type-এ রূপান্তর করে। এখানে আমরা এটি string থেকে সংখ্যায় রূপান্তরের জন্য ব্যবহার
করছি। `let guess: u32` লিখে আমাদের Rust-কে ঠিক কোন সংখ্যার type চাই, তা জানাতে
হবে। `guess`-এর পরের কোলন (`:`) Rust-কে বলে যে আমরা variable-টির type
annotate (type স্পষ্ট করে লিখে দেওয়া) করব। Rust-এ কয়েকটি বিল্ট-ইন সংখ্যার type
আছে; এখানে দেখা `u32` হলো একটি unsigned, 32-bit integer। ছোট ধনাত্মক সংখ্যার জন্য
এটি একটি ভালো ডিফল্ট পছন্দ। অন্যান্য সংখ্যার type সম্পর্কে শিখবেন
[অধ্যায় 3][integers]<!-- ignore -->-এ।

এছাড়া, এই উদাহরণ প্রোগ্রামে `u32` annotation আর `secret_number`-এর সঙ্গে তুলনার
কারণে Rust infer করবে যে `secret_number`-ও একটি `u32` হবে। ফলে এখন তুলনাটি হবে
একই type-এর দুটি মানের মধ্যে!

`parse` method শুধু এমন অক্ষরের ওপর কাজ করবে, যেগুলোকে যৌক্তিকভাবে সংখ্যায়
রূপান্তর করা যায়, তাই এতে সহজেই error হতে পারে। যেমন, string-এ যদি `A👍%` থাকে,
তাহলে সেটিকে সংখ্যায় রূপান্তরের কোনো উপায় নেই। ব্যর্থ হতে পারে বলে `parse` method
একটি `Result` type return করে, ঠিক যেমন `read_line` method করে (আগে
[“`Result` দিয়ে সম্ভাব্য ব্যর্থতা
সামলানো”](#handling-potential-failure-with-result)<!-- ignore -->-এ আলোচিত)।
আবার `expect` method ব্যবহার করে আমরা এই `Result`-ও একইভাবে সামলাব। string থেকে
সংখ্যা তৈরি করতে না পারার কারণে `parse` যদি `Result`-এর `Err` variant return
করে, তাহলে `expect` call খেলাটিকে ক্র্যাশ করাবে এবং আমাদের দেওয়া বার্তাটি প্রিন্ট
করবে। আর `parse` যদি string-টিকে সফলভাবে সংখ্যায় রূপান্তর করতে পারে, তাহলে এটি
`Result`-এর `Ok` variant return করবে, এবং `expect` সেই `Ok` মান থেকে আমাদের
কাঙ্ক্ষিত সংখ্যাটি return করবে।

এবার প্রোগ্রামটি চালাই:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
touch src/main.rs
cargo run
  76
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

চমৎকার! আন্দাজের আগে ফাঁকা জায়গা থাকলেও প্রোগ্রামটি ঠিকই বুঝে নিয়েছে যে ব্যবহারকারী
76 আন্দাজ করেছেন। বিভিন্ন ধরনের ইনপুটে প্রোগ্রামের আলাদা আচরণ যাচাই করতে কয়েকবার
প্রোগ্রামটি চালান: সঠিক সংখ্যা আন্দাজ করুন, খুব বড় একটি সংখ্যা আন্দাজ করুন, আর খুব
ছোট একটি সংখ্যা আন্দাজ করুন।

খেলার বেশিরভাগ অংশ এখন কাজ করছে, কিন্তু ব্যবহারকারী মাত্র একবার আন্দাজ করতে পারেন।
একটি loop (বারবার চলতে থাকা কোড-অংশ) যোগ করে চলুন সেটা বদলাই!

## loop দিয়ে একাধিকবার আন্দাজের সুযোগ দেওয়া {#allowing-multiple-guesses-with-looping}

`loop` keyword (ভাষার সংরক্ষিত শব্দ) একটি infinite loop (কখনো না থামা loop) তৈরি
করে। ব্যবহারকারীকে সংখ্যাটি আন্দাজ করার আরও সুযোগ দিতে আমরা একটি loop যোগ করব:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

দেখতেই পাচ্ছেন, আন্দাজ চাওয়ার বার্তা থেকে শুরু করে পরের সবকিছু আমরা একটি loop-এর
ভেতরে সরিয়ে দিয়েছি। loop-এর ভেতরের প্রতিটি লাইন আরও চার space করে ইনডেন্ট করতে
ভুলবেন না, তারপর প্রোগ্রামটি আবার চালান। প্রোগ্রামটি এখন চিরকাল নতুন আন্দাজ চাইতে
থাকবে, আর এতে আসলে একটি নতুন সমস্যা তৈরি হয়। মনে হচ্ছে, ব্যবহারকারী খেলা থেকে
বেরোতেই পারবেন না!

ব্যবহারকারী সবসময় <kbd>ctrl</kbd>-<kbd>C</kbd> কিবোর্ড শর্টকাট দিয়ে প্রোগ্রামটি
থামিয়ে দিতে পারেন। তবে এই অতৃপ্ত দানবের হাত থেকে বাঁচার আরেকটি উপায় আছে, যার কথা
[“আন্দাজের সঙ্গে গোপন সংখ্যা
মেলানো”](#comparing-the-guess-to-the-secret-number)<!-- ignore --> অংশে `parse`
নিয়ে আলোচনায় বলা হয়েছিল: ব্যবহারকারী সংখ্যা নয় এমন কোনো উত্তর লিখলে প্রোগ্রামটি
ক্র্যাশ করবে। ব্যবহারকারীকে খেলা ছাড়ার সুযোগ দিতে আমরা এর সুবিধা নিতে পারি, এখানে
যেমন দেখানো হয়েছে:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
touch src/main.rs
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit

thread 'main' panicked at src/main.rs:28:47:
Please type a number!: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`quit` লিখলে খেলা বন্ধ হবে, তবে খেয়াল করবেন, সংখ্যা নয় এমন অন্য যেকোনো ইনপুট
দিলেও তা-ই হবে। এটা মোটেও আদর্শ ব্যবস্থা নয়; আমরা চাই সঠিক সংখ্যা আন্দাজ করা
হলেও খেলাটি থেমে যাক।

### সঠিক আন্দাজের পর খেলা শেষ করা {#quitting-after-a-correct-guess}

একটি `break` statement যোগ করে চলুন খেলাটিকে এমনভাবে প্রোগ্রাম করি, যাতে
ব্যবহারকারী জিতলে খেলা শেষ হয়:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

`You win!`-এর পরে `break` লাইনটি যোগ করায় ব্যবহারকারী গোপন সংখ্যাটি সঠিকভাবে
আন্দাজ করলে প্রোগ্রামটি loop থেকে বেরিয়ে যায়। loop থেকে বেরোনো মানে প্রোগ্রাম
থেকেও বেরোনো, কারণ loop-টিই `main`-এর শেষ অংশ।

### ভুল ইনপুট সামলানো {#handling-invalid-input}

খেলার আচরণ আরও উন্নত করতে, ব্যবহারকারী সংখ্যা নয় এমন কিছু লিখলে প্রোগ্রামটি
ক্র্যাশ না করিয়ে চলুন খেলাটি সেই ইনপুট উপেক্ষা করুক, যাতে ব্যবহারকারী আন্দাজ
চালিয়ে যেতে পারেন। এটি করতে আমরা সেই লাইনটি বদলাব, যেখানে `guess`-কে `String`
থেকে `u32`-তে রূপান্তর করা হয়, Listing 2-5-এ যেমন দেখানো হয়েছে।

<Listing number="2-5" file-name="src/main.rs" caption="প্রোগ্রাম ক্র্যাশ না করিয়ে সংখ্যা নয় এমন আন্দাজ উপেক্ষা করা এবং আবার আন্দাজ চাওয়া">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

</Listing>

error হলে ক্র্যাশ করার বদলে error সামলানোর দিকে যেতে আমরা `expect` call বাদ দিয়ে
একটি `match` expression ব্যবহার করছি। মনে রাখবেন, `parse` একটি `Result` type
return করে, আর `Result` হলো একটি enum, যার variant হলো `Ok` আর `Err`। এখানে আমরা
একটি `match` expression ব্যবহার করছি, ঠিক যেমন `cmp` method-এর `Ordering`
ফলাফলের ক্ষেত্রে করেছিলাম।

`parse` যদি string-টিকে সফলভাবে সংখ্যায় রূপান্তর করতে পারে, তাহলে এটি একটি `Ok`
মান return করবে, যার ভেতরে থাকবে ফলাফলের সংখ্যাটি। সেই `Ok` মানটি প্রথম arm-এর
pattern-এর সঙ্গে মিলবে, আর `match` expression শুধু সেই `num` মানটি return করবে,
যা `parse` তৈরি করে `Ok` মানের ভেতরে রেখেছিল। সংখ্যাটি গিয়ে পৌঁছাবে ঠিক যেখানে
আমরা চাই, আমাদের তৈরি করা নতুন `guess` variable-এ।

`parse` যদি string-টিকে সংখ্যায় রূপান্তর করতে _না_ পারে, তাহলে এটি একটি `Err`
মান return করবে, যার ভেতরে error সম্পর্কে আরও তথ্য থাকবে। `Err` মানটি প্রথম
`match` arm-এর `Ok(num)` pattern-এর সঙ্গে মেলে না, কিন্তু দ্বিতীয় arm-এর `Err(_)`
pattern-এর সঙ্গে মেলে। আন্ডারস্কোর `_` হলো একটি catch-all (বাকি সব কিছুর সঙ্গে
মিলে যায় এমন) মান; এই উদাহরণে আমরা বলছি যে আমরা সব `Err` মানের সঙ্গে মেলাতে চাই,
তাদের ভেতরে যে তথ্যই থাকুক না কেন। তাই প্রোগ্রামটি দ্বিতীয় arm-এর কোড `continue`
চালাবে, যা প্রোগ্রামকে `loop`-এর পরের iteration (loop-এর একেকবার চলা)-এ গিয়ে আরেকটি
আন্দাজ চাইতে বলে। ফলে কার্যত প্রোগ্রামটি `parse`-এর সম্ভাব্য সব error উপেক্ষা করে!

এখন প্রোগ্রামের সবকিছু প্রত্যাশামতো কাজ করার কথা। চলুন চালিয়ে দেখি:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(too small guess)
(too big guess)
foo
(correct guess)
-->

```console
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

অসাধারণ! শেষ একটি ছোট্ট পরিবর্তন করলেই আমাদের সংখ্যা আন্দাজের খেলা তৈরি। মনে আছে,
প্রোগ্রামটি এখনো গোপন সংখ্যাটি প্রিন্ট করছে। পরীক্ষার সময় এটি কাজে লেগেছে, কিন্তু
এতে খেলার মজা নষ্ট হয়। চলুন গোপন সংখ্যাটি প্রিন্ট করা `println!`-টি মুছে দিই।
Listing 2-6-এ চূড়ান্ত কোডটি দেখানো হয়েছে।

<Listing number="2-6" file-name="src/main.rs" caption="সংখ্যা আন্দাজের খেলার সম্পূর্ণ কোড">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

</Listing>

এই পর্যায়ে আপনি সফলভাবে সংখ্যা আন্দাজের খেলাটি বানিয়ে ফেলেছেন। অভিনন্দন!

## সারসংক্ষেপ {#summary}

এই প্রজেক্টটি ছিল হাতে-কলমে Rust-এর অনেক নতুন ধারণার সঙ্গে আপনাকে পরিচয় করিয়ে
দেওয়ার একটি উপায়: `let`, `match`, function, বাইরের crate-এর ব্যবহার এবং আরও
অনেক কিছু। পরের কয়েকটি অধ্যায়ে এই ধারণাগুলো আরও বিস্তারিত শিখবেন। অধ্যায় 3-এ
এমন ধারণাগুলো আছে, যা বেশিরভাগ প্রোগ্রামিং ভাষাতেই থাকে, যেমন variable, ডেটা
type আর function, এবং Rust-এ এগুলো কীভাবে ব্যবহার করতে হয়, তা দেখানো হয়েছে।
অধ্যায় 4-এ দেখা হয়েছে ownership (মালিকানা), যে বৈশিষ্ট্য Rust-কে অন্য ভাষা থেকে
আলাদা করে। অধ্যায় 5-এ আলোচনা করা হয়েছে struct (নিজস্ব গঠনের ডেটা টাইপ) আর method
syntax নিয়ে, আর অধ্যায় 6-এ ব্যাখ্যা করা হয়েছে enum কীভাবে কাজ করে।

[prelude]: https://doc.rust-lang.org/std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#variables-and-mutability
[comments]: ch03-04-comments.html
[string]: https://doc.rust-lang.org/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: https://doc.rust-lang.org/cargo/
[doccratesio]: https://doc.rust-lang.org/cargo/reference/publishing.html
[match]: ch06-02-match.html
[shadowing]: ch03-01-variables-and-mutability.html#shadowing
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
[integers]: ch03-02-data-types.html#integer-types
