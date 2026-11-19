## unsafe Rust {#unsafe-rust}

এ পর্যন্ত আলোচিত সব কোডে Rust-এর memory safety-র নিশ্চয়তা compile time-এ প্রয়োগ করা হয়েছে। তবে Rust-এর ভেতরে দ্বিতীয় একটি ভাষা লুকিয়ে আছে, যা এই memory safety-র নিশ্চয়তা প্রয়োগ করে না: একে
বলা হয় _unsafe Rust_, আর এটি সাধারণ Rust-এর মতোই কাজ করে, তবে আমাদের কিছু বাড়তি অতিমানবীয় ক্ষমতা দেয়।

unsafe Rust আছে কারণ স্বভাবতই static analysis রক্ষণশীল। কোড নিশ্চয়তাগুলো বজায় রাখে কি না তা নির্ধারণের চেষ্টা করার সময় compiler-এর জন্য কিছু অবৈধ প্রোগ্রাম গ্রহণ করার চেয়ে কিছু বৈধ প্রোগ্রাম বাতিল
করা ভালো। কোড _হয়তো_ ঠিক আছে, কিন্তু আস্থা রাখার মতো যথেষ্ট তথ্য Rust compiler-এর কাছে না থাকলে এটি কোডটি বাতিল করবে। এসব ক্ষেত্রে unsafe কোড ব্যবহার করে compiler-কে বলতে পারেন, “আমাকে
বিশ্বাস করো, আমি জানি কী করছি।” তবে সতর্ক থাকুন, unsafe Rust আপনি নিজের ঝুঁকিতে ব্যবহার করছেন: unsafe কোড ভুলভাবে ব্যবহার করলে memory unsafety-র কারণে সমস্যা হতে পারে, যেমন null pointer
dereference করা।

Rust-এর একটি unsafe দ্বিতীয় সত্তা থাকার আরেকটি কারণ হলো, ভেতরের কম্পিউটার hardware স্বভাবতই unsafe। Rust আপনাকে unsafe operation করতে না দিলে কিছু কাজ আপনি করতে পারতেন না। Rust-কে আপনাকে
নিম্ন-স্তরের systems programming করতে দিতে হয়, যেমন সরাসরি operating system-এর সঙ্গে কাজ করা, বা এমনকি নিজের operating system লেখা। নিম্ন-স্তরের systems programming নিয়ে কাজ করা ভাষাটির একটি
লক্ষ্য। চলুন দেখি unsafe Rust দিয়ে কী করা যায় এবং কীভাবে করতে হয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="unsafe-superpowers"></a>

### unsafe অতিমানবীয় ক্ষমতা প্রয়োগ করা {#performing-unsafe-superpowers}

unsafe Rust-এ যেতে `unsafe` keyword ব্যবহার করে unsafe কোড রাখার জন্য একটি নতুন block শুরু করুন। unsafe Rust-এ এমন পাঁচটি কাজ করা যায় যা safe Rust-এ করা যায় না, যেগুলোকে আমরা বলি _unsafe superpowers_
(unsafe অতিমানবীয় ক্ষমতা)। এই ক্ষমতাগুলোর মধ্যে আছে:

1. একটি raw pointer dereference করা।
1. একটি unsafe function বা method call করা।
1. একটি mutable static variable-এ প্রবেশ বা তা পরিবর্তন করা।
1. একটি unsafe trait implement করা।
1. `union`-এর field-এ প্রবেশ করা।

বোঝা গুরুত্বপূর্ণ যে `unsafe` borrow checker বন্ধ করে না বা Rust-এর অন্য কোনো নিরাপত্তা যাচাই নিষ্ক্রিয় করে না: unsafe কোডে একটি reference ব্যবহার করলে সেটি তবুও যাচাই হবে। `unsafe` keyword শুধু
এই পাঁচটি বৈশিষ্ট্যে প্রবেশাধিকার দেয়, যেগুলো তখন memory safety-র জন্য compiler যাচাই করে না। একটি unsafe block-এর ভেতরেও আপনি কিছু মাত্রায় নিরাপত্তা পাবেন।

তা ছাড়া `unsafe` মানে এই নয় যে block-এর ভেতরের কোড অবশ্যই বিপজ্জনক বা এতে নিশ্চিতভাবে memory safety-র সমস্যা থাকবে: উদ্দেশ্য হলো প্রোগ্রামার হিসেবে আপনি নিশ্চিত করবেন যে একটি `unsafe`
block-এর ভেতরের কোড বৈধ উপায়ে মেমোরিতে প্রবেশ করবে।

মানুষ ভুল করে, ভুল হবেই, কিন্তু এই পাঁচটি unsafe operation-কে `unsafe` দিয়ে annotate করা block-এর ভেতরে রাখা বাধ্যতামূলক করায় আপনি জানবেন যে memory safety সংক্রান্ত যেকোনো error অবশ্যই কোনো `unsafe`
block-এর ভেতরে। `unsafe` block ছোট রাখুন; পরে মেমোরি-বাগ খুঁজতে গিয়ে কৃতজ্ঞ থাকবেন।

unsafe কোড যতটা সম্ভব আলাদা করে রাখতে এমন কোডকে একটি নিরাপদ abstraction-এর ভেতরে রেখে একটি নিরাপদ API দেওয়াই ভালো, যা নিয়ে অধ্যায়ের পরে unsafe function আর method দেখার সময় আলোচনা করব।
standard library-র কিছু অংশ যাচাই-করা unsafe কোডের ওপর নিরাপদ abstraction হিসেবে implement করা। unsafe কোডকে একটি নিরাপদ abstraction-এ মুড়ে দিলে `unsafe`-এর ব্যবহার সেই সব জায়গায় ছড়িয়ে
পড়ে না, যেখানে আপনি বা আপনার ব্যবহারকারীরা `unsafe` কোড দিয়ে implement করা কার্যক্ষমতা ব্যবহার করতে চাইতে পারেন, কারণ একটি নিরাপদ abstraction ব্যবহার করা নিরাপদ।

চলুন পাঁচটি unsafe অতিমানবীয় ক্ষমতার প্রতিটি একে একে দেখি। unsafe কোডের জন্য একটি নিরাপদ interface দেয় এমন কিছু abstraction-ও দেখব।

### একটি raw pointer dereference করা {#dereferencing-a-raw-pointer}

অধ্যায় 4-এর [“borrow checker permission-এর লঙ্ঘন খুঁজে বের করে”][permission-violations]<!-- ignore -->-এ আমরা
বর্ণনা করেছিলাম compiler কীভাবে নিশ্চিত করে যে reference সবসময় বৈধ। unsafe Rust-এ _raw pointers_ (raw pointer, কাঁচা pointer) নামের দুটি নতুন type আছে, যা reference-এর মতো। reference-এর মতোই raw
pointer immutable বা mutable হতে পারে এবং যথাক্রমে লেখা হয় `*const
T` আর `*mut T` হিসেবে। তারকাচিহ্নটি dereference operator নয়; এটি type-এর নামের অংশ। raw pointer-এর প্রেক্ষাপটে _immutable_ মানে dereference করার পর pointer-এ সরাসরি মান বসানো যায় না।

reference আর smart pointer থেকে আলাদা, raw pointer:

- একই জায়গার immutable আর mutable দুই ধরনের pointer, বা একাধিক mutable pointer রেখে borrowing-এর নিয়ম উপেক্ষা করতে পারে
- বৈধ মেমোরিকে নির্দেশ করবে এমন নিশ্চয়তা নেই
- null হতে পারে
- কোনো স্বয়ংক্রিয় পরিষ্কারের ব্যবস্থা implement করে না

Rust-কে দিয়ে এই নিশ্চয়তাগুলো প্রয়োগ করানো থেকে বেরিয়ে এসে আপনি নিশ্চিত নিরাপত্তা ছেড়ে দিয়ে বিনিময়ে বেশি performance বা এমন অন্য ভাষা বা hardware-এর সঙ্গে যুক্ত হওয়ার ক্ষমতা পেতে পারেন,
যেখানে Rust-এর নিশ্চয়তা প্রযোজ্য নয়।

Listing 20-1-এ দেখানো হয়েছে কীভাবে একটি immutable আর একটি mutable raw pointer তৈরি করতে হয়।

<Listing number="20-1" caption="raw borrow operator দিয়ে raw pointer তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-01/src/main.rs:here}}
```

</Listing>

খেয়াল করুন, এই কোডে আমরা `unsafe` keyword রাখিনি। নিরাপদ কোডে raw pointer তৈরি করা যায়; শুধু unsafe block-এর বাইরে raw pointer dereference করা যায় না, যা একটু পরেই দেখবেন।

আমরা raw borrow operator ব্যবহার করে raw pointer তৈরি করেছি: `&raw const num` একটি `*const i32` immutable raw pointer তৈরি করে, আর `&raw mut num` একটি `*mut
i32` mutable raw pointer তৈরি করে। এগুলো সরাসরি একটি local variable থেকে তৈরি করেছি বলে আমরা জানি এই নির্দিষ্ট raw pointer-গুলো বৈধ, কিন্তু যেকোনো raw pointer সম্পর্কে এই ধারণা করা যায় না।

এটি দেখাতে এরপর আমরা এমন একটি raw pointer তৈরি করব যার বৈধতা সম্পর্কে অতটা নিশ্চিত হওয়া যায় না, raw borrow operator ব্যবহার না করে একটি মান cast করতে `as` keyword ব্যবহার করে। Listing 20-2-এ
দেখানো হয়েছে কীভাবে মেমোরির একটি ইচ্ছামতো জায়গার দিকে raw pointer তৈরি করতে হয়। ইচ্ছামতো মেমোরি ব্যবহারের চেষ্টা অনির্ধারিত: সেই ঠিকানায় ডেটা থাকতে পারে, না-ও থাকতে পারে, compiler কোডটি এমনভাবে
optimize করতে পারে যে কোনো মেমোরি প্রবেশই ঘটে না, অথবা প্রোগ্রাম একটি segmentation fault দিয়ে বন্ধ হয়ে যেতে পারে। সাধারণত এমন কোড লেখার কোনো ভালো কারণ নেই, বিশেষ করে যেখানে এর বদলে একটি raw
borrow operator ব্যবহার করা যায়, তবে এটি সম্ভব।

<Listing number="20-2" caption="একটি ইচ্ছামতো মেমোরি ঠিকানার দিকে raw pointer তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-02/src/main.rs:here}}
```

</Listing>

মনে করুন, নিরাপদ কোডে raw pointer তৈরি করা যায়, কিন্তু raw pointer dereference করে নির্দেশিত ডেটা পড়া যায় না। Listing 20-3-এ আমরা একটি raw pointer-এর ওপর dereference operator `*` ব্যবহার করি,
যার জন্য একটি `unsafe` block লাগে।

<Listing number="20-3" caption="একটি `unsafe` block-এর ভেতরে raw pointer dereference করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-03/src/main.rs:here}}
```

</Listing>

pointer তৈরি করায় কোনো ক্ষতি নেই; কেবল যখন এটি যে মানকে নির্দেশ করে তাতে প্রবেশের চেষ্টা করি, তখনই হয়তো একটি অবৈধ মান সামলাতে হতে পারে।

আরও খেয়াল করুন, Listing 20-1 আর 20-3-এ আমরা `*const i32` আর `*mut
i32` raw pointer তৈরি করেছি, যা দুটিই একই মেমোরির জায়গাকে নির্দেশ করত, যেখানে `num` রাখা আছে। এর বদলে `num`-এর একটি immutable আর একটি mutable reference তৈরির চেষ্টা করলে কোড compile হতো না, কারণ
Rust-এর ownership-এর নিয়ম কোনো immutable reference থাকা অবস্থায় একই সময়ে একটি mutable reference অনুমোদন করে না। raw pointer দিয়ে একই জায়গার একটি mutable pointer আর একটি immutable pointer তৈরি করে
mutable pointer দিয়ে ডেটা বদলানো যায়, যাতে সম্ভবত data race তৈরি হয়। সাবধান!

এত সব বিপদ থাকলে কেন raw pointer ব্যবহার করবেন? একটি প্রধান ব্যবহার হলো C কোডের সঙ্গে যুক্ত হওয়া, যা পরের অংশে দেখবেন। আরেকটি হলো এমন নিরাপদ abstraction তৈরি করা, যা borrow checker বোঝে না।
আমরা unsafe function-এর সঙ্গে পরিচয় করিয়ে তারপর unsafe কোড ব্যবহার করা একটি নিরাপদ abstraction-এর উদাহরণ দেখব।

### একটি unsafe function বা method call করা {#calling-an-unsafe-function-or-method}

unsafe block-এ আপনি দ্বিতীয় যে ধরনের operation করতে পারেন তা হলো unsafe function call করা। unsafe function আর method দেখতে হুবহু সাধারণ function আর method-এর মতো, তবে সংজ্ঞার বাকি অংশের আগে
একটি বাড়তি `unsafe` থাকে। এই প্রেক্ষাপটে `unsafe` keyword বোঝায় যে function-টির এমন কিছু শর্ত আছে যা এই function call করার সময় আমাদের মানতে হবে, কারণ আমরা শর্তগুলো মেনেছি কি না Rust তার
নিশ্চয়তা দিতে পারে না। একটি `unsafe` block-এর ভেতরে একটি unsafe function call করে আমরা বলছি যে এই function-এর documentation পড়েছি এবং function-এর চুক্তি মেনে চলার দায়িত্ব নিচ্ছি।

এখানে `dangerous` নামের একটি unsafe function দেওয়া হলো, যা এর body-তে কিছুই করে না:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-01-unsafe-fn/src/main.rs:here}}
```

`dangerous` function আমাদের একটি আলাদা `unsafe` block-এর ভেতরে call করতে হবে। `unsafe` block ছাড়া `dangerous` call করার চেষ্টা করলে error পাব:

```console
{{#include ../listings/ch20-advanced-features/output-only-01-missing-unsafe/output.txt}}
```

`unsafe` block দিয়ে আমরা Rust-কে জোর দিয়ে বলছি যে function-এর documentation পড়েছি, একে কীভাবে ঠিকমতো ব্যবহার করতে হয় তা বুঝি, এবং যাচাই করেছি যে function-এর চুক্তি পূরণ করছি।

একটি `unsafe` function-এর body-তে unsafe operation করতে তবুও একটি `unsafe` block ব্যবহার করতে হয়, ঠিক সাধারণ function-এর ভেতরের মতো, আর ভুলে গেলে compiler সতর্ক করবে। এতে `unsafe` block যতটা সম্ভব
ছোট রাখা যায়, কারণ পুরো function body জুড়ে হয়তো unsafe operation দরকার নেই।

#### unsafe কোডের ওপর একটি নিরাপদ abstraction তৈরি করা {#creating-a-safe-abstraction-over-unsafe-code}

একটি function-এ unsafe কোড আছে বলেই পুরো function-কে unsafe হিসেবে চিহ্নিত করতে হবে এমন নয়। আসলে unsafe কোডকে একটি নিরাপদ function-এ মুড়ে দেওয়া একটি প্রচলিত abstraction। উদাহরণ হিসেবে চলুন
standard library-র `split_at_mut` function দেখি, যার জন্য কিছু unsafe কোড লাগে। আমরা দেখব একে কীভাবে implement করা যেতে পারে। এই নিরাপদ method mutable slice-এর ওপর সংজ্ঞায়িত: এটি একটি slice নিয়ে
argument হিসেবে দেওয়া index-এ ভাগ করে দুটি slice বানায়। Listing 20-4-এ দেখানো হয়েছে কীভাবে `split_at_mut` ব্যবহার করতে হয়।

<Listing number="20-4" caption="নিরাপদ `split_at_mut` function ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-04/src/main.rs:here}}
```

</Listing>

শুধু safe Rust দিয়ে এই function implement করা যায় না। একটি চেষ্টা দেখতে মোটামুটি Listing 20-5-এর মতো হতে পারে, যা compile হবে না। সরলতার জন্য আমরা `split_at_mut`-কে method-এর বদলে একটি function
হিসেবে implement করব, এবং generic type `T`-এর বদলে শুধু `i32` মানের slice-এর জন্য।

<Listing number="20-5" caption="শুধু safe Rust ব্যবহার করে `split_at_mut` implement করার একটি চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-05/src/main.rs:here}}
```

</Listing>

এই function প্রথমে slice-এর মোট দৈর্ঘ্য নেয়। তারপর parameter হিসেবে দেওয়া index দৈর্ঘ্যের চেয়ে কম বা সমান কি না যাচাই করে assert করে যে index-টি slice-এর ভেতরে। assertion-এর মানে, slice ভাগ করার
জন্য দৈর্ঘ্যের চেয়ে বড় কোনো index পাঠালে function-টি সেই index ব্যবহারের চেষ্টা করার আগেই panic করবে।

তারপর একটি tuple-এ দুটি mutable slice return করি: একটি মূল slice-এর শুরু থেকে `mid` index পর্যন্ত, আরেকটি `mid` থেকে slice-এর শেষ পর্যন্ত।

Listing 20-5-এর কোড compile করার চেষ্টা করলে error পাব:

```console
{{#include ../listings/ch20-advanced-features/listing-20-05/output.txt}}
```

Rust-এর borrow checker বুঝতে পারে না যে আমরা slice-এর ভিন্ন ভিন্ন অংশ borrow করছি; এটি শুধু জানে যে একই slice থেকে দুবার borrow করছি। একটি slice-এর ভিন্ন ভিন্ন অংশ borrow করা মৌলিকভাবে ঠিক আছে,
কারণ দুটি slice একে অপরের ওপর পড়ছে না, কিন্তু Rust এটা জানার মতো যথেষ্ট বুদ্ধিমান নয়। যখন আমরা জানি কোড ঠিক আছে কিন্তু Rust জানে না, তখনই unsafe কোডের দ্বারস্থ হওয়ার সময়।

Listing 20-6-এ দেখানো হয়েছে `split_at_mut`-এর implementation কাজ করাতে কীভাবে একটি `unsafe` block, একটি raw pointer আর কয়েকটি unsafe function call ব্যবহার করতে হয়।

<Listing number="20-6" caption="`split_at_mut` function-এর implementation-এ unsafe কোড ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-06/src/main.rs:here}}
```

</Listing>

অধ্যায় 4-এর [“slice type”][the-slice-type]<!-- ignore --> অংশ থেকে মনে করুন, একটি slice হলো কিছু ডেটার দিকে একটি pointer আর slice-এর দৈর্ঘ্য। একটি slice-এর দৈর্ঘ্য পেতে আমরা `len` method আর
একটি slice-এর raw pointer পেতে `as_mut_ptr` method ব্যবহার করি। এই ক্ষেত্রে আমাদের কাছে `i32` মানের একটি mutable slice আছে বলে `as_mut_ptr` `*mut i32` type-এর একটি raw pointer return করে, যা
আমরা `ptr` variable-এ রেখেছি।

`mid` index slice-এর ভেতরে, এই assertion আমরা রেখে দিই। তারপর আসি unsafe কোডে: `slice::from_raw_parts_mut` function একটি raw pointer আর একটি দৈর্ঘ্য নিয়ে একটি slice তৈরি করে। আমরা এই function দিয়ে
এমন একটি slice তৈরি করি যা `ptr` থেকে শুরু এবং `mid` সংখ্যক item লম্বা। তারপর `mid` থেকে শুরু হওয়া একটি raw pointer পেতে `ptr`-এর ওপর argument হিসেবে `mid` দিয়ে `add` method call করি, আর সেই
pointer আর `mid`-এর পরের বাকি item-এর সংখ্যাকে দৈর্ঘ্য হিসেবে নিয়ে একটি slice তৈরি করি।

`slice::from_raw_parts_mut` function unsafe, কারণ এটি একটি raw pointer নেয় এবং ভরসা রাখতে হয় যে pointer-টি বৈধ। raw pointer-এর ওপর `add` method-ও unsafe, কারণ ভরসা রাখতে হয় যে offset করা
জায়গাটিও একটি বৈধ pointer। তাই `slice::from_raw_parts_mut` আর `add` call করতে পারার জন্য এদের call-গুলোর চারপাশে একটি `unsafe` block রাখতে হয়েছে। কোডটি দেখে এবং `mid`-কে `len`-এর চেয়ে কম বা
সমান হতে হবে এই assertion যোগ করে আমরা বলতে পারি যে `unsafe` block-এর ভেতরে ব্যবহৃত সব raw pointer slice-এর ভেতরের ডেটার বৈধ pointer হবে। এটি `unsafe`-এর একটি গ্রহণযোগ্য ও উপযুক্ত ব্যবহার।

খেয়াল করুন, ফলাফলের `split_at_mut` function-কে `unsafe` হিসেবে চিহ্নিত করতে হয় না, এবং safe Rust থেকে এই function call করা যায়। আমরা unsafe কোডের একটি নিরাপদ abstraction তৈরি করেছি, এমন একটি
function-এর implementation দিয়ে যা নিরাপদ উপায়ে `unsafe` কোড ব্যবহার করে, কারণ এটি শুধু এই function-এর নাগালে থাকা ডেটা থেকে বৈধ pointer তৈরি করে।

বিপরীতে, Listing 20-7-এ `slice::from_raw_parts_mut`-এর ব্যবহার slice ব্যবহারের সময় সম্ভবত crash করবে। এই কোড মেমোরির একটি ইচ্ছামতো জায়গা নিয়ে 10,000 item লম্বা একটি slice তৈরি করে।

<Listing number="20-7" caption="মেমোরির একটি ইচ্ছামতো জায়গা থেকে একটি slice তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-07/src/main.rs:here}}
```

</Listing>

এই ইচ্ছামতো জায়গার মেমোরির মালিক আমরা নই, আর এই কোডের তৈরি slice-এ বৈধ `i32` মান থাকবে এমন কোনো নিশ্চয়তা নেই। `values`-কে একটি বৈধ slice হিসেবে ব্যবহারের চেষ্টা করলে undefined behavior হয়।

#### বাইরের কোড call করতে `extern` function ব্যবহার করা {#using-extern-functions-to-call-external-code}

কখনো কখনো আপনার Rust কোডকে অন্য ভাষায় লেখা কোডের সঙ্গে কাজ করতে হতে পারে। এজন্য Rust-এ `extern` keyword আছে, যা একটি _Foreign Function Interface (FFI)_ (বিদেশি function-এর interface) তৈরি ও ব্যবহার
সহজ করে; এটি এমন একটি উপায়, যাতে একটি প্রোগ্রামিং ভাষা function সংজ্ঞায়িত করে এবং ভিন্ন (বিদেশি) একটি প্রোগ্রামিং ভাষাকে সেই function-গুলো call করতে দেয়।

Listing 20-8 দেখায় C standard library-র `abs` function-এর সঙ্গে কীভাবে সংযোগ তৈরি করতে হয়। `extern` block-এর ভেতরে ঘোষিত function Rust কোড থেকে call করা সাধারণত unsafe, তাই `extern` block-কেও
`unsafe` হিসেবে চিহ্নিত করতে হয়। কারণ অন্য ভাষা Rust-এর নিয়ম ও নিশ্চয়তা প্রয়োগ করে না, আর Rust সেগুলো যাচাই করতে পারে না, তাই নিরাপত্তা নিশ্চিত করার দায়িত্ব প্রোগ্রামারের ওপর পড়ে।

<Listing number="20-8" file-name="src/main.rs" caption="অন্য ভাষায় সংজ্ঞায়িত একটি `extern` function ঘোষণা ও call করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-08/src/main.rs}}
```

</Listing>

`unsafe extern "C"` block-এর ভেতরে আমরা অন্য ভাষার সেই বাইরের function-গুলোর নাম আর signature লিখি, যা call করতে চাই। `"C"` অংশটি বলে দেয় বাইরের function কোন _application binary interface (ABI)_
ব্যবহার করে: ABI assembly স্তরে function কীভাবে call করতে হয় তা সংজ্ঞায়িত করে। `"C"` ABI সবচেয়ে প্রচলিত এবং C প্রোগ্রামিং ভাষার ABI মেনে চলে। Rust যত ABI সমর্থন করে, সেগুলোর তথ্য
[Rust Reference-এ][ABI] পাওয়া যায়।

একটি `unsafe extern` block-এর ভেতরে ঘোষিত প্রতিটি item অপ্রকাশ্যভাবে unsafe। তবে কিছু FFI function call করা _নিরাপদ_। যেমন, C-এর standard library-র `abs` function-এ memory safety নিয়ে বিবেচনার
কিছু নেই, আর আমরা জানি যেকোনো `i32` দিয়ে এটি call করা যায়। এমন ক্ষেত্রে `safe` keyword ব্যবহার করে বলা যায় যে এই নির্দিষ্ট function call করা নিরাপদ, যদিও এটি একটি `unsafe extern` block-এ আছে।
এই পরিবর্তন করলে এটি call করতে আর `unsafe` block লাগে না, যেমনটি Listing 20-9-এ দেখানো হয়েছে।

<Listing number="20-9" file-name="src/main.rs" caption="একটি `unsafe extern` block-এর ভেতরে একটি function-কে স্পষ্টভাবে `safe` হিসেবে চিহ্নিত করে নিরাপদে call করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-09/src/main.rs}}
```

</Listing>

একটি function-কে `safe` হিসেবে চিহ্নিত করলেই এটি নিজে থেকে নিরাপদ হয়ে যায় না! বরং এটি Rust-কে দেওয়া আপনার একটি প্রতিশ্রুতির মতো যে এটি নিরাপদ। সেই প্রতিশ্রুতি রাখা হচ্ছে কি না তা নিশ্চিত করা
এখনো আপনার দায়িত্ব!

#### অন্য ভাষা থেকে Rust function call করা {#calling-rust-functions-from-other-languages}

অন্য ভাষাকে Rust function call করতে দেয় এমন একটি interface তৈরি করতেও `extern` ব্যবহার করা যায়। পুরো একটি `extern` block তৈরি না করে আমরা সংশ্লিষ্ট function-এর `fn` keyword-এর ঠিক আগে `extern`
keyword যোগ করে ব্যবহার্য ABI উল্লেখ করি। Rust compiler-কে এই function-এর নাম mangle না করতে বলতে একটি `#[unsafe(no_mangle)]` annotation-ও যোগ করতে হয়। _Mangling_ হলো যখন compiler আমাদের দেওয়া
function-এর নাম বদলে এমন একটি ভিন্ন নাম দেয়, যাতে compilation প্রক্রিয়ার অন্য অংশের ব্যবহারের জন্য আরও তথ্য থাকে, কিন্তু মানুষের পক্ষে পড়া কঠিন। প্রতিটি প্রোগ্রামিং ভাষার compiler একটু ভিন্নভাবে
নাম mangle করে, তাই একটি Rust function-কে অন্য ভাষা থেকে নাম ধরে ডাকতে পারার জন্য Rust compiler-এর name mangling বন্ধ করতে হয়। এটি unsafe, কারণ built-in mangling ছাড়া library-গুলোর মধ্যে নামের
সংঘাত হতে পারে, তাই আমাদের বেছে নেওয়া নাম mangling ছাড়া export করা নিরাপদ কি না তা নিশ্চিত করা আমাদের দায়িত্ব।

নিচের উদাহরণে আমরা `call_from_c` function-কে C কোড থেকে ব্যবহারযোগ্য করি, একটি shared library হিসেবে compile করে C থেকে link করার পর:

```
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

`extern`-এর এই ব্যবহারে `unsafe` লাগে শুধু attribute-এ, `extern` block-এ নয়।

### একটি mutable static variable-এ প্রবেশ বা পরিবর্তন করা {#accessing-or-modifying-a-mutable-static-variable}

এই বইয়ে আমরা এখনো global variable নিয়ে কথা বলিনি, যা Rust সমর্থন করে, তবে Rust-এর ownership-এর নিয়মের সঙ্গে সমস্যা তৈরি করতে পারে। দুটি thread একই mutable global variable-এ প্রবেশ করলে data race
হতে পারে।

Rust-এ global variable-কে বলা হয় _static_ variable। Listing 20-10-এ মান হিসেবে একটি string slice-সহ একটি static variable ঘোষণা ও ব্যবহারের উদাহরণ দেখানো হয়েছে।

<Listing number="20-10" file-name="src/main.rs" caption="একটি immutable static variable সংজ্ঞায়িত ও ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-10/src/main.rs}}
```

</Listing>

static variable constant-এর মতো, যা নিয়ে অধ্যায় 3-এর [“constant ঘোষণা করা”][constants]<!-- ignore --> অংশে আলোচনা করেছি। রীতি অনুযায়ী static variable-এর নাম `SCREAMING_SNAKE_CASE`-এ লেখা হয়।
static variable শুধু `'static` lifetime-এর reference রাখতে পারে, অর্থাৎ Rust compiler lifetime বের করতে পারে এবং আমাদের স্পষ্টভাবে annotate করতে হয় না। একটি immutable static variable-এ প্রবেশ নিরাপদ।

constant আর immutable static variable-এর একটি সূক্ষ্ম পার্থক্য হলো, static variable-এর মানের মেমোরিতে একটি নির্দিষ্ট ঠিকানা থাকে। মানটি ব্যবহার করলে সবসময় একই ডেটায় প্রবেশ হবে। অন্যদিকে constant
যখনই ব্যবহার হয় তখনই এর ডেটা নকল হতে পারে। আরেকটি পার্থক্য হলো static variable mutable হতে পারে। mutable static variable-এ প্রবেশ ও পরিবর্তন _unsafe_। Listing 20-11-এ দেখানো হয়েছে `COUNTER`
নামের একটি mutable static variable কীভাবে ঘোষণা, প্রবেশ ও পরিবর্তন করতে হয়।

<Listing number="20-11" file-name="src/main.rs" caption="একটি mutable static variable থেকে পড়া বা তাতে লেখা unsafe।">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-11/src/main.rs}}
```

</Listing>

সাধারণ variable-এর মতোই `mut` keyword দিয়ে mutability উল্লেখ করি। `COUNTER` থেকে পড়ে বা তাতে লেখে এমন যেকোনো কোডকে একটি `unsafe` block-এর ভেতরে থাকতে হবে। Listing 20-11-এর কোড compile হয় এবং
প্রত্যাশামতো print করে `COUNTER: 3`, কারণ এটি single threaded। একাধিক thread `COUNTER`-এ প্রবেশ করলে সম্ভবত data race হতো, তাই এটি undefined behavior। তাই পুরো function-কে `unsafe` হিসেবে
চিহ্নিত করে নিরাপত্তার সীমাবদ্ধতা লিখে রাখতে হবে, যাতে function call করা যে কেউ জানে নিরাপদে কী করা যায় আর কী যায় না।

যখনই একটি unsafe function লিখি, তখন `SAFETY` দিয়ে শুরু হওয়া একটি comment লেখা প্রচলিত রীতি, যাতে ব্যাখ্যা থাকে function নিরাপদে call করতে caller-কে কী করতে হবে। একইভাবে যখনই একটি unsafe
operation করি, তখন নিরাপত্তার নিয়মগুলো কীভাবে বজায় রাখা হচ্ছে তা ব্যাখ্যা করতে `SAFETY` দিয়ে শুরু হওয়া একটি comment লেখা প্রচলিত রীতি।

তা ছাড়া একটি compiler lint (সম্ভাব্য সমস্যার সতর্কতা)-এর মাধ্যমে compiler default-ভাবে একটি mutable static variable-এর reference তৈরির যেকোনো চেষ্টা বাতিল করবে। আপনাকে হয় একটি
`#[allow(static_mut_refs)]` annotation যোগ করে স্পষ্টভাবে সেই lint-এর সুরক্ষা থেকে বেরিয়ে আসতে হবে, নয়তো raw borrow operator-গুলোর একটি দিয়ে তৈরি raw pointer-এর মাধ্যমে mutable static variable-এ
প্রবেশ করতে হবে। এর মধ্যে সেই ক্ষেত্রগুলোও আছে যেখানে reference অদৃশ্যভাবে তৈরি হয়, যেমন এই listing-এর `println!`-এ ব্যবহারের সময়। static mutable variable-এর reference raw pointer দিয়ে তৈরি করা বাধ্যতামূলক
করায় এগুলো ব্যবহারের নিরাপত্তার শর্ত আরও স্পষ্ট হয়।

বিশ্বব্যাপী প্রবেশযোগ্য mutable ডেটার ক্ষেত্রে কোনো data race নেই তা নিশ্চিত করা কঠিন, এ কারণেই Rust mutable static variable-কে unsafe মনে করে। যেখানে সম্ভব, অধ্যায় 16-এ আলোচিত concurrency কৌশল আর
thread-safe smart pointer ব্যবহার করাই ভালো, যাতে compiler যাচাই করে যে বিভিন্ন thread থেকে ডেটায় প্রবেশ নিরাপদে হচ্ছে।

### একটি unsafe trait implement করা {#implementing-an-unsafe-trait}

একটি unsafe trait implement করতে `unsafe` ব্যবহার করা যায়। একটি trait unsafe হয় যখন এর অন্তত একটি method-এর এমন কোনো invariant থাকে যা compiler যাচাই করতে পারে না। `trait`-এর আগে `unsafe` keyword যোগ
করে এবং trait-এর implementation-কেও `unsafe` হিসেবে চিহ্নিত করে আমরা ঘোষণা করি যে একটি trait `unsafe`, যেমনটি Listing 20-12-এ দেখানো হয়েছে।

<Listing number="20-12" caption="একটি unsafe trait সংজ্ঞায়িত ও implement করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-12/src/main.rs:here}}
```

</Listing>

`unsafe impl` ব্যবহার করে আমরা প্রতিশ্রুতি দিচ্ছি যে compiler যে invariant যাচাই করতে পারে না সেগুলো আমরা বজায় রাখব।

উদাহরণ হিসেবে অধ্যায় 16-এর [“`Send` ও `Sync` দিয়ে বাড়ানো যায় এমন concurrency”][send-and-sync]<!-- ignore -->
অংশে আলোচিত `Send` আর `Sync` marker trait-এর কথা মনে করুন: আমাদের type যদি পুরোপুরি `Send` আর `Sync` implement করা অন্য type দিয়ে গঠিত হয়, তাহলে compiler এই trait-গুলো স্বয়ংক্রিয়ভাবে implement
করে। যদি এমন একটি type implement করি, যাতে `Send` বা `Sync` implement করে না এমন একটি type আছে, যেমন raw pointer, এবং সেই type-কে `Send` বা `Sync` হিসেবে চিহ্নিত করতে চাই, তাহলে `unsafe` ব্যবহার করতে
হবে। আমাদের type নিরাপদে thread-এর মধ্যে পাঠানো বা একাধিক thread থেকে প্রবেশ করার নিশ্চয়তা বজায় রাখে কি না তা Rust যাচাই করতে পারে না; তাই সেই যাচাইগুলো আমাদের হাতে করতে হবে এবং `unsafe` দিয়ে তা
জানাতে হবে।

### একটি union-এর field-এ প্রবেশ করা {#accessing-fields-of-a-union}

শুধু `unsafe` দিয়েই কাজ করে এমন শেষ কাজটি হলো একটি union-এর field-এ প্রবেশ করা। একটি *union* একটি `struct`-এর মতো, তবে একটি নির্দিষ্ট instance-এ একবারে শুধু একটি ঘোষিত field ব্যবহৃত হয়। union মূলত C
কোডের union-এর সঙ্গে যুক্ত হতে ব্যবহৃত হয়। union-এর field-এ প্রবেশ unsafe, কারণ union instance-এ এই মুহূর্তে কোন type-এর ডেটা রাখা আছে তা Rust নিশ্চিত করতে পারে না। union সম্পর্কে আরও জানতে পারেন
[Rust Reference-এ][unions]।

### unsafe কোড যাচাই করতে Miri ব্যবহার করা {#using-miri-to-check-unsafe-code}

unsafe কোড লেখার সময় হয়তো যাচাই করতে চাইবেন যে যা লিখেছেন তা আসলেই নিরাপদ ও সঠিক। এটা করার একটি সেরা উপায় হলো Miri ব্যবহার করা, যা undefined behavior শনাক্ত করার জন্য Rust-এর একটি দাপ্তরিক
tool। borrow checker যেখানে compile time-এ কাজ করা একটি _static_ tool, Miri সেখানে runtime-এ কাজ করা একটি _dynamic_ tool। এটি আপনার প্রোগ্রাম বা এর test suite চালিয়ে কোড যাচাই করে, এবং Rust কীভাবে
কাজ করা উচিত সে সম্পর্কে এর বোঝা নিয়মগুলো আপনি কখন ভঙ্গ করেন তা শনাক্ত করে।

Miri ব্যবহার করতে Rust-এর একটি nightly build লাগে (যা নিয়ে [Appendix G: How Rust is Made and “Nightly Rust”][nightly]<!-- ignore -->-এ আরও কথা বলি)। `rustup
+nightly component add miri` লিখে Rust-এর একটি nightly সংস্করণ আর Miri tool দুটিই install করতে পারেন। এতে আপনার project Rust-এর কোন সংস্করণ ব্যবহার করে তা বদলায় না; এটি শুধু আপনার system-এ tool-টি
যোগ করে, যাতে চাইলে ব্যবহার করতে পারেন। `cargo +nightly miri run` বা
`cargo +nightly miri test` লিখে একটি project-এ Miri চালাতে পারেন।

এটি কতটা কাজের হতে পারে তার একটি উদাহরণ হিসেবে দেখুন Listing 20-7-এর ওপর এটি চালালে কী ঘটে।

```console
{{#include ../listings/ch20-advanced-features/listing-20-07/output.txt}}
```

Miri সঠিকভাবে আমাদের সতর্ক করে যে আমরা একটি integer-কে pointer-এ cast করছি, যা একটি সমস্যা হতে পারে, কিন্তু সমস্যা আছে কি না Miri নির্ধারণ করতে পারে না, কারণ এটি জানে না pointer-টি কোথা থেকে
এসেছে। তারপর Listing 20-7-এ যেখানে undefined behavior আছে সেখানে Miri একটি error return করে, কারণ আমাদের একটি dangling pointer আছে। Miri-র কল্যাণে আমরা এখন জানি যে undefined behavior-এর ঝুঁকি আছে,
আর কোড কীভাবে নিরাপদ করা যায় তা ভাবতে পারি। কিছু ক্ষেত্রে Miri এমনকি error কীভাবে ঠিক করতে হয় সে বিষয়ে পরামর্শও দিতে পারে।

unsafe কোড লেখার সময় আপনি যত ভুল করতে পারেন, Miri তার সবকিছু ধরে না। Miri একটি dynamic analysis tool, তাই এটি শুধু আসলে চলা কোডের সমস্যা ধরে। এর মানে, আপনার লেখা unsafe কোড সম্পর্কে আস্থা
বাড়াতে ভালো testing কৌশলের সঙ্গে মিলিয়ে এটি ব্যবহার করতে হবে। আপনার কোড যত রকমে অনিরাপদ (unsound) হতে পারে, Miri তার সবকিছুও ঢাকে না।

অন্যভাবে বললে: Miri একটি সমস্যা _ধরলে_ আপনি জানবেন একটি বাগ আছে, কিন্তু Miri কোনো বাগ _ধরেনি_ বলেই যে কোনো সমস্যা নেই তা নয়। তবে এটি অনেক কিছু ধরতে পারে। এই অধ্যায়ের unsafe কোডের অন্য
উদাহরণগুলোতে এটি চালিয়ে দেখুন কী বলে!

Miri সম্পর্কে আরও জানতে পারেন [এর GitHub repository-তে][miri]।

<!-- Old headings. Do not remove or links may break. -->

<a id="when-to-use-unsafe-code"></a>

### unsafe কোড সঠিকভাবে ব্যবহার করা {#using-unsafe-code-correctly}

এইমাত্র আলোচিত পাঁচটি অতিমানবীয় ক্ষমতার একটি ব্যবহার করতে `unsafe` ব্যবহার করা ভুল নয়, এমনকি অপছন্দনীয়ও নয়, তবে `unsafe` কোড সঠিক করা বেশি কঠিন, কারণ compiler memory safety বজায় রাখতে সাহায্য
করতে পারে না। `unsafe` কোড ব্যবহারের কারণ থাকলে তা করতে পারেন, আর স্পষ্ট `unsafe` annotation থাকায় সমস্যা হলে তার উৎস খুঁজে বের করা সহজ হয়। যখনই unsafe কোড লিখবেন, আপনার লেখা কোড Rust-এর নিয়ম
মানে কি না সে বিষয়ে আরও আস্থা পেতে Miri ব্যবহার করতে পারেন।

unsafe Rust নিয়ে কার্যকরভাবে কাজ করার আরও অনেক গভীর আলোচনার জন্য `unsafe`-এর জন্য Rust-এর দাপ্তরিক নির্দেশিকা [The Rustonomicon][nomicon] পড়ুন।

{{#quiz ../quizzes/ch19-01-unsafe-rust.toml}}

[permission-violations]: ch04-02-references-and-borrowing.html#the-borrow-checker-finds-permission-violations
[ABI]: https://doc.rust-lang.org/reference/items/external-blocks.html#abi
[the-slice-type]: ch04-04-slices.html#the-slice-type
[constants]: ch03-01-variables-and-mutability.html#declaring-constants
[send-and-sync]: ch16-04-extensible-concurrency-sync-and-send.html
[the-slice-type]: ch04-03-slices.html#the-slice-type
[unions]: https://doc.rust-lang.org/reference/items/unions.html
[miri]: https://github.com/rust-lang/miri
[editions]: appendix-05-editions.html
[nightly]: appendix-07-nightly-rust.html
[nomicon]: https://doc.rust-lang.org/nomicon/
