## `Drop` trait দিয়ে পরিষ্কারের সময় কোড চালানো {#running-code-on-cleanup-with-the-drop-trait}

smart pointer pattern-এর জন্য দ্বিতীয় গুরুত্বপূর্ণ trait হলো `Drop`, যা দিয়ে আপনি ঠিক করে দিতে পারেন একটি মান scope-এর বাইরে যাওয়ার মুহূর্তে কী ঘটবে। যেকোনো
type-এর ওপর `Drop` trait-এর implementation দিতে পারেন, আর সেই কোড দিয়ে ফাইল বা network connection-এর মতো resource (সম্পদ) ছেড়ে দেওয়া যায়।

smart pointer-এর প্রেক্ষাপটে আমরা `Drop`-এর সঙ্গে পরিচয় করাচ্ছি, কারণ smart pointer implement করার সময় প্রায় সবসময়ই `Drop` trait-এর কার্যক্ষমতা ব্যবহার হয়।
যেমন, একটি `Box<T>` drop হলে box যে heap-এর জায়গাকে নির্দেশ করে, তা deallocate হয়ে যাবে।

কিছু ভাষায় কিছু type-এর ক্ষেত্রে প্রোগ্রামারকে প্রতিবার সেই type-এর একটি instance ব্যবহার শেষ করার পর মেমোরি বা resource মুক্ত করার কোড call করতে হয়। উদাহরণের
মধ্যে আছে file handle, socket আর lock। প্রোগ্রামার ভুলে গেলে system অতিরিক্ত চাপে পড়ে crash করতে পারে। Rust-এ আপনি বলে দিতে পারেন যে একটি মান scope-এর বাইরে
গেলেই নির্দিষ্ট কিছু কোড চলবে, আর compiler এই কোড স্বয়ংক্রিয়ভাবে বসিয়ে দেবে। ফলে প্রোগ্রামের যেখানেই নির্দিষ্ট একটি type-এর instance-এর কাজ শেষ হয়, সেখানে
সতর্ক হয়ে পরিষ্কারের কোড বসাতে হয় না—তবুও কোনো resource leak হবে না!

একটি মান scope-এর বাইরে গেলে কোন কোড চলবে তা আপনি `Drop` trait implement করে ঠিক করে দেন। `Drop` trait আপনাকে `drop` নামের একটি method implement করতে বলে, যা
`self`-এর একটি mutable reference নেয়। Rust কখন `drop` call করে তা দেখতে আপাতত `println!` statement দিয়ে `drop` implement করি।

Listing 15-14-এ একটি `CustomSmartPointer` struct দেখানো হয়েছে, যার একমাত্র নিজস্ব কার্যক্ষমতা হলো instance scope-এর বাইরে গেলে এটি print করবে `Dropping CustomSmartPointer!`,
যাতে Rust কখন `drop` method চালায় তা দেখা যায়।

<Listing number="15-14" file-name="src/main.rs" caption="`Drop` trait implement করা একটি `CustomSmartPointer` struct, যেখানে আমরা পরিষ্কারের কোড রাখতাম">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-14/src/main.rs}}
```

</Listing>

`Drop` trait prelude-এ আছে, তাই একে scope-এ আনার দরকার নেই। আমরা `CustomSmartPointer`-এর ওপর `Drop` trait implement করি এবং `drop` method-এর এমন একটি
implementation দিই যা `println!` call করে। আপনার type-এর একটি instance scope-এর বাইরে গেলে যে logic চালাতে চান, তা রাখবেন `drop` method-এর body-তে। Rust কখন
`drop` call করবে তা চোখে দেখাতে এখানে আমরা কিছু লেখা print করছি।

`main`-এ আমরা `CustomSmartPointer`-এর দুটি instance তৈরি করি, তারপর print করি `CustomSmartPointers created`। `main`-এর শেষে আমাদের `CustomSmartPointer`
instance-গুলো scope-এর বাইরে যাবে, আর Rust `drop` method-এ রাখা আমাদের কোড call করবে, যা আমাদের শেষ message print করবে। খেয়াল করুন, `drop` method স্পষ্টভাবে call
করতে হয়নি।

এই প্রোগ্রাম চালালে এই output দেখব:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-14/output.txt}}
```

আমাদের instance-গুলো scope-এর বাইরে যাওয়ার সময় Rust স্বয়ংক্রিয়ভাবে আমাদের জন্য `drop` call করেছে, আমাদের দেওয়া কোড চালিয়েছে। variable-গুলো তৈরির উল্টো ক্রমে
drop হয়, তাই `d`, `c`-এর আগে drop হয়েছে। এই উদাহরণের উদ্দেশ্য `drop` method কীভাবে কাজ করে তার একটি দৃশ্যমান ধারণা দেওয়া; সাধারণত আপনি print message-এর বদলে
আপনার type-এর দরকারি পরিষ্কারের কোড লিখবেন।

<!-- Old headings. Do not remove or links may break. -->

<a id="dropping-a-value-early-with-std-mem-drop"></a>

দুর্ভাগ্যবশত, স্বয়ংক্রিয় `drop` কার্যক্ষমতা বন্ধ করা সোজা নয়। `drop` বন্ধ করার সাধারণত দরকারও হয় না; `Drop` trait-এর মূল কথাই হলো এটি স্বয়ংক্রিয়ভাবে সামলানো
হয়। তবে মাঝে মাঝে হয়তো কোনো মান আগেভাগে পরিষ্কার করতে চাইবেন। একটি উদাহরণ হলো lock সামলানো smart pointer ব্যবহারের সময়: হয়তো lock ছেড়ে দেওয়া `drop` method
জোর করে চালাতে চাইবেন, যাতে একই scope-এর অন্য কোড lock নিতে পারে। Rust আপনাকে `Drop` trait-এর `drop` method হাতে call করতে দেয় না; বরং scope শেষ হওয়ার আগে
একটি মান জোর করে drop করাতে চাইলে standard library-র দেওয়া `std::mem::drop` function call করতে হবে।

Listing 15-14-এর `main` function বদলে `Drop` trait-এর `drop` method হাতে call করার চেষ্টা কাজ করবে না, যেমনটি Listing 15-15-এ দেখানো হয়েছে।

<Listing number="15-15" file-name="src/main.rs" caption="আগেভাগে পরিষ্কার করতে `Drop` trait-এর `drop` method হাতে call করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-15/src/main.rs:here}}
```

</Listing>

এই কোড compile করার চেষ্টা করলে এই error পাব:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-15/output.txt}}
```

এই error message বলছে যে আমাদের স্পষ্টভাবে `drop` call করার অনুমতি নেই। error message-এ _destructor_ পরিভাষাটি ব্যবহার করা হয়েছে, যা একটি instance পরিষ্কার করে
এমন function-এর সাধারণ প্রোগ্রামিং পরিভাষা। _destructor_ হলো _constructor_-এর বিপরীত রূপ, যা একটি instance তৈরি করে। Rust-এর `drop` function একটি নির্দিষ্ট
destructor।

Rust আমাদের স্পষ্টভাবে `drop` call করতে দেয় না, কারণ Rust তবুও `main`-এর শেষে মানটির ওপর স্বয়ংক্রিয়ভাবে `drop` call করত। এতে double free (একই মেমোরি দুবার মুক্ত
করা) error হতো, কারণ Rust একই মান দুবার পরিষ্কার করার চেষ্টা করত।

একটি মান scope-এর বাইরে গেলে `drop`-এর স্বয়ংক্রিয় সংযোজন আমরা বন্ধ করতে পারি না, আবার `drop` method স্পষ্টভাবে call-ও করতে পারি না। তাই কোনো মান জোর করে
আগেভাগে পরিষ্কার করতে হলে আমরা `std::mem::drop` function ব্যবহার করি।

`std::mem::drop` function `Drop` trait-এর `drop` method থেকে আলাদা। যে মান জোর করে drop করাতে চাই সেটি argument হিসেবে পাঠিয়ে আমরা এটি call করি। function-টি
prelude-এ আছে, তাই Listing 15-15-এর `main` বদলে `drop` function call করাতে পারি, যেমনটি Listing 15-16-এ দেখানো হয়েছে।

<Listing number="15-16" file-name="src/main.rs" caption="scope-এর বাইরে যাওয়ার আগে একটি মান স্পষ্টভাবে drop করতে `std::mem::drop` call করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-16/src/main.rs:here}}
```

</Listing>

এই কোড চালালে এই output আসবে:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-16/output.txt}}
```

``Dropping CustomSmartPointer with data `some data`!`` লেখাটি print হয় `CustomSmartPointer created` আর `CustomSmartPointer dropped before
the end of main` লেখার মাঝখানে, যা দেখায় যে ওই মুহূর্তে `c` drop করতে `drop` method-এর কোড call হয়েছে।

পরিষ্কারের কাজ সুবিধাজনক ও নিরাপদ করতে `Drop` trait implementation-এ দেওয়া কোড নানাভাবে ব্যবহার করতে পারেন: যেমন, এটি দিয়ে নিজস্ব memory allocator (মেমোরি
বরাদ্দকারী) বানাতে পারেন! `Drop` trait আর Rust-এর ownership system থাকায় পরিষ্কার করার কথা আপনাকে মনে রাখতে হয় না, কারণ Rust এটি স্বয়ংক্রিয়ভাবে করে।

এখনো ব্যবহৃত হচ্ছে এমন মান ভুল করে পরিষ্কার করে ফেলার ফলে তৈরি সমস্যা নিয়েও চিন্তা করতে হয় না: যে ownership system নিশ্চিত করে reference সবসময় বৈধ, সেটিই নিশ্চিত
করে যে মানটি আর ব্যবহার না হলে `drop` মাত্র একবার call হয়।

`Box<T>` আর smart pointer-এর কিছু বৈশিষ্ট্য দেখার পর এবার standard library-তে সংজ্ঞায়িত আরও কয়েকটি smart pointer দেখা যাক।

{{#quiz ../quizzes/ch15-03-drop.toml}}
