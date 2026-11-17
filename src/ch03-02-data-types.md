## data type {#data-types}

Rust-এর প্রতিটি মানই কোনো না কোনো _data type_ (ডেটার ধরন)-এর। data type Rust-কে
জানায় কী ধরনের ডেটা দেওয়া হচ্ছে, যাতে সে বোঝে ওই ডেটা নিয়ে কীভাবে কাজ করতে হবে।
আমরা data type-এর দুটি ভাগ দেখব: scalar আর compound।

মনে রাখবেন, Rust একটি _statically typed_ ভাষা, অর্থাৎ compile time-এই এটিকে সব
variable-এর type জানতে হয়। মান আর সেটি কীভাবে ব্যবহার করা হচ্ছে, তার ভিত্তিতে
compiler সাধারণত infer (type অনুমান করে নেওয়া) করতে পারে আমরা কোন type ব্যবহার করতে
চাই। যেখানে একাধিক type সম্ভব, যেমন অধ্যায় 2-এর [“আন্দাজের সঙ্গে গোপন সংখ্যা
মেলানো”][comparing-the-guess-to-the-secret-number]<!-- ignore --> অংশে আমরা যখন
`parse` দিয়ে একটি `String`-কে সংখ্যার type-এ রূপান্তর করেছিলাম, সেখানে আমাদের একটি
type annotation যোগ করতে হয়, এভাবে:

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

ওপরের কোডে দেখানো `: u32` type annotation যোগ না করলে Rust নিচের error দেখাবে, যার
মানে হলো আমরা কোন type ব্যবহার করতে চাই, তা জানতে compiler-এর আমাদের কাছ থেকে আরও
তথ্য দরকার:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

অন্যান্য data type-এর জন্য আলাদা আলাদা type annotation দেখতে পাবেন।

### scalar type {#scalar-types}

একটি _scalar_ type (একটিমাত্র মান প্রকাশ করে এমন type) একটিমাত্র মান প্রকাশ করে।
Rust-এ চারটি প্রধান scalar type আছে: integer (পূর্ণসংখ্যা), floating-point সংখ্যা
(দশমিক বিন্দুযুক্ত সংখ্যা), Boolean (সত্য বা মিথ্যা মান) আর অক্ষর। অন্য প্রোগ্রামিং
ভাষা থেকে হয়তো এগুলো চেনেন। চলুন দেখি Rust-এ এগুলো কীভাবে কাজ করে।

#### integer type {#integer-types}

_integer_ হলো এমন সংখ্যা, যার কোনো ভগ্নাংশ অংশ নেই। অধ্যায় 2-এ আমরা একটি integer
type ব্যবহার করেছিলাম, `u32` type। এই type ঘোষণা বোঝায় যে এর সঙ্গে যুক্ত মানটি
হবে একটি unsigned (চিহ্নহীন) integer (signed integer type-গুলো `u`-এর বদলে `i` দিয়ে
শুরু হয়), যা 32 bit জায়গা নেয়। Table 3-1-এ Rust-এর বিল্ট-ইন integer type-গুলো
দেখানো হয়েছে। কোনো integer মানের type ঘোষণা করতে এই variant-গুলোর যেকোনোটি
ব্যবহার করতে পারি।

<span class="caption">Table 3-1: Rust-এর integer type</span>

| দৈর্ঘ্য  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| architecture-নির্ভর | `isize` | `usize`  |

প্রতিটি variant হয় signed নয়তো unsigned, এবং প্রতিটির একটি নির্দিষ্ট আকার আছে।
_signed_ (চিহ্নযুক্ত) আর _unsigned_ বোঝায় সংখ্যাটি ঋণাত্মক হতে পারে কি না—অন্যভাবে
বললে, সংখ্যাটির সঙ্গে একটি চিহ্ন থাকা দরকার কি না (signed), নাকি এটি সবসময় ধনাত্মক
হবে এবং তাই চিহ্ন ছাড়াই প্রকাশ করা যায় (unsigned)। এটা কাগজে সংখ্যা লেখার মতো:
চিহ্ন গুরুত্বপূর্ণ হলে সংখ্যাটি যোগ বা বিয়োগ চিহ্নসহ লেখা হয়; কিন্তু সংখ্যাটি ধনাত্মক
বলে নিশ্চিন্তে ধরে নেওয়া গেলে কোনো চিহ্ন ছাড়াই লেখা হয়। signed সংখ্যা রাখা হয়
[two’s complement][twos-complement]<!-- ignore
--> (ঋণাত্মক সংখ্যা বাইনারিতে রাখার একটি পদ্ধতি) রূপে।

প্রতিটি signed variant −(2<sup>n − 1</sup>) থেকে 2<sup>n −
1</sup> − 1 পর্যন্ত (দুই সীমাসহ) সংখ্যা রাখতে পারে, যেখানে _n_ হলো ওই variant-এর
ব্যবহৃত bit-এর সংখ্যা। তাই একটি
`i8` রাখতে পারে −(2<sup>7</sup>) থেকে 2<sup>7</sup> − 1 পর্যন্ত, অর্থাৎ
−128 থেকে 127 পর্যন্ত সংখ্যা। unsigned variant রাখতে পারে 0 থেকে 2<sup>n</sup> − 1
পর্যন্ত, তাই একটি `u8` রাখতে পারে 0 থেকে 2<sup>8</sup> − 1, অর্থাৎ 0 থেকে 255 পর্যন্ত।

এছাড়া `isize` আর `usize` type নির্ভর করে আপনার প্রোগ্রাম যে কম্পিউটারে চলছে, তার
architecture (প্রসেসরের গঠন)-এর ওপর: 64-bit architecture-এ 64 bit, আর 32-bit
architecture-এ 32 bit।

integer literal (কোডে সরাসরি লেখা মান) Table 3-2-এ দেখানো যেকোনো রূপে লিখতে পারেন।
লক্ষ করুন, যেসব সংখ্যার literal একাধিক সংখ্যার type হতে পারে, সেগুলোতে type নির্দিষ্ট
করতে একটি type suffix (সংখ্যার শেষে জুড়ে দেওয়া type-এর নাম) দেওয়া যায়, যেমন `57u8`।
সংখ্যা সহজে পড়ার জন্য সংখ্যার literal-এ দেখার সুবিধার্থে বিভাজক হিসেবে `_`-ও ব্যবহার
করা যায়, যেমন `1_000`, যার মান হবে ঠিক `1000` লেখার মতোই।

<span class="caption">Table 3-2: Rust-এর integer literal</span>

| সংখ্যার literal  | উদাহরণ       |
| ---------------- | ------------- |
| দশমিক          | `98_222`      |
| হেক্সাডেসিমাল              | `0xff`        |
| অক্টাল            | `0o77`        |
| বাইনারি           | `0b1111_0000` |
| বাইট (শুধু `u8`) | `b'A'`        |

তাহলে কোন integer type ব্যবহার করবেন, তা কীভাবে বুঝবেন? নিশ্চিত না হলে Rust-এর
ডিফল্টগুলো দিয়ে শুরু করাই সাধারণত ভালো: integer type-এর ডিফল্ট হলো `i32`। `isize` বা
`usize` ব্যবহারের প্রধান ক্ষেত্র হলো কোনো ধরনের collection (একাধিক মান রাখার কাঠামো)-এ
index (ক্রমিক অবস্থান নম্বর) দিয়ে উপাদান নেওয়া।

> ##### integer overflow {#integer-overflow}
>
> ধরুন আপনার কাছে `u8` type-এর একটি variable আছে, যা 0 থেকে 255-এর মধ্যের মান রাখতে
> পারে। variable-টিকে যদি সেই সীমার বাইরের কোনো মানে বদলানোর চেষ্টা করেন, যেমন
> 256, তাহলে _integer overflow_ (type-এর ধারণক্ষমতার বাইরে মান চলে যাওয়া) ঘটবে, যার
> ফলে দুটি আচরণের যেকোনো একটি হতে পারে। debug mode (ডিবাগের জন্য compile)-এ compile
> করার সময় Rust integer overflow-এর জন্য এমন যাচাই যোগ করে, যার ফলে এই ঘটনা ঘটলে
> runtime-এ আপনার প্রোগ্রাম _panic_ (প্রোগ্রামের হঠাৎ থেমে যাওয়া) করে। কোনো প্রোগ্রাম
> error নিয়ে বন্ধ হয়ে গেলে Rust _panicking_ শব্দটি ব্যবহার করে; panic নিয়ে আমরা
> অধ্যায় 9-এর [“`panic!` দিয়ে অপূরণীয়
> error”][unrecoverable-errors-with-panic]<!-- ignore --> অংশে আরও গভীরভাবে আলোচনা
> করব।
>
> `--release` flag দিয়ে release mode (চূড়ান্ত, optimize করা compile)-এ compile করার
> সময় Rust panic ঘটানো integer overflow-এর যাচাইগুলো যোগ করে _না_। বরং overflow
> ঘটলে Rust _two’s complement wrapping_ করে। সংক্ষেপে, type যত বড় মান রাখতে পারে,
> তার চেয়ে বড় মান “ঘুরে” type-এর রাখা যায় এমন সবচেয়ে ছোট মানের দিকে চলে যায়। `u8`-এর
> ক্ষেত্রে 256 মানটি হয়ে যায় 0, 257 হয়ে যায় 1, এভাবে চলতে থাকে। প্রোগ্রাম panic করবে
> না, তবে variable-টির মান সম্ভবত আপনার প্রত্যাশামতো হবে না। integer overflow-এর
> wrapping (সীমা পেরোলে আবার শুরুর দিকে ঘুরে আসা) আচরণের ওপর নির্ভর করাকে একটি ভুল
> হিসেবে গণ্য করা হয়।
>
> overflow-এর সম্ভাবনা স্পষ্টভাবে সামলাতে চাইলে primitive (ভাষায় সরাসরি থাকা মৌলিক)
> সংখ্যার type-গুলোর জন্য standard library (আদর্শ লাইব্রেরি)-র দেওয়া এই method
> (টাইপের সঙ্গে যুক্ত function)-পরিবারগুলো ব্যবহার করতে পারেন:
>
> - `wrapping_*` method দিয়ে সব mode-এ wrap করুন, যেমন `wrapping_add`।
> - `checked_*` method দিয়ে overflow হলে `None` মান return (ফেরত দেওয়া) করুন।
> - `overflowing_*` method দিয়ে মানটি আর overflow হয়েছে কি না তা জানানো একটি Boolean
>   return করুন।
> - `saturating_*` method দিয়ে মানটিকে type-এর সর্বনিম্ন বা সর্বোচ্চ মানে saturate
>   (সীমায় পৌঁছে আটকে থাকা) করুন।

#### floating-point type {#floating-point-types}

_floating-point সংখ্যা_, অর্থাৎ দশমিক বিন্দুযুক্ত সংখ্যার জন্যও Rust-এ দুটি primitive
type আছে। Rust-এর floating-point type হলো `f32` আর `f64`, যাদের আকার যথাক্রমে
32 bit আর 64 bit। ডিফল্ট type হলো `f64`, কারণ আধুনিক CPU-তে এটি মোটামুটি `f32`-এর
সমান গতিতে চলে, কিন্তু বেশি নির্ভুলতা দিতে পারে। সব floating-point type-ই signed।

floating-point সংখ্যার ব্যবহার দেখানো একটি উদাহরণ:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

floating-point সংখ্যা IEEE-754 মানদণ্ড অনুযায়ী প্রকাশ করা হয়।

#### সংখ্যার operation {#numeric-operations}

সব সংখ্যার type-এর জন্য Rust প্রত্যাশিত মৌলিক গাণিতিক operation-গুলো সমর্থন করে:
যোগ, বিয়োগ, গুণ, ভাগ আর ভাগশেষ। integer ভাগের ফল শূন্যের দিকে ছেঁটে নিকটতম integer-এ
নেওয়া হয়। নিচের কোডে দেখানো হয়েছে, একটি `let` statement-এ প্রতিটি সংখ্যার operation কীভাবে ব্যবহার করবেন:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

এই statement-গুলোর প্রতিটি expression (মান তৈরি করে এমন কোড-অংশ) একটি গাণিতিক
operator (অপারেটর-চিহ্ন) ব্যবহার করে এবং একটিমাত্র মানে evaluate হয়, যা তারপর একটি
variable-এর সঙ্গে bind হয়। Rust-এর সব operator-এর তালিকা আছে [পরিশিষ্ট
B][appendix_b]<!-- ignore -->-তে।

#### Boolean type {#the-boolean-type}

অন্যান্য বেশিরভাগ প্রোগ্রামিং ভাষার মতোই Rust-এ Boolean type-এর দুটি সম্ভাব্য মান
আছে: `true` আর `false`। Boolean-এর আকার এক byte। Rust-এ Boolean type লেখা হয়
`bool` দিয়ে। যেমন:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Boolean মান ব্যবহারের প্রধান উপায় হলো conditional (শর্তের ওপর নির্ভর করে চলা কোড),
যেমন একটি `if` expression। Rust-এ `if` expression কীভাবে কাজ করে, তা আমরা
[“control flow”][control-flow]<!-- ignore --> অংশে দেখব।

#### character type {#the-character-type}

Rust-এর `char` type হলো ভাষাটির সবচেয়ে মৌলিক বর্ণভিত্তিক type। `char` মান ঘোষণার
কয়েকটি উদাহরণ:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

লক্ষ করুন, `char` literal আমরা লিখি একক উদ্ধৃতিচিহ্ন দিয়ে, আর string literal
(উদ্ধৃতিচিহ্নের ভেতরে সরাসরি লেখা string) লেখা হয় জোড়া উদ্ধৃতিচিহ্ন দিয়ে। Rust-এর
`char` type-এর আকার 4 byte এবং এটি একটি Unicode scalar value (Unicode-এর একটি একক
অক্ষর-মান) প্রকাশ করে, অর্থাৎ এটি শুধু ASCII-র চেয়ে অনেক বেশি কিছু প্রকাশ করতে পারে।
উচ্চারণ-চিহ্নযুক্ত বর্ণ; চীনা, জাপানি ও কোরিয়ান অক্ষর; ইমোজি; আর শূন্য-প্রস্থের ফাঁকা
অক্ষর—এগুলোর সবই Rust-এ বৈধ `char` মান। Unicode
scalar value-র পরিসর হলো `U+0000` থেকে `U+D7FF` এবং `U+E000` থেকে
`U+10FFFF` পর্যন্ত (দুই সীমাসহ)। তবে Unicode-এ “অক্ষর” আসলে কোনো ধারণা নয়, তাই
“অক্ষর” বলতে আপনার স্বাভাবিক বোধ Rust-এর `char`-এর সঙ্গে না-ও মিলতে পারে। এ বিষয়ে
আমরা অধ্যায় 8-এর [“string দিয়ে UTF-8
এনকোড করা লেখা রাখা”][strings]<!-- ignore --> অংশে বিস্তারিত আলোচনা করব।

{{#quiz ../quizzes/ch03-02-data-types-sec1-scalar.toml}}

### compound type {#compound-types}

_compound type_ (একাধিক মান মিলিয়ে তৈরি type) একাধিক মানকে একটি type-এ একত্র করতে
পারে। Rust-এ দুটি primitive compound type আছে: tuple আর array।

#### tuple type {#the-tuple-type}

_tuple_ (বিভিন্ন type-এর নির্দিষ্টসংখ্যক মানের একটি দল) হলো বিভিন্ন type-এর কয়েকটি
মানকে একটি compound type-এ একত্র করার একটি সাধারণ উপায়। tuple-এর দৈর্ঘ্য নির্দিষ্ট:
একবার ঘোষণা করার পর এর আকার বাড়ানো বা কমানো যায় না।

বন্ধনীর ভেতরে কমা দিয়ে আলাদা করা মানের একটি তালিকা লিখে আমরা tuple তৈরি করি। tuple-এর
প্রতিটি অবস্থানের একটি type আছে, আর tuple-এর বিভিন্ন মানের type একই হওয়ার দরকার নেই।
এই উদাহরণে আমরা ঐচ্ছিক type annotation যোগ করেছি:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

`tup` variable-টি পুরো tuple-এর সঙ্গে bind হয়, কারণ একটি tuple-কে একটিমাত্র compound
উপাদান হিসেবে গণ্য করা হয়। tuple থেকে আলাদা আলাদা মান বের করে আনতে আমরা pattern
matching (নকশার সঙ্গে মেলানো) দিয়ে একটি tuple মানকে destructure (একটি মানকে ভেঙে
আলাদা আলাদা অংশে নেওয়া) করতে পারি, এভাবে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

এই প্রোগ্রামটি প্রথমে একটি tuple তৈরি করে সেটিকে `tup` variable-এর সঙ্গে bind করে।
তারপর `let`-এর সঙ্গে একটি pattern ব্যবহার করে `tup`-কে তিনটি আলাদা variable-এ পরিণত
করে: `x`, `y` আর `z`। একে বলা হয় _destructuring_, কারণ এটি একটিমাত্র tuple-কে তিনটি
অংশে ভেঙে দেয়। সবশেষে প্রোগ্রামটি `y`-এর মান প্রিন্ট করে, যা হলো `6.4`।

একটি পূর্ণবিরাম (`.`) এবং তারপর যে মানটি নিতে চাই তার index লিখে আমরা tuple-এর কোনো
উপাদান সরাসরি নিতেও পারি। যেমন:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

এই প্রোগ্রামটি `x` tuple তৈরি করে, তারপর প্রতিটি উপাদানের নিজ নিজ index দিয়ে tuple-এর
প্রতিটি উপাদান নেয়। বেশিরভাগ প্রোগ্রামিং ভাষার মতোই tuple-এর প্রথম index হলো 0।

কোনো মান নেই এমন tuple-এর একটি বিশেষ নাম আছে: _unit_ (খালি মান)। এই মান আর এর
type—দুটোই লেখা হয় `()`, এবং এরা একটি খালি মান বা একটি খালি return type (function যে
type-এর মান ফেরত দেয়) প্রকাশ করে। কোনো expression অন্য কোনো মান return না করলে
নিজে থেকেই unit মান return করে।

এছাড়া একটি mutable tuple-এর আলাদা আলাদা উপাদান বদলাতেও পারি। যেমন:

<span class="filename">Filename: src/main.rs</span>

```rust
fn main() {
    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;
}
```

এই প্রোগ্রামটি প্রথম উপাদানকে শূন্য করে এবং দ্বিতীয় উপাদানের সঙ্গে পাঁচ যোগ করে।
`x`-এর চূড়ান্ত মান হয় `(0, 7)`।

#### array type {#the-array-type}

একাধিক মানের একটি সংগ্রহ রাখার আরেকটি উপায় হলো _array_ (একই type-এর নির্দিষ্টসংখ্যক
মানের সারি)। tuple-এর বিপরীতে, array-এর প্রতিটি উপাদানের type একই হতে হবে। অন্য কিছু
ভাষার array-র বিপরীতে Rust-এর array-র দৈর্ঘ্য নির্দিষ্ট।

array-র মানগুলো আমরা লিখি তৃতীয় বন্ধনী (square bracket)-র ভেতরে কমা দিয়ে আলাদা করা
একটি তালিকা হিসেবে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

array কাজে আসে যখন আপনি চান আপনার ডেটা heap-এ না রেখে stack-এ allocate (মেমোরিতে
জায়গা বরাদ্দ) করা হোক, এ পর্যন্ত দেখা অন্যান্য type-এর মতোই (stack (ক্রমানুসারে মান
রাখার মেমোরি-অংশ) আর heap (প্রয়োজনমতো জায়গা বরাদ্দের মেমোরি-অংশ) নিয়ে আমরা
[অধ্যায় 4][stack-and-heap]<!-- ignore -->-এ আরও আলোচনা করব), অথবা যখন নিশ্চিত হতে
চান যে আপনার কাছে সবসময় একটি নির্দিষ্টসংখ্যক উপাদান থাকবে। তবে array, vector type-এর
মতো নমনীয় নয়। vector (পরিবর্তনশীল দৈর্ঘ্যের তালিকা) হলো standard library-র দেওয়া
একই ধরনের একটি collection type, যার আকার বাড়ানো বা কমানো _যায়_, কারণ এর বিষয়বস্তু
থাকে heap-এ। array নাকি vector ব্যবহার করবেন তা নিয়ে নিশ্চিত না হলে, খুব সম্ভবত
vector-ই ব্যবহার করা উচিত। [অধ্যায়
8][vectors]<!-- ignore -->-এ vector নিয়ে আরও বিস্তারিত আলোচনা আছে।

তবে যখন জানেন উপাদানের সংখ্যা বদলানোর দরকার হবে না, তখন array বেশি কাজের। যেমন,
কোনো প্রোগ্রামে মাসের নামগুলো ব্যবহার করলে সম্ভবত vector-এর বদলে array ব্যবহার
করবেন, কারণ আপনি জানেন এতে সবসময় 12টি উপাদানই থাকবে:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

array-র type লেখা হয় তৃতীয় বন্ধনীর ভেতরে প্রতিটি উপাদানের type, একটি সেমিকোলন, আর
তারপর array-তে উপাদানের সংখ্যা দিয়ে, এভাবে:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

এখানে `i32` হলো প্রতিটি উপাদানের type। সেমিকোলনের পরের সংখ্যা `5` বোঝায় যে array-তে
পাঁচটি উপাদান আছে।

প্রতিটি উপাদানে একই মান রেখে একটি array initialize (প্রথম মান দেওয়া)-ও করতে পারেন:
তৃতীয় বন্ধনীর ভেতরে শুরুর মানটি, তারপর একটি সেমিকোলন, আর তারপর array-র দৈর্ঘ্য
লিখে, এখানে যেমন দেখানো হয়েছে:

```rust
let a = [3; 5];
```

`a` নামের array-তে `5`টি উপাদান থাকবে, যাদের প্রতিটির শুরুর মান হবে `3`। এটি
`let a = [3, 3, 3, 3, 3];` লেখার মতোই, তবে আরও সংক্ষেপে।

<!-- Old headings. Do not remove or links may break. -->
<a id="accessing-array-elements"></a>

#### array-র উপাদান নেওয়া {#array-element-access}

array হলো জানা, নির্দিষ্ট আকারের একটিমাত্র মেমোরি-খণ্ড, যা stack-এ allocate করা যায়।
index ব্যবহার করে array-র উপাদান নিতে পারেন, এভাবে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

এই উদাহরণে `first` নামের variable-টি মান পাবে `1`, কারণ array-র `[0]` index-এ
সেই মানটিই আছে। `second` নামের variable-টি array-র `[1]` index থেকে মান পাবে `2`।

#### array-র উপাদান নেওয়ার ভুল চেষ্টা {#invalid-array-element-access}

চলুন দেখি, array-র শেষের পরের কোনো উপাদান নেওয়ার চেষ্টা করলে কী হয়। ধরুন, অধ্যায়
2-এর সংখ্যা আন্দাজের খেলার মতো করে ব্যবহারকারীর কাছ থেকে একটি array index নিতে আপনি
এই কোডটি চালালেন:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,panics
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

এই কোড সফলভাবে compile হয়। `cargo run` দিয়ে কোডটি চালিয়ে `0`, `1`, `2`, `3` বা `4`
লিখলে প্রোগ্রামটি array-র ওই index-এর মানটি প্রিন্ট করবে। আর যদি তার বদলে array-র শেষের
পরের কোনো সংখ্যা লেখেন, যেমন `10`, তাহলে এরকম আউটপুট দেখবেন:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at src/main.rs:19:19:
index out of bounds: the len is 5 but the index is 10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

index operation-এ একটি অবৈধ মান ব্যবহারের জায়গায় প্রোগ্রামটিতে একটি runtime error
হয়েছে। প্রোগ্রামটি একটি error message দিয়ে বন্ধ হয়ে গেছে এবং শেষের `println!`
statement-টি চালায়নি। index দিয়ে কোনো উপাদান নেওয়ার চেষ্টা করলে Rust যাচাই করে দেখে যে
আপনার দেওয়া index array-র দৈর্ঘ্যের চেয়ে ছোট কি না। index দৈর্ঘ্যের সমান বা তার চেয়ে
বড় হলে Rust panic করবে। এই যাচাই runtime-এই হতে হয়, বিশেষ করে এই ক্ষেত্রে, কারণ পরে
কোড চালানোর সময় ব্যবহারকারী কোন মান লিখবেন, তা compiler-এর পক্ষে জানা সম্ভব নয়।

এটি কাজের মধ্যে Rust-এর memory safety (মেমোরি-নিরাপত্তা) নীতির একটি উদাহরণ। অনেক
low-level (নিম্ন-স্তরের) ভাষায় এ ধরনের যাচাই করা হয় না, আর ভুল index দিলে অবৈধ
মেমোরিতে ঢুকে পড়া সম্ভব হয়। Rust এ ধরনের error থেকে আপনাকে রক্ষা করে: মেমোরিতে
ঢোকার অনুমতি দিয়ে কাজ চালিয়ে যাওয়ার বদলে সঙ্গে সঙ্গে বন্ধ হয়ে যায়। অধ্যায় 9-এ
Rust-এর error handling (ত্রুটি সামলানো) নিয়ে আরও আলোচনা আছে, আর কীভাবে এমন পাঠযোগ্য,
নিরাপদ কোড লিখবেন, যা panic-ও করে না, অবৈধ মেমোরিতে ঢোকার সুযোগও দেয় না, তা-ও
দেখানো হয়েছে।

{{#quiz ../quizzes/ch03-02-data-types-sec2-compound.toml}}

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[twos-complement]: https://en.wikipedia.org/wiki/Two%27s_complement
[control-flow]: ch03-05-control-flow.html#control-flow
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[wrapping]: https://doc.rust-lang.org/std/num/struct.Wrapping.html
[appendix_b]: appendix-02-operators.md
