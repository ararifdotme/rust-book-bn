## heap-এর ডেটা নির্দেশ করতে `Box<T>` ব্যবহার {#using-boxt-to-point-to-data-on-the-heap}

সবচেয়ে সোজাসাপ্টা smart pointer হলো box, যার type লেখা হয়
`Box<T>`। _Boxes_ (box) দিয়ে ডেটা stack-এর বদলে heap-এ রাখা যায়। stack-এ থেকে যায় heap-এর ডেটার দিকে pointer-টি। stack আর heap-এর পার্থক্য আবার দেখে
নিতে অধ্যায় 4 দেখুন।

ডেটা stack-এর বদলে heap-এ রাখা ছাড়া box-এর performance-এ কোনো বাড়তি খরচ নেই। তবে এদের খুব বেশি বাড়তি ক্ষমতাও নেই। এই পরিস্থিতিগুলোতে এগুলো সবচেয়ে
বেশি ব্যবহার করবেন:

- যখন আপনার এমন একটি type আছে যার আকার compile time-এ জানা যায় না, আর আপনি সেই type-এর একটি মান এমন প্রেক্ষাপটে ব্যবহার করতে চান যেখানে নির্দিষ্ট আকার
  দরকার
- যখন আপনার কাছে অনেক ডেটা আছে, আর আপনি ownership হস্তান্তর করতে চান কিন্তু নিশ্চিত হতে চান যে তা করার সময় ডেটা কপি হবে না
- যখন আপনি একটি মানের মালিক হতে চান, এবং এটি নির্দিষ্ট কোনো type কি না তা নয়, বরং শুধু এটুকু নিয়ে মাথা ঘামান যে এটি একটি নির্দিষ্ট trait implement করে

প্রথম পরিস্থিতিটি আমরা [“box দিয়ে recursive type সম্ভব
করা”](#enabling-recursive-types-with-boxes)<!-- ignore -->-এ দেখাব। দ্বিতীয় ক্ষেত্রে, অনেক ডেটার ownership হস্তান্তর করতে অনেক সময় লাগতে পারে, কারণ ডেটা
stack-এ এদিক-ওদিক কপি হয়। এই পরিস্থিতিতে performance উন্নত করতে অনেক ডেটা একটি box-এ করে heap-এ রাখতে পারি। তখন শুধু অল্প pointer ডেটা stack-এ এদিক-ওদিক
কপি হয়, আর এটি যে ডেটাকে নির্দেশ করে তা heap-এ এক জায়গায় থাকে। তৃতীয় ক্ষেত্রটি _trait object_ নামে পরিচিত, আর অধ্যায় 18-এর [“Using Trait Objects to Abstract over Shared
Behavior”][trait-objects]<!-- ignore --> অংশটি পুরোপুরি এই বিষয় নিয়ে। তাই এখানে যা শিখবেন, সেই অংশে আবার তা কাজে লাগাবেন!

<!-- Old headings. Do not remove or links may break. -->

<a id="using-boxt-to-store-data-on-the-heap"></a>

### heap-এ ডেটা রাখা {#storing-data-on-the-heap}

`Box<T>`-এর heap-এ রাখার ব্যবহার নিয়ে আলোচনার আগে syntax আর `Box<T>`-এর ভেতরে রাখা মানের সঙ্গে কীভাবে কাজ করতে হয় তা দেখব।

Listing 15-1-এ দেখানো হয়েছে কীভাবে একটি box ব্যবহার করে heap-এ একটি `i32` মান রাখা যায়।

<Listing number="15-1" file-name="src/main.rs" caption="box ব্যবহার করে heap-এ একটি `i32` মান রাখা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-01/src/main.rs}}
```

</Listing>

আমরা `b` variable-কে এমন একটি `Box`-এর মান দিয়ে সংজ্ঞায়িত করি, যা heap-এ allocate করা `5` মানকে নির্দেশ করে। এই প্রোগ্রাম print করবে `b = 5`; এই ক্ষেত্রে
box-এর ভেতরের ডেটা আমরা প্রায় সেভাবেই পেতে পারি, যেভাবে ডেটাটি stack-এ থাকলে পেতাম। যেকোনো owned মানের মতোই, একটি box scope-এর বাইরে গেলে, যেমন `main`-এর
শেষে `b` যায়, এটি deallocate হবে। deallocation ঘটে box (stack-এ রাখা) আর এটি যে ডেটাকে নির্দেশ করে (heap-এ রাখা), দুটিরই।

heap-এ একটিমাত্র মান রাখা খুব একটা কাজের নয়, তাই এভাবে আলাদাভাবে box খুব একটা ব্যবহার করবেন না। বেশিরভাগ পরিস্থিতিতে একটি `i32`-এর মতো মানগুলো stack-এ
রাখাই বেশি উপযুক্ত, যেখানে এগুলো default-ভাবে রাখা হয়। চলুন এমন একটি ক্ষেত্র দেখি যেখানে box আমাদের এমন type সংজ্ঞায়িত করতে দেয়, যা box না থাকলে সংজ্ঞায়িত
করতে পারতাম না।

### box দিয়ে recursive type সম্ভব করা {#enabling-recursive-types-with-boxes}

একটি _recursive type_ (নিজের ভেতরে নিজেকেই ধারণ করে এমন type)-এর মানের নিজের অংশ হিসেবে একই type-এর আরেকটি মান থাকতে পারে। recursive type একটি সমস্যা
তৈরি করে, কারণ compile time-এ Rust-এর জানা দরকার একটি type কতটা জায়গা নেয়। কিন্তু তাত্ত্বিকভাবে recursive type-এর মানগুলোর একটির ভেতরে আরেকটি অসীম পর্যন্ত
চলতে পারে, তাই মানটির কতটা জায়গা লাগবে তা Rust জানতে পারে না। box-এর আকার যেহেতু জানা, তাই recursive type-এর সংজ্ঞায় একটি box বসিয়ে recursive type সম্ভব
করা যায়।

recursive type-এর উদাহরণ হিসেবে চলুন cons list দেখি। এটি functional programming ভাষায় সচরাচর পাওয়া একটি data type। আমরা যে cons list type সংজ্ঞায়িত করব,
recursion ছাড়া তা সোজাসাপ্টা; তাই যে উদাহরণ নিয়ে কাজ করব তার ধারণাগুলো recursive type সংক্রান্ত আরও জটিল পরিস্থিতিতে যখনই পড়বেন তখনই কাজে লাগবে।

<!-- Old headings. Do not remove or links may break. -->

<a id="more-information-about-the-cons-list"></a>

#### cons list বোঝা {#understanding-the-cons-list}

_cons list_ হলো Lisp প্রোগ্রামিং ভাষা ও এর উপভাষাগুলো থেকে আসা একটি data structure, যা একটির ভেতরে আরেকটি জোড়া দিয়ে তৈরি, এবং এটি linked list-এর Lisp
সংস্করণ। এর নাম এসেছে Lisp-এর `cons` function (_construct
function_-এর সংক্ষেপ) থেকে, যা তার দুটি argument থেকে একটি নতুন জোড়া তৈরি করে। একটি মান আর আরেকটি জোড়া নিয়ে গঠিত জোড়ার ওপর `cons` call করে আমরা recursive
জোড়া দিয়ে তৈরি cons list গড়তে পারি।

যেমন, `1, 2, 3` তালিকাযুক্ত একটি cons list-এর pseudocode (কাল্পনিক কোডে লেখা) রূপ এখানে দেওয়া হলো, প্রতিটি জোড়া বন্ধনীর ভেতরে:

```text
(1, (2, (3, Nil)))
```

cons list-এর প্রতিটি item-এ দুটি উপাদান থাকে: বর্তমান item-এর মান আর পরের item। তালিকার শেষ item-এ পরের item ছাড়া শুধু `Nil` নামের একটি মান থাকে। `cons`
function recursive-ভাবে call করে একটি cons list তৈরি হয়। recursion-এর base case (যেখানে recursion থামে) বোঝানোর প্রচলিত নাম হলো `Nil`। খেয়াল করুন, এটি অধ্যায়
6-এ আলোচিত “null” বা “nil” ধারণার মতো নয়, যা একটি অবৈধ বা অনুপস্থিত মান।

cons list Rust-এ সচরাচর ব্যবহৃত data structure নয়। Rust-এ item-এর তালিকা থাকলে বেশিরভাগ সময় `Vec<T>` ব্যবহার করাই ভালো। অন্যান্য, আরও জটিল recursive data
type বিভিন্ন পরিস্থিতিতে _সত্যিই_ কাজে লাগে, কিন্তু এই অধ্যায়ে cons list দিয়ে শুরু করলে খুব বেশি মনোযোগ না সরিয়েই দেখা যায় box কীভাবে আমাদের একটি recursive
data type সংজ্ঞায়িত করতে দেয়।

Listing 15-2-এ একটি cons list-এর জন্য একটি enum-এর সংজ্ঞা আছে। খেয়াল করুন, এই কোড এখনো compile হবে না, কারণ `List` type-এর আকার জানা নেই, যা আমরা দেখাব।

<Listing number="15-2" file-name="src/main.rs" caption="`i32` মানের একটি cons list data structure বোঝাতে enum সংজ্ঞায়িত করার প্রথম চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-02/src/main.rs:here}}
```

</Listing>

> Note: এই উদাহরণের উদ্দেশ্যে আমরা এমন একটি cons list implement করছি যা শুধু `i32` মান রাখে। অধ্যায় 10-এ যেমন আলোচনা করেছি, generics ব্যবহার করে আমরা এটি
> এমনভাবে implement করতে পারতাম যাতে cons list type যেকোনো type-এর মান রাখতে পারে।

`List` type ব্যবহার করে `1, 2, 3` তালিকাটি রাখলে কোড দেখতে Listing 15-3-এর মতো হবে।

<Listing number="15-3" file-name="src/main.rs" caption="`1, 2, 3` তালিকাটি রাখতে `List` enum ব্যবহার করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-03/src/main.rs:here}}
```

</Listing>

প্রথম `Cons` মান রাখে `1` আর আরেকটি `List` মান। এই `List` মানটি আরেকটি `Cons` মান, যা রাখে `2` আর আরেকটি `List` মান। এই `List` মানটি আরও একটি `Cons` মান,
যা রাখে `3` আর একটি `List` মান, যা শেষ পর্যন্ত `Nil`, অর্থাৎ recursive নয় এমন variant, যা তালিকার শেষ বোঝায়।

Listing 15-3-এর কোড compile করার চেষ্টা করলে Listing 15-4-এ দেখানো error পাই।

<Listing number="15-4" caption="একটি recursive enum সংজ্ঞায়িত করার চেষ্টায় পাওয়া error">

```console
{{#include ../listings/ch15-smart-pointers/listing-15-03/output.txt}}
```

</Listing>

error দেখায় এই type-এর “has infinite size” (আকার অসীম)। কারণ আমরা `List`-কে এমন একটি variant দিয়ে সংজ্ঞায়িত করেছি যা recursive: এটি সরাসরি নিজেরই আরেকটি মান
রাখে। ফলে একটি `List` মান রাখতে কতটা জায়গা লাগবে তা Rust বের করতে পারে না। চলুন বিশ্লেষণ করি কেন এই error পাই। প্রথমে দেখব recursive নয় এমন type-এর একটি মান
রাখতে কতটা জায়গা লাগবে তা Rust কীভাবে ঠিক করে।

#### recursive নয় এমন type-এর আকার হিসাব করা {#computing-the-size-of-a-non-recursive-type}

মনে করুন, অধ্যায় 6-এ enum-এর সংজ্ঞা নিয়ে আলোচনার সময় Listing 6-2-এ সংজ্ঞায়িত `Message` enum-টি:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

একটি `Message` মানের জন্য কতটা জায়গা allocate করতে হবে তা ঠিক করতে Rust প্রতিটি variant দেখে কোন variant-এর সবচেয়ে বেশি জায়গা লাগে। Rust দেখে যে
`Message::Quit`-এর কোনো জায়গা লাগে না, `Message::Move`-এর দুটি `i32` মান রাখার মতো জায়গা লাগে, ইত্যাদি। যেহেতু মাত্র একটি variant ব্যবহৃত হবে, তাই একটি
`Message` মানের সর্বোচ্চ যতটা জায়গা লাগবে, তা হলো এর সবচেয়ে বড় variant রাখতে যতটা জায়গা লাগে।

এবার এর সঙ্গে তুলনা করুন, Listing 15-2-এর `List` enum-এর মতো একটি recursive type-এর কতটা জায়গা লাগবে তা নির্ধারণ করতে গেলে Rust-এর কী হয়। compiler শুরু করে
`Cons` variant দেখে, যা `i32` type-এর একটি মান আর `List` type-এর একটি মান রাখে। তাই `Cons`-এর একটি `i32`-এর আকার আর একটি `List`-এর আকারের যোগফলের সমান জায়গা
লাগে। `List` type-এর কতটা মেমোরি লাগবে তা বের করতে compiler variant-গুলো দেখে, শুরু করে `Cons` variant থেকে। `Cons` variant `i32` type-এর একটি মান আর `List`
type-এর একটি মান রাখে, আর এই প্রক্রিয়া অসীম পর্যন্ত চলতে থাকে, যেমনটি Figure 15-1-এ দেখানো হয়েছে।

<img alt="একটি অসীম Cons list: 'Cons' লেখা একটি আয়তক্ষেত্র, যা দুটি ছোট আয়তক্ষেত্রে ভাগ করা। প্রথম ছোট আয়তক্ষেত্রে লেখা 'i32', আর দ্বিতীয় ছোট আয়তক্ষেত্রে লেখা 'Cons' এবং বাইরের 'Cons' আয়তক্ষেত্রের একটি ছোট সংস্করণ। 'Cons' আয়তক্ষেত্রগুলো নিজেদের আরও ছোট ছোট সংস্করণ ধারণ করতে থাকে, যতক্ষণ না সবচেয়ে ছোট আয়তক্ষেত্রে একটি অসীম-চিহ্ন থাকে, যা বোঝায় এই পুনরাবৃত্তি চিরকাল চলতে থাকে।" src="img/trpl15-01.svg" class="center" style="width: 50%;" />

<span class="caption">Figure 15-1: অসীম সংখ্যক `Cons` variant দিয়ে গঠিত
একটি অসীম `List`</span>

<!-- Old headings. Do not remove or links may break. -->

<a id="using-boxt-to-get-a-recursive-type-with-a-known-size"></a>

#### জানা আকারের একটি recursive type পাওয়া {#getting-a-recursive-type-with-a-known-size}

recursive-ভাবে সংজ্ঞায়িত type-এর জন্য কতটা জায়গা allocate করতে হবে তা Rust বের করতে পারে না বলে compiler এই কাজের পরামর্শসহ একটি error দেয়:

<!-- manual-regeneration
after doing automatic regeneration, look at listings/ch15-smart-pointers/listing-15-03/output.txt and copy the relevant line
-->

```text
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
2 |     Cons(i32, Box<List>),
  |               ++++    +
```

এই পরামর্শে _indirection_ (পরোক্ষভাবে নির্দেশ) মানে হলো, একটি মান সরাসরি না রেখে data structure বদলে মানটির দিকে একটি pointer রেখে পরোক্ষভাবে মানটি রাখা
উচিত।

`Box<T>` যেহেতু একটি pointer, তাই একটি `Box<T>`-এর কতটা জায়গা লাগে তা Rust সবসময় জানে: একটি pointer-এর আকার এটি কতটা ডেটাকে নির্দেশ করছে তার ওপর নির্ভর
করে বদলায় না। এর মানে আমরা `Cons` variant-এর ভেতরে সরাসরি আরেকটি `List` মানের বদলে একটি `Box<T>` রাখতে পারি। `Box<T>` পরের `List` মানটিকে নির্দেশ করবে, যা
`Cons` variant-এর ভেতরে নয়, heap-এ থাকবে। ধারণাগতভাবে আমাদের এখনো একটি তালিকা আছে, যা অন্য তালিকা ধারণ করা তালিকা দিয়ে তৈরি, কিন্তু এই implementation এখন
item-গুলোকে একটির ভেতরে আরেকটি না রেখে একটির পাশে আরেকটি রাখার মতো।

Listing 15-2-এর `List` enum-এর সংজ্ঞা আর Listing 15-3-এর `List`-এর ব্যবহার বদলে Listing 15-5-এর কোড করতে পারি, যা compile হবে।

<Listing number="15-5" file-name="src/main.rs" caption="জানা আকার পেতে `Box<T>` ব্যবহার করা `List`-এর সংজ্ঞা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-05/src/main.rs}}
```

</Listing>

`Cons` variant-এর লাগে একটি `i32`-এর আকার আর box-এর pointer ডেটা রাখার জায়গা। `Nil` variant কোনো মান রাখে না, তাই stack-এ এর `Cons` variant-এর চেয়ে কম জায়গা
লাগে। এখন আমরা জানি যেকোনো `List` মান একটি `i32`-এর আকার আর একটি box-এর pointer ডেটার আকারের সমান জায়গা নেবে। box ব্যবহার করে আমরা অসীম, recursive শিকলটি
ভেঙে দিয়েছি, তাই একটি `List` মান রাখতে কত আকার লাগবে তা compiler বের করতে পারে। Figure 15-2-এ দেখানো হয়েছে `Cons` variant এখন দেখতে কেমন।

<img alt="'Cons' লেখা একটি আয়তক্ষেত্র, যা দুটি ছোট আয়তক্ষেত্রে ভাগ করা। প্রথম ছোট আয়তক্ষেত্রে লেখা 'i32', আর দ্বিতীয় ছোট আয়তক্ষেত্রে লেখা 'Box', যার ভেতরে 'usize' লেখা একটি আয়তক্ষেত্র আছে, যা box-এর pointer-এর সসীম আকার বোঝায়।" src="img/trpl15-02.svg" class="center" />

<span class="caption">Figure 15-2: এমন একটি `List`, যার আকার অসীম নয়,
কারণ `Cons` একটি `Box` রাখে</span>

box শুধু indirection আর heap allocation দেয়; অন্য smart pointer type-এ যেসব বিশেষ ক্ষমতা দেখব, এদের তেমন কোনো ক্ষমতা নেই। সেই বিশেষ ক্ষমতাগুলোর জন্য যে
performance খরচ হয়, তা-ও এদের নেই, তাই cons list-এর মতো ক্ষেত্রে এরা কাজে লাগতে পারে, যেখানে indirection-ই আমাদের একমাত্র দরকারি বৈশিষ্ট্য। box-এর আরও
ব্যবহার দেখব অধ্যায় 18-এ।

`Box<T>` type একটি smart pointer, কারণ এটি `Deref` trait implement করে, যার ফলে `Box<T>` মানকে reference-এর মতো ব্যবহার করা যায়। একটি `Box<T>` মান scope-এর
বাইরে গেলে box যে heap ডেটাকে নির্দেশ করছে সেটিও পরিষ্কার হয়ে যায়, `Drop` trait-এর implementation-এর কারণে। এই অধ্যায়ের বাকি অংশে যে অন্য smart pointer
type-গুলো নিয়ে আলোচনা করব, তাদের কার্যক্ষমতার জন্য এই দুটি trait আরও বেশি গুরুত্বপূর্ণ হবে। চলুন এই দুটি trait আরও বিস্তারিত দেখি।

{{#quiz ../quizzes/ch15-01-box.toml}}

[trait-objects]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
