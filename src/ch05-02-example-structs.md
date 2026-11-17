## struct ব্যবহার করা একটি উদাহরণ প্রোগ্রাম {#an-example-program-using-structs}

কখন আমরা struct ব্যবহার করতে চাইতে পারি, তা বুঝতে চলুন একটি প্রোগ্রাম লিখি, যা একটি
আয়তক্ষেত্রের ক্ষেত্রফল হিসাব করে। আমরা শুরু করব আলাদা আলাদা variable দিয়ে, তারপর
প্রোগ্রামটি refactor (আচরণ না বদলে কোড পুনর্গঠন) করতে করতে struct ব্যবহারে পৌঁছাব।

চলুন Cargo দিয়ে _rectangles_ নামে একটি নতুন binary প্রজেক্ট বানাই, যা পিক্সেলে দেওয়া
একটি আয়তক্ষেত্রের প্রস্থ আর উচ্চতা নিয়ে আয়তক্ষেত্রটির ক্ষেত্রফল হিসাব করবে। Listing
5-8-এ একটি ছোট প্রোগ্রাম দেখানো হয়েছে, যা আমাদের প্রজেক্টের _src/main.rs_-এ ঠিক এই
কাজটি করার একটি উপায়।

<Listing number="5-8" file-name="src/main.rs" caption="আলাদা প্রস্থ আর উচ্চতা variable দিয়ে দেওয়া একটি আয়তক্ষেত্রের ক্ষেত্রফল হিসাব করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

</Listing>

এবার `cargo run` দিয়ে প্রোগ্রামটি চালান:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

প্রতিটি মাপ দিয়ে `area` function call করে এই কোড আয়তক্ষেত্রের ক্ষেত্রফল সফলভাবে বের
করে ফেলে, তবে কোডটিকে আরও স্পষ্ট ও পাঠযোগ্য করতে আমরা আরও কিছু করতে পারি।

এই কোডের সমস্যা `area`-এর signature (function-এর নাম, parameter ও return type-এর
ঘোষণা) দেখলেই স্পষ্ট:

```rust,ignore
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:here}}
```

`area` function-এর কাজ একটি আয়তক্ষেত্রের ক্ষেত্রফল হিসাব করা, কিন্তু আমাদের লেখা
function-টির দুটি parameter আছে, আর প্রোগ্রামের কোথাও স্পষ্ট নয় যে parameter দুটি
পরস্পর সম্পর্কিত। প্রস্থ আর উচ্চতা একসঙ্গে রাখলে কোডটি আরও পাঠযোগ্য আর সামলানো সহজ
হতো। এটি করার একটি উপায় আমরা ইতিমধ্যে অধ্যায় 3-এর [“tuple type”][the-tuple-type]<!-- ignore --> অংশে
আলোচনা করেছি: tuple ব্যবহার করে।

### tuple দিয়ে refactor করা {#refactoring-with-tuples}

Listing 5-9-এ আমাদের প্রোগ্রামের আরেকটি সংস্করণ দেখানো হয়েছে, যা tuple ব্যবহার করে।

<Listing number="5-9" file-name="src/main.rs" caption="একটি tuple দিয়ে আয়তক্ষেত্রের প্রস্থ আর উচ্চতা নির্দিষ্ট করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

</Listing>

এক দিক থেকে এই প্রোগ্রামটি ভালো। tuple দিয়ে আমরা কিছুটা কাঠামো যোগ করতে পারি, আর এখন
মাত্র একটি argument পাঠাচ্ছি। কিন্তু আরেক দিক থেকে এই সংস্করণটি কম স্পষ্ট: tuple তার
উপাদানগুলোর নাম দেয় না, তাই আমাদের tuple-এর অংশগুলোতে index দিয়ে পৌঁছাতে হয়, ফলে
আমাদের হিসাবটি কম সুস্পষ্ট হয়।

ক্ষেত্রফলের হিসাবে প্রস্থ আর উচ্চতা গুলিয়ে ফেললে কিছু যায় আসে না, কিন্তু আয়তক্ষেত্রটি
স্ক্রিনে আঁকতে চাইলে তা গুরুত্বপূর্ণ হতো! আমাদের মনে রাখতে হতো যে `width` হলো tuple-এর
index `0` আর `height` হলো tuple-এর index `1`। অন্য কেউ আমাদের কোড ব্যবহার করলে তার পক্ষে
এটি বুঝে বের করা আর মনে রাখা আরও কঠিন হতো। আমাদের কোডে ডেটার অর্থ প্রকাশ করিনি বলে
এখন ভুল ঢুকে পড়া আরও সহজ।

<!-- Old headings. Do not remove or links may break. -->

<a id="refactoring-with-structs-adding-more-meaning"></a>

### struct দিয়ে refactor করা {#refactoring-with-structs}

ডেটায় লেবেল দিয়ে অর্থ যোগ করতে আমরা struct ব্যবহার করি। যে tuple ব্যবহার করছি, সেটিকে
আমরা একটি struct-এ রূপান্তর করতে পারি, যাতে পুরোটার একটি নাম আর অংশগুলোরও নাম থাকে,
Listing 5-10-এ যেমন দেখানো হয়েছে।

<Listing number="5-10" file-name="src/main.rs" caption="একটি `Rectangle` struct সংজ্ঞায়িত করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

</Listing>

এখানে আমরা একটি struct সংজ্ঞায়িত করে তার নাম দিয়েছি `Rectangle`। কার্লি ব্র্যাকেটের
ভেতরে field হিসেবে সংজ্ঞায়িত করেছি `width` আর `height`, দুটোরই type `u32`। তারপর
`main`-এ `Rectangle`-এর একটি নির্দিষ্ট instance তৈরি করেছি, যার প্রস্থ `30` আর উচ্চতা
`50`।

আমাদের `area` function এখন একটি parameter দিয়ে সংজ্ঞায়িত, যার নাম আমরা দিয়েছি
`rectangle`, আর যার type হলো একটি `Rectangle` struct instance-এর একটি immutable borrow।
অধ্যায় 4-এ যেমন বলা হয়েছে, আমরা struct-টির ownership নেওয়ার বদলে সেটিকে borrow করতে
চাই। এভাবে `main` তার ownership ধরে রাখে এবং `rect1` ব্যবহার চালিয়ে যেতে পারে, আর
এ কারণেই আমরা function signature-এ এবং যেখানে function call করি, সেখানে `&` ব্যবহার করি।

`area` function `Rectangle` instance-এর `width` আর `height` field পড়ে (লক্ষ করুন, borrow
করা একটি struct instance-এর field পড়লে field-এর মানগুলো move হয় না, আর এ কারণেই প্রায়ই
struct-এর borrow দেখতে পাবেন)। `area`-এর function signature এখন ঠিক আমরা যা বোঝাতে চাই
তা-ই বলে: `Rectangle`-এর `width` আর `height` field ব্যবহার করে এর ক্ষেত্রফল হিসাব
করো। এটি বোঝায় যে প্রস্থ আর উচ্চতা পরস্পর সম্পর্কিত, আর tuple-এর index মান `0` আর `1`
ব্যবহারের বদলে মানগুলোকে বর্ণনামূলক নাম দেয়। স্পষ্টতার দিক থেকে এটি একটি জয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="adding-useful-functionality-with-derived-traits"></a>

### derive করা trait দিয়ে সুবিধা যোগ করা {#adding-functionality-with-derived-traits}

প্রোগ্রাম ডিবাগ করার সময় `Rectangle`-এর একটি instance প্রিন্ট করে তার সব field-এর মান
দেখতে পারলে কাজে লাগত। Listing 5-11-এ আগের অধ্যায়গুলোর মতো করে
[`println!` macro][println]<!-- ignore --> ব্যবহারের চেষ্টা করা হয়েছে। তবে এটি
কাজ করবে না।

<Listing number="5-11" file-name="src/main.rs" caption="একটি `Rectangle` instance প্রিন্ট করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

</Listing>

এই কোড compile করলে আমরা এই মূল বার্তাসহ একটি error পাই:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:3}}
```

`println!` macro (কোড তৈরি করে এমন কোড) অনেক ধরনের ফরম্যাটিং করতে পারে, আর ডিফল্টভাবে
কার্লি ব্র্যাকেট `println!`-কে `Display` নামের ফরম্যাটিং ব্যবহার করতে বলে: যে আউটপুট
সরাসরি শেষ ব্যবহারকারীর জন্য। এ পর্যন্ত দেখা primitive type (ভাষায় সরাসরি থাকা মৌলিক
type)-গুলো ডিফল্টভাবে `Display` implement করে, কারণ একজন ব্যবহারকারীকে একটি `1` বা অন্য
কোনো primitive type দেখানোর উপায় একটাই। কিন্তু struct-এর ক্ষেত্রে `println!`-এর আউটপুট
কীভাবে ফরম্যাট করা উচিত, তা কম স্পষ্ট, কারণ দেখানোর সম্ভাবনা অনেক: কমা দেবেন কি দেবেন
না? কার্লি ব্র্যাকেট প্রিন্ট করবেন? সব field দেখানো উচিত? এই অস্পষ্টতার কারণে Rust
আমরা কী চাই তা অনুমান করার চেষ্টা করে না, আর `println!` ও `{}` placeholder-এর সঙ্গে
ব্যবহারের জন্য struct-এর কোনো তৈরি `Display` implementation নেই।

error-গুলো পড়া চালিয়ে গেলে আমরা এই কাজের নোটটি পাব:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/output.txt:9:10}}
```

চলুন চেষ্টা করি! `println!` macro call এখন দেখতে হবে `println!("rect1 is
{rect1:?}");`-এর মতো। কার্লি ব্র্যাকেটের ভেতরে `:?` নির্দেশক বসালে `println!` বোঝে যে আমরা
`Debug` নামের একটি আউটপুট ফরম্যাট ব্যবহার করতে চাই। `Debug` trait আমাদের struct-টিকে
এমনভাবে প্রিন্ট করতে দেয়, যা ডেভেলপারদের কাজে লাগে, যাতে কোড ডিবাগ করার সময় এর মান
দেখতে পাই।

এই পরিবর্তনসহ কোডটি compile করুন। ধুর! এখনো error পাচ্ছি:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:3}}
```

তবে আবারও compiler আমাদের একটি কাজের নোট দেয়:

```text
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-01-debug/output.txt:9:10}}
```

ডিবাগিং-এর তথ্য প্রিন্ট করার সুবিধা Rust-এ _আছে_, কিন্তু আমাদের struct-এর জন্য সেই সুবিধা
চালু করতে আমাদের স্পষ্টভাবে বেছে নিতে হয়। এটি করতে struct-এর সংজ্ঞার ঠিক আগে outer
attribute (যে item-এর ঠিক আগে বসে, তার ওপর প্রযোজ্য attribute) `#[derive(Debug)]` যোগ
করি, Listing 5-12-এ যেমন দেখানো হয়েছে।

<Listing number="5-12" file-name="src/main.rs" caption="`Debug` trait derive করার attribute যোগ করা এবং debug ফরম্যাটিং দিয়ে `Rectangle` instance প্রিন্ট করা">

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

</Listing>

এবার প্রোগ্রামটি চালালে আমরা কোনো error পাব না, আর নিচের আউটপুট
দেখব:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

চমৎকার! আউটপুটটি সবচেয়ে সুন্দর নয়, তবে এটি এই instance-এর সব field-এর মান দেখায়, যা
ডিবাগিং-এর সময় নিশ্চয়ই কাজে লাগবে। বড় struct থাকলে আউটপুট একটু বেশি সহজপাঠ্য হওয়া
কাজের; সেসব ক্ষেত্রে `println!` string-এ `{:?}`-এর বদলে `{:#?}` ব্যবহার করতে পারি।
এই উদাহরণে `{:#?}` ধরন ব্যবহার করলে নিচের আউটপুট আসবে:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/output-only-02-pretty-debug/output.txt}}
```

`Debug` ফরম্যাট ব্যবহার করে কোনো মান প্রিন্ট করার আরেকটি উপায় হলো [`dbg!`
macro][dbg]<!-- ignore --> ব্যবহার করা, যা একটি expression-এর ownership নেয় (`println!`-এর
বিপরীতে, যা একটি reference নেয়), আপনার কোডে ওই `dbg!` macro call যে ফাইল ও লাইন নম্বরে
আছে, তা ওই expression-এর ফলাফলের মানসহ প্রিন্ট করে, এবং মানটির ownership ফেরত দেয়।

> Note: `dbg!` macro call করলে তা standard error কনসোল ধারায়
> (`stderr`) প্রিন্ট করে, `println!`-এর বিপরীতে, যা standard output কনসোল ধারায়
> (`stdout`) প্রিন্ট করে। `stderr` আর `stdout` নিয়ে আমরা অধ্যায় 12-এর
> [“Redirecting Errors to Standard Error” অংশে][err]<!-- ignore --> আরও
> কথা বলব।

এখানে একটি উদাহরণ, যেখানে আমরা `width` field-এ assign হওয়া মান আর `rect1`-এর পুরো
struct-এর মান—দুটোই জানতে আগ্রহী:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

`30 * scale` expression-এর চারপাশে আমরা `dbg!` বসাতে পারি, আর `dbg!` যেহেতু
expression-এর মানের ownership ফেরত দেয়, তাই `width` field এমন মানই পাবে, যেন সেখানে
`dbg!` call ছিলই না। আমরা চাই না `dbg!` `rect1`-এর ownership নিয়ে নিক, তাই পরের call-এ
`rect1`-এর একটি reference ব্যবহার করি। এই উদাহরণের আউটপুট দেখতে এরকম:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/output.txt}}
```

দেখতে পাচ্ছি, আউটপুটের প্রথম অংশটি এসেছে _src/main.rs_-এর লাইন 10 থেকে, যেখানে আমরা
`30 * scale` expression ডিবাগ করছি, আর এর ফলাফলের মান `60` (integer-এর জন্য implement
করা `Debug` ফরম্যাটিং শুধু তাদের মান প্রিন্ট করে)। _src/main.rs_-এর লাইন 14-এর `dbg!`
call `&rect1`-এর মান আউটপুট দেয়, যা হলো `Rectangle` struct। এই আউটপুট `Rectangle`
type-এর সুন্দর করে সাজানো `Debug` ফরম্যাটিং ব্যবহার করে। আপনার কোড কী করছে তা বোঝার
চেষ্টা করার সময় `dbg!` macro সত্যিই খুব কাজে আসতে পারে!

`Debug` trait ছাড়াও Rust আমাদের `derive` attribute-এর সঙ্গে ব্যবহারের জন্য বেশ কয়েকটি
trait দিয়েছে, যেগুলো আমাদের নিজস্ব type-এ কাজের আচরণ যোগ করতে পারে। সেই trait আর তাদের
আচরণগুলোর তালিকা আছে [পরিশিষ্ট C][app-c]<!--
ignore -->-তে। নিজস্ব আচরণসহ এই trait-গুলো কীভাবে implement করতে হয় এবং কীভাবে নিজের
trait তৈরি করতে হয়, তা আমরা অধ্যায় 10-এ দেখব। `derive` ছাড়াও আরও অনেক attribute আছে;
আরও জানতে দেখুন [Rust Reference-এর “Attributes”
অংশ][attributes]।

আমাদের `area` function খুবই নির্দিষ্ট: এটি শুধু আয়তক্ষেত্রের ক্ষেত্রফল হিসাব করে। এই
আচরণকে আমাদের `Rectangle` struct-এর সঙ্গে আরও ঘনিষ্ঠভাবে বেঁধে দিলে কাজে লাগত, কারণ এটি
অন্য কোনো type-এর সঙ্গে কাজ করবে না। চলুন দেখি কীভাবে `area` function-কে আমাদের
`Rectangle` type-এর ওপর সংজ্ঞায়িত একটি `area` method-এ পরিণত করে এই কোড refactor করা
চালিয়ে যাওয়া যায়।

{{#quiz ../quizzes/ch05-02-example-structs.toml}}

[the-tuple-type]: ch03-02-data-types.html#the-tuple-type
[app-c]: appendix-03-derivable-traits.md
[println]: https://doc.rust-lang.org/std/macro.println.html
[dbg]: https://doc.rust-lang.org/std/macro.dbg.html
[err]: ch12-06-writing-to-stderr-instead-of-stdout.html
[attributes]: https://doc.rust-lang.org/reference/attributes.html
