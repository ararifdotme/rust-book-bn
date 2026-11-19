## উন্নত type {#advanced-types}

Rust-এর type system-এর এমন কিছু বৈশিষ্ট্য আছে, যা আমরা এ পর্যন্ত উল্লেখ করেছি কিন্তু আলোচনা করিনি। type হিসেবে newtype কেন কাজের তা দেখার সময় আমরা সাধারণভাবে newtype নিয়ে আলোচনা দিয়ে শুরু করব। তারপর
type alias-এ যাব, যা newtype-এর মতো একটি বৈশিষ্ট্য, তবে অর্থে একটু আলাদা। `!` type আর dynamically sized type নিয়েও আলোচনা করব।

<!-- Old headings. Do not remove or links may break. -->

<a id="using-the-newtype-pattern-for-type-safety-and-abstraction"></a>

### newtype pattern দিয়ে type safety ও abstraction {#type-safety-and-abstraction-with-the-newtype-pattern}

এই অংশ ধরে নিচ্ছে আপনি আগের [“newtype pattern দিয়ে বাইরের trait implement
করা”][newtype]<!-- ignore --> অংশটি পড়েছেন। এ পর্যন্ত আলোচিত কাজের বাইরেও newtype pattern কাজে লাগে, যেমন স্থিরভাবে নিশ্চিত করা যে মানগুলো কখনো গুলিয়ে যাবে না, আর একটি মানের একক বোঝানো। একক বোঝাতে
newtype ব্যবহারের একটি উদাহরণ আপনি Listing
20-16-এ দেখেছেন: মনে করুন, `Millimeters` আর `Meters` struct `u32` মানকে একটি newtype-এ মুড়েছিল। `Millimeters` type-এর parameter-যুক্ত একটি function লিখলে, যে প্রোগ্রাম ভুল করে `Meters` type-এর
বা সাধারণ `u32`-এর একটি মান দিয়ে সেই function call করার চেষ্টা করে, তা compile করা যেত না।

একটি type-এর implementation-এর কিছু খুঁটিনাটি আড়ালে রাখতেও newtype pattern ব্যবহার করা যায়: নতুন type এমন একটি public API প্রকাশ করতে পারে, যা private ভেতরের type-এর API থেকে আলাদা।

newtype অভ্যন্তরীণ implementation-ও লুকিয়ে রাখতে পারে। যেমন, আমরা একটি `HashMap<i32, String>` মুড়ে রাখার জন্য একটি `People` type দিতে পারি, যা একজন ব্যক্তির নামের সঙ্গে যুক্ত তার ID রাখে। `People`
ব্যবহার করা কোড শুধু আমাদের দেওয়া public API-র সঙ্গে কাজ করবে, যেমন `People` collection-এ একটি নামের string যোগ করার একটি method; সেই কোডের জানার দরকার নেই যে আমরা ভেতরে ভেতরে নামগুলোকে একটি
`i32` ID দিই। implementation-এর খুঁটিনাটি লুকিয়ে রাখতে encapsulation অর্জনের একটি হালকা উপায় হলো newtype pattern, যা নিয়ে অধ্যায় 18-এর [“implementation-এর খুঁটিনাটি লুকিয়ে রাখা
encapsulation”][encapsulation-that-hides-implementation-details]<!-- ignore -->
অংশে আলোচনা করেছি।

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-type-synonyms-with-type-aliases"></a>

### type synonym ও type alias {#type-synonyms-and-type-aliases}

একটি বিদ্যমান type-কে আরেকটি নাম দিতে Rust একটি _type alias_ ঘোষণার ক্ষমতা দেয়। এজন্য আমরা `type` keyword ব্যবহার করি। যেমন, `i32`-এর জন্য `Kilometers` alias এভাবে তৈরি করা যায়:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:here}}
```

এখন `Kilometers` alias হলো `i32`-এর একটি _synonym_ (সমার্থক নাম); Listing 20-16-এ তৈরি `Millimeters` আর `Meters` type-এর মতো নয়, `Kilometers` আলাদা কোনো নতুন type নয়। `Kilometers` type-এর
মানগুলোকে `i32` type-এর মানের মতোই গণ্য করা হবে:

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-04-kilometers-alias/src/main.rs:there}}
```

`Kilometers` আর `i32` যেহেতু একই type, তাই দুই type-এর মান যোগ করা যায় এবং `i32` parameter নেয় এমন function-এ `Kilometers` মান পাঠানো যায়। তবে এই পদ্ধতিতে আগে আলোচিত newtype pattern থেকে পাওয়া
type-checking-এর সুবিধা পাই না। অন্যভাবে বললে, কোথাও `Kilometers` আর `i32` মান গুলিয়ে ফেললে compiler আমাদের কোনো error দেবে না।

type synonym-এর প্রধান ব্যবহার হলো পুনরাবৃত্তি কমানো। যেমন, আমাদের এমন একটি লম্বা type থাকতে পারে:

```rust,ignore
Box<dyn Fn() + Send + 'static>
```

কোড জুড়ে function signature-এ আর type annotation হিসেবে এই লম্বা type লেখা ক্লান্তিকর এবং ভুলপ্রবণ হতে পারে। ভাবুন Listing 20-25-এর মতো কোডে ভরা একটি project-এর কথা।

<Listing number="20-25" caption="অনেক জায়গায় একটি লম্বা type ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-25/src/main.rs:here}}
```

</Listing>

একটি type alias পুনরাবৃত্তি কমিয়ে এই কোড আরও সামলানো সহজ করে। Listing 20-26-এ আমরা শব্দবহুল type-টির জন্য `Thunk` নামের একটি alias এনেছি এবং type-টির সব ব্যবহার ছোট alias `Thunk` দিয়ে বদলে দিতে
পারি।

<Listing number="20-26" caption="পুনরাবৃত্তি কমাতে একটি type alias, `Thunk`, আনা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-26/src/main.rs:here}}
```

</Listing>

এই কোড পড়া ও লেখা অনেক সহজ! একটি type alias-এর জন্য অর্থবহ নাম বেছে নিলে আপনার উদ্দেশ্যও বোঝাতে সুবিধা হয় (_thunk_ হলো পরে মূল্যায়ন করা হবে এমন কোডের একটি নাম, তাই সংরক্ষণ করে রাখা একটি
closure-এর জন্য এটি উপযুক্ত নাম)।

পুনরাবৃত্তি কমাতে `Result<T, E>` type-এর সঙ্গেও type alias প্রচলিতভাবে ব্যবহার হয়। standard library-র `std::io` module-এর কথা ভাবুন। operation কাজ না করার পরিস্থিতি সামলাতে I/O operation প্রায়ই
একটি `Result<T, E>` return করে। এই library-তে একটি `std::io::Error` struct আছে, যা সব সম্ভাব্য I/O error বোঝায়। `std::io`-এর অনেক function `Result<T, E>` return করবে, যেখানে `E` হলো
`std::io::Error`, যেমন `Write` trait-এর এই function-গুলো:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-05-write-trait/src/lib.rs}}
```

`Result<..., Error>` অনেকবার পুনরাবৃত্ত হয়। তাই `std::io`-এ এই type alias ঘোষণা আছে:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:here}}
```

এই ঘোষণা যেহেতু `std::io` module-এ আছে, তাই আমরা পূর্ণ পরিচয়সহ alias `std::io::Result<T>` ব্যবহার করতে পারি; অর্থাৎ একটি `Result<T, E>`, যেখানে `E`-এর জায়গায় বসানো
`std::io::Error`। `Write` trait-এর function signature-গুলো শেষ পর্যন্ত দেখতে এমন হয়:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-06-result-alias/src/lib.rs:there}}
```

type alias দুইভাবে সাহায্য করে: এটি কোড লেখা সহজ করে, _এবং_ পুরো `std::io` জুড়ে আমাদের একটি সামঞ্জস্যপূর্ণ interface দেয়। এটি যেহেতু একটি alias, তাই এটি আরেকটি `Result<T, E>` মাত্র, অর্থাৎ
`Result<T, E>`-এর ওপর কাজ করে এমন যেকোনো method এর সঙ্গে ব্যবহার করা যায়, `?` operator-এর মতো বিশেষ syntax-ও।

### কখনো return করে না এমন never type {#the-never-type-that-never-returns}

Rust-এ `!` নামের একটি বিশেষ type আছে, যা type তত্ত্বের পরিভাষায় _empty type_ (খালি type) নামে পরিচিত, কারণ এর কোনো মান নেই। আমরা একে _never type_ বলতে পছন্দ করি, কারণ একটি function কখনো return না
করলে এটি return type-এর জায়গায় বসে। একটি উদাহরণ:

```rust,noplayground
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-07-never-type/src/lib.rs:here}}
```

এই কোড পড়া হয় এভাবে: “`bar` function কখনো return করে না।” যে function কখনো return করে না তাদের বলা হয় _diverging functions_ (diverging function)। `!` type-এর মান তৈরি করা যায় না, তাই `bar`
কখনো return করতে পারে না।

কিন্তু যে type-এর কখনো মান তৈরি করা যায় না, তার কী কাজ? সংখ্যা আন্দাজের খেলার অংশ, Listing 2-5-এর কোডটি মনে করুন; এর কিছু অংশ এখানে Listing 20-27-এ আবার দেওয়া হলো।

<Listing number="20-27" caption="`continue` দিয়ে শেষ হওয়া একটি arm-যুক্ত `match`">

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:ch19}}
```

</Listing>

তখন আমরা এই কোডের কিছু খুঁটিনাটি এড়িয়ে গিয়েছিলাম। অধ্যায় 6-এর [“`match` control flow
গঠন”][the-match-control-flow-construct]<!-- ignore --> অংশে আলোচনা করেছিলাম যে `match`-এর সব arm-কে একই type return করতে হয়। তাই যেমন, নিচের কোড কাজ করে না:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-08-match-arms-different-types/src/main.rs:here}}
```

এই কোডে `guess`-এর type-কে একটি integer _এবং_ একটি string হতে হতো, আর Rust চায় `guess`-এর মাত্র একটি type থাকুক। তাহলে `continue` কী return করে? Listing 20-27-এ কীভাবে একটি arm থেকে `u32`
return করে আরেকটি arm `continue` দিয়ে শেষ করার অনুমতি পেয়েছিলাম?

হয়তো আন্দাজ করেছেন, `continue`-এর মান `!`। অর্থাৎ Rust যখন `guess`-এর type হিসাব করে, তখন দুটি match arm-ই দেখে: প্রথমটির মান `u32`, আর দ্বিতীয়টির মান `!`। `!`-এর যেহেতু কখনো কোনো মান থাকতে পারে না,
তাই Rust ঠিক করে যে `guess`-এর type হলো `u32`।

এই আচরণ বর্ণনার আনুষ্ঠানিক উপায় হলো, `!` type-এর expression-কে অন্য যেকোনো type-এ রূপান্তর (coerce) করা যায়। এই `match` arm `continue` দিয়ে শেষ করার অনুমতি আছে, কারণ `continue` কোনো মান return
করে না; বরং এটি নিয়ন্ত্রণ loop-এর শুরুতে ফিরিয়ে নেয়, তাই `Err`-এর ক্ষেত্রে আমরা কখনো `guess`-এ মান বসাই না।

never type `panic!` macro-র সঙ্গেও কাজে লাগে। `Option<T>` মানের ওপর একটি মান পেতে বা panic করতে যে `unwrap` function call করি, তার এই সংজ্ঞাটি মনে করুন:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-09-unwrap-definition/src/lib.rs:here}}
```

এই কোডে Listing 20-27-এর `match`-এর মতোই ঘটে: Rust দেখে যে `val`-এর type `T` আর `panic!`-এর type `!`, তাই পুরো `match` expression-এর ফল হলো `T`। এই কোড কাজ করে, কারণ `panic!` কোনো মান তৈরি
করে না; এটি প্রোগ্রাম শেষ করে দেয়। `None`-এর ক্ষেত্রে আমরা `unwrap` থেকে কোনো মান return করব না, তাই এই কোড বৈধ।

`!` type-এর শেষ একটি expression হলো loop:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-10-loop-returns-never/src/main.rs:here}}
```

এখানে loop কখনো শেষ হয় না, তাই expression-এর মান হলো `!`। তবে একটি `break` রাখলে এটি সত্য হতো না, কারণ `break`-এ পৌঁছালে loop শেষ হয়ে যেত।

### dynamically sized type ও `Sized` trait {#dynamically-sized-types-and-the-sized-trait}

Rust-এর তার type সম্পর্কে কিছু নির্দিষ্ট খুঁটিনাটি জানা দরকার, যেমন একটি নির্দিষ্ট type-এর মানের জন্য কতটা জায়গা allocate করতে হবে। এতে এর type system-এর একটি কোণ শুরুতে একটু বিভ্রান্তিকর লাগে: _dynamically
sized types_ (চলার সময় আকার জানা যায় এমন type)-এর ধারণা। কখনো কখনো _DSTs_ বা _unsized types_ নামে পরিচিত এই type-গুলো এমন মান ব্যবহার করে কোড লিখতে দেয়, যাদের আকার শুধু runtime-এ জানা যায়।

চলুন `str` নামের একটি dynamically sized type-এর খুঁটিনাটি দেখি, যা আমরা বই জুড়ে ব্যবহার করে আসছি। ঠিক ধরেছেন, `&str` নয়, বরং আলাদাভাবে `str` একটি DST। অনেক ক্ষেত্রে, যেমন ব্যবহারকারীর লেখা টেক্সট
রাখার সময়, string কত লম্বা তা runtime-এর আগে জানা যায় না। এর মানে `str` type-এর variable তৈরি করা যায় না, `str` type-এর argument-ও নেওয়া যায় না। নিচের কোডটি দেখুন, যা কাজ করে না:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-11-cant-create-str/src/main.rs:here}}
```

একটি নির্দিষ্ট type-এর যেকোনো মানের জন্য কতটা মেমোরি allocate করতে হবে তা Rust-এর জানা দরকার, আর একটি type-এর সব মানকে একই পরিমাণ মেমোরি ব্যবহার করতে হয়। Rust যদি আমাদের এই কোড লিখতে দিত,
তাহলে এই দুটি `str` মানকে একই পরিমাণ জায়গা নিতে হতো। কিন্তু এদের দৈর্ঘ্য আলাদা: `s1`-এর 12 byte জায়গা লাগে আর `s2`-এর 15। এ কারণেই dynamically sized type রাখা variable তৈরি করা সম্ভব নয়।

তাহলে কী করব? এই ক্ষেত্রে উত্তর আপনি আগেই জানেন: `s1` আর `s2`-এর type `str`-এর বদলে string slice (`&str`) বানাই। অধ্যায় 4-এর [“string slice”][string-slices]<!-- ignore --> অংশ থেকে মনে
করুন, slice data structure শুধু slice-এর শুরুর অবস্থান আর দৈর্ঘ্য রাখে। তাই `&T` যেখানে একটিমাত্র মান, যা `T` কোথায় আছে তার মেমোরি ঠিকানা রাখে, একটি string slice সেখানে _দুটি_ মান: `str`-এর ঠিকানা
আর এর দৈর্ঘ্য। ফলে compile time-এ একটি string slice মানের আকার জানা যায়: এটি একটি `usize`-এর দৈর্ঘ্যের দ্বিগুণ। অর্থাৎ string slice যে string-কে নির্দেশ করে তা যত লম্বাই হোক, আমরা সবসময় এর
আকার জানি। সাধারণভাবে Rust-এ dynamically sized type এভাবেই ব্যবহার হয়: এদের একটু বাড়তি মেটাডেটা থাকে, যা dynamic তথ্যের আকার রাখে। dynamically sized type-এর সোনালি নিয়ম হলো, dynamically sized
type-এর মান সবসময় কোনো না কোনো pointer-এর পেছনে রাখতে হবে।

`str`-কে সব ধরনের pointer-এর সঙ্গে মেলানো যায়: যেমন `Box<str>` বা `Rc<str>`। আসলে আপনি এটা আগেও দেখেছেন, তবে ভিন্ন একটি dynamically sized type-এর সঙ্গে: trait। প্রতিটি trait একটি dynamically
sized type, যাকে trait-এর নাম ব্যবহার করে নির্দেশ করা যায়। অধ্যায় 18-এর [“সাধারণ আচরণের ওপর abstraction করতে trait object
ব্যবহার”][using-trait-objects-to-abstract-over-shared-behavior]<!--
ignore --> অংশে আমরা বলেছিলাম যে trait-কে trait object হিসেবে ব্যবহার করতে হলে এদের একটি pointer-এর পেছনে রাখতে হবে, যেমন `&dyn Trait` বা `Box<dyn
Trait>` (`Rc<dyn Trait>`-ও কাজ করত)।

DST নিয়ে কাজ করতে Rust `Sized` trait দেয়, যা দিয়ে নির্ধারণ করা যায় একটি type-এর আকার compile time-এ জানা কি না। compile time-এ আকার জানা সবকিছুর জন্য এই trait স্বয়ংক্রিয়ভাবে implement হয়। তা ছাড়া
Rust প্রতিটি generic function-এ অপ্রকাশ্যভাবে `Sized`-এর একটি bound যোগ করে। অর্থাৎ এমন একটি generic function-এর সংজ্ঞা:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-12-generic-fn-definition/src/lib.rs}}
```

আসলে এমনভাবে গণ্য হয় যেন আমরা এটা লিখেছি:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-13-generic-implicit-sized-bound/src/lib.rs}}
```

default-ভাবে generic function শুধু সেই type-এর ওপর কাজ করবে, যাদের আকার compile time-এ জানা। তবে এই সীমাবদ্ধতা শিথিল করতে নিচের বিশেষ syntax ব্যবহার করতে পারেন:

```rust,ignore
{{#rustdoc_include ../listings/ch20-advanced-features/no-listing-14-generic-maybe-sized/src/lib.rs}}
```

`?Sized`-এর ওপর একটি trait bound মানে “`T` `Sized` হতেও পারে, না-ও পারে,” আর এই চিহ্ন generic type-এর আকার compile time-এ জানা থাকতে হবে, এই default-কে override করে। এই অর্থে `?Trait` syntax শুধু
`Sized`-এর জন্য পাওয়া যায়, অন্য কোনো trait-এর জন্য নয়।

আরও খেয়াল করুন, আমরা `t` parameter-এর type `T` থেকে বদলে `&T` করেছি। type-টি `Sized` না-ও হতে পারে বলে একে কোনো না কোনো pointer-এর পেছনে ব্যবহার করতে হবে। এই ক্ষেত্রে আমরা একটি reference বেছে নিয়েছি।

এরপর আমরা function আর closure নিয়ে কথা বলব!

{{#quiz ../quizzes/ch19-04-advanced-types.toml}}

[encapsulation-that-hides-implementation-details]: ch18-01-what-is-oo.html#encapsulation-that-hides-implementation-details
[string-slices]: ch04-04-slices.html#string-slices
[the-match-control-flow-construct]: ch06-02-match.html#the-match-control-flow-construct
[using-trait-objects-to-abstract-over-shared-behavior]: ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior
[newtype]: ch20-02-advanced-traits.html#implementing-external-traits-with-the-newtype-pattern
