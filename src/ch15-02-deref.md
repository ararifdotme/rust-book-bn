<!-- Old headings. Do not remove or links may break. -->

<a id="treating-smart-pointers-like-regular-references-with-the-deref-trait"></a>
<a id="treating-smart-pointers-like-regular-references-with-deref"></a>

## smart pointer-কে সাধারণ reference-এর মতো ব্যবহার করা {#treating-smart-pointers-like-regular-references}

`Deref` trait implement করে আপনি _dereference operator_ (reference থেকে আসল মানে পৌঁছানোর operator) `*`-এর আচরণ নিজের মতো সাজাতে পারেন (গুণ বা glob
operator-এর সঙ্গে গুলিয়ে ফেলবেন না)। `Deref` এমনভাবে implement করলে যাতে একটি smart pointer-কে সাধারণ reference-এর মতো ব্যবহার করা যায়, আপনি reference-এর
ওপর কাজ করে এমন কোড লিখে সেই কোড smart pointer-এর সঙ্গেও ব্যবহার করতে পারেন।

প্রথমে দেখা যাক dereference operator সাধারণ reference-এর সঙ্গে কীভাবে কাজ করে। তারপর `Box<T>`-এর মতো আচরণ করে এমন একটি নিজস্ব type সংজ্ঞায়িত করার চেষ্টা করব
এবং দেখব কেন আমাদের নতুন সংজ্ঞায়িত type-এ dereference operator reference-এর মতো কাজ করে না। আমরা দেখব `Deref` trait implement করলে কীভাবে smart pointer
reference-এর মতো করে কাজ করতে পারে। তারপর Rust-এর deref coercion বৈশিষ্ট্য দেখব, এবং এটি কীভাবে আমাদের reference বা smart pointer, যেকোনোটির সঙ্গে কাজ
করতে দেয়।

<!-- Old headings. Do not remove or links may break. -->

<a id="following-the-pointer-to-the-value-with-the-dereference-operator"></a>
<a id="following-the-pointer-to-the-value"></a>

### reference ধরে মানে পৌঁছানো {#following-the-reference-to-the-value}

সাধারণ reference এক ধরনের pointer, আর pointer-কে ভাবার একটি উপায় হলো অন্য কোথাও রাখা একটি মানের দিকে একটি তীরচিহ্ন হিসেবে। Listing 15-6-এ আমরা একটি `i32`
মানের একটি reference তৈরি করি, তারপর dereference operator ব্যবহার করে reference ধরে মানে পৌঁছাই।

<Listing number="15-6" file-name="src/main.rs" caption="dereference operator ব্যবহার করে একটি `i32` মানের reference ধরে মানে পৌঁছানো">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

</Listing>

`x` variable একটি `i32` মান `5` রাখে। আমরা `y`-কে `x`-এর একটি reference-এর সমান করি। আমরা assert করতে পারি যে `x`, `5`-এর সমান। কিন্তু `y`-এর মান সম্পর্কে
assertion করতে চাইলে `*y` ব্যবহার করে reference ধরে এটি যে মানকে নির্দেশ করছে সেখানে পৌঁছাতে হবে (তাই নাম _dereference_), যাতে compiler আসল মানটি তুলনা
করতে পারে। `y`-কে dereference করলে `y` যে integer মানকে নির্দেশ করছে তা পাই, যা `5`-এর সঙ্গে তুলনা করা যায়।

এর বদলে `assert_eq!(5, y);` লেখার চেষ্টা করলে এই compilation error পেতাম:

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

একটি সংখ্যা আর একটি সংখ্যার reference তুলনা করা অনুমোদিত নয়, কারণ এরা ভিন্ন type। reference ধরে এটি যে মানকে নির্দেশ করছে সেখানে পৌঁছাতে আমাদের dereference
operator ব্যবহার করতেই হবে।

### `Box<T>`-কে reference-এর মতো ব্যবহার করা {#using-boxt-like-a-reference}

Listing 15-6-এর কোড reference-এর বদলে একটি `Box<T>` ব্যবহার করে নতুন করে লিখতে পারি; Listing 15-7-এ `Box<T>`-এর ওপর ব্যবহৃত dereference operator ঠিক সেভাবেই
কাজ করে, যেভাবে Listing 15-6-এ reference-এর ওপর ব্যবহৃত dereference operator কাজ করে।

<Listing number="15-7" file-name="src/main.rs" caption="একটি `Box<i32>`-এর ওপর dereference operator ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

</Listing>

Listing 15-7 আর Listing 15-6-এর মূল পার্থক্য হলো, এখানে আমরা `y`-কে `x`-এর মানকে নির্দেশ করা reference নয়, বরং `x`-এর কপি করা মানকে নির্দেশ করা একটি box-এর
instance বানাই। শেষ assertion-এ আমরা box-এর pointer ধরে মানে পৌঁছাতে dereference operator ব্যবহার করতে পারি, ঠিক যেমন `y` reference থাকাকালে করেছিলাম। এরপর
নিজস্ব box type সংজ্ঞায়িত করে দেখব `Box<T>`-এর কী বিশেষত্ব আমাদের dereference operator ব্যবহার করতে দেয়।

### নিজস্ব smart pointer সংজ্ঞায়িত করা {#defining-our-own-smart-pointer}

standard library-র দেওয়া `Box<T>` type-এর মতো একটি wrapper type (অন্য মানকে মুড়ে রাখে এমন type) বানাই, যাতে নিজে দেখতে পারি default-ভাবে smart pointer type
reference থেকে কীভাবে ভিন্ন আচরণ করে। তারপর দেখব কীভাবে dereference operator ব্যবহারের ক্ষমতা যোগ করা যায়।

> Note: আমরা যে `MyBox<T>` type বানাতে যাচ্ছি আর আসল `Box<T>`-এর মধ্যে একটি বড় পার্থক্য আছে: আমাদের সংস্করণ তার ডেটা heap-এ রাখবে না। এই উদাহরণে আমরা
> `Deref`-এর দিকে মনোযোগ দিচ্ছি, তাই ডেটা আসলে কোথায় রাখা হচ্ছে তার চেয়ে pointer-এর মতো আচরণটিই বেশি গুরুত্বপূর্ণ।

`Box<T>` type মূলত একটি উপাদানের tuple struct হিসেবে সংজ্ঞায়িত, তাই Listing 15-8 একইভাবে একটি `MyBox<T>` type সংজ্ঞায়িত করে। `Box<T>`-এর ওপর সংজ্ঞায়িত
`new` function-এর সঙ্গে মিল রেখে একটি `new` function-ও সংজ্ঞায়িত করব।

<Listing number="15-8" file-name="src/main.rs" caption="একটি `MyBox<T>` type সংজ্ঞায়িত করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

</Listing>

আমরা `MyBox` নামের একটি struct সংজ্ঞায়িত করি এবং একটি generic parameter `T` ঘোষণা করি, কারণ আমরা চাই আমাদের type যেকোনো type-এর মান রাখতে পারুক। `MyBox`
type হলো `T` type-এর একটি উপাদানের tuple struct। `MyBox::new` function `T` type-এর একটি parameter নেয় এবং পাঠানো মানটি রাখা একটি `MyBox` instance return
করে।

চলুন Listing 15-7-এর `main` function Listing 15-8-এ যোগ করে দেখি, আর এটি বদলে `Box<T>`-এর বদলে আমাদের সংজ্ঞায়িত `MyBox<T>` type ব্যবহার করাই। Listing 15-9-এর
কোড compile হবে না, কারণ Rust জানে না `MyBox`-কে কীভাবে dereference করতে হয়।

<Listing number="15-9" file-name="src/main.rs" caption="reference আর `Box<T>` যেভাবে ব্যবহার করেছি, সেভাবেই `MyBox<T>` ব্যবহারের চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

</Listing>

এর ফলে পাওয়া compilation error এই:

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

আমাদের `MyBox<T>` type dereference করা যায় না, কারণ আমাদের type-এ সেই ক্ষমতা আমরা implement করিনি। `*` operator দিয়ে dereference সম্ভব করতে আমরা `Deref`
trait implement করি।

<!-- Old headings. Do not remove or links may break. -->

<a id="treating-a-type-like-a-reference-by-implementing-the-deref-trait"></a>

### `Deref` trait implement করা {#implementing-the-deref-trait}

অধ্যায় 10-এর [“একটি type-এর ওপর trait implement করা”][impl-trait]<!-- ignore -->-এ যেমন আলোচনা করেছি, একটি trait implement করতে trait-এর বাধ্যতামূলক
method-গুলোর implementation দিতে হয়। standard library-র দেওয়া `Deref` trait আমাদের `deref` নামের একটি method implement করতে বলে, যা `self` borrow করে এবং
ভেতরের ডেটার একটি reference return করে। Listing 15-10-এ `MyBox<T>`-এর সংজ্ঞায় যোগ করার জন্য `Deref`-এর একটি implementation আছে।

<Listing number="15-10" file-name="src/main.rs" caption="`MyBox<T>`-এর ওপর `Deref` implement করা">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

</Listing>

`type Target = T;` syntax `Deref` trait-এর ব্যবহারের জন্য একটি associated type সংজ্ঞায়িত করে। associated type হলো generic parameter ঘোষণার একটু ভিন্ন একটি উপায়,
তবে আপাতত এ নিয়ে চিন্তার দরকার নেই; অধ্যায় 20-এ এগুলো আরও বিস্তারিত আলোচনা করব।

`deref` method-এর body-তে আমরা `&self.0` লিখি, যাতে `deref` সেই মানের একটি reference return করে যা আমরা `*` operator দিয়ে পেতে চাই; অধ্যায় 5-এর [“tuple
struct দিয়ে আলাদা type তৈরি করা”][tuple-structs]<!--
ignore --> থেকে মনে করুন, `.0` একটি tuple struct-এর প্রথম মান দেয়। Listing 15-9-এর যে `main` function `MyBox<T>` মানের ওপর `*` call করে, তা এখন compile হয়,
আর assertion-গুলো pass করে!

`Deref` trait ছাড়া compiler শুধু `&` reference dereference করতে পারে। `deref` method compiler-কে এই ক্ষমতা দেয় যে `Deref` implement করে এমন যেকোনো type-এর একটি
মান নিয়ে `deref` method call করে এমন একটি reference পায়, যা কীভাবে dereference করতে হয় তা এটি জানে।

Listing 15-9-এ আমরা যখন `*y` লিখেছিলাম, তখন পর্দার আড়ালে Rust আসলে এই কোড চালিয়েছিল:

```rust,ignore
*(y.deref())
```

Rust `*` operator-এর জায়গায় `deref` method-এর একটি call আর তারপর একটি সাধারণ dereference বসিয়ে দেয়, যাতে `deref` method call করতে হবে কি না তা নিয়ে আমাদের
ভাবতে না হয়। Rust-এর এই বৈশিষ্ট্য আমাদের এমন কোড লিখতে দেয় যা সাধারণ reference বা `Deref` implement করা type, যা-ই থাকুক, একইভাবে কাজ করে।

`deref` method কেন একটি মানের reference return করে, আর `*(y.deref())`-এ বন্ধনীর বাইরের সাধারণ dereference কেন তবুও দরকার, তার কারণ ownership system-এর সঙ্গে
জড়িত। `deref` method যদি মানের reference-এর বদলে সরাসরি মানটি return করত, তাহলে মানটি `self` থেকে বাইরে সরে যেত। এই ক্ষেত্রে, বা dereference operator ব্যবহারের
বেশিরভাগ ক্ষেত্রে, আমরা `MyBox<T>`-এর ভেতরের মানের ownership নিতে চাই না।

খেয়াল করুন, আমাদের কোডে যতবার `*` ব্যবহার করি, প্রতিবার `*` operator-এর জায়গায় `deref` method-এর একটি call আর তারপর মাত্র একবার `*` operator-এর একটি call বসে।
`*` operator-এর প্রতিস্থাপন অসীম পর্যন্ত recursive-ভাবে চলে না বলে আমরা শেষ পর্যন্ত `i32` type-এর ডেটা পাই, যা Listing 15-9-এর `assert_eq!`-এর `5`-এর সঙ্গে মেলে।

<!-- Old headings. Do not remove or links may break. -->

<a id="implicit-deref-coercions-with-functions-and-methods"></a>
<a id="using-deref-coercions-in-functions-and-methods"></a>

### function ও method-এ deref coercion ব্যবহার করা {#using-deref-coercion-in-functions-and-methods}

_Deref coercion_ (deref-এর মাধ্যমে স্বয়ংক্রিয় type রূপান্তর) `Deref` trait implement করে এমন একটি type-এর reference-কে আরেকটি type-এর reference-এ রূপান্তর করে।
যেমন, deref coercion `&String`-কে `&str`-এ রূপান্তর করতে পারে, কারণ `String` `Deref` trait এমনভাবে implement করে যে এটি `&str` return করে। deref coercion হলো
function ও method-এর argument-এর ওপর Rust-এর দেওয়া একটি সুবিধা, এবং এটি শুধু `Deref` trait implement করে এমন type-এর ক্ষেত্রেই কাজ করে। এটি স্বয়ংক্রিয়ভাবে
ঘটে, যখন আমরা কোনো নির্দিষ্ট type-এর মানের reference এমন একটি function বা method-এ argument হিসেবে পাঠাই, যার সংজ্ঞার parameter type-এর সঙ্গে এটি মেলে না।
`deref` method-এর পরপর কয়েকটি call আমাদের দেওয়া type-কে parameter-এর দরকারি type-এ রূপান্তর করে।

deref coercion Rust-এ যোগ করা হয়েছে, যাতে function ও method call লেখা প্রোগ্রামারদের `&` আর `*` দিয়ে এত বেশি স্পষ্ট reference আর dereference লিখতে না হয়। deref
coercion বৈশিষ্ট্য আমাদের এমন আরও বেশি কোড লিখতে দেয় যা reference বা smart pointer, দুটির জন্যই কাজ করতে পারে।

deref coercion কাজ করতে দেখতে চলুন Listing 15-8-এ সংজ্ঞায়িত `MyBox<T>` type আর Listing 15-10-এ যোগ করা `Deref`-এর implementation ব্যবহার করি। Listing 15-11-এ
string slice parameter-যুক্ত একটি function-এর সংজ্ঞা দেখানো হয়েছে।

<Listing number="15-11" file-name="src/main.rs" caption="`&str` type-এর `name` parameter-যুক্ত একটি `hello` function">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

</Listing>

`hello` function-কে argument হিসেবে একটি string slice দিয়ে call করা যায়, যেমন `hello("Rust");`। deref coercion-এর কারণে `MyBox<String>` type-এর একটি মানের
reference দিয়েও `hello` call করা যায়, যেমনটি Listing 15-12-এ দেখানো হয়েছে।

<Listing number="15-12" file-name="src/main.rs" caption="একটি `MyBox<String>` মানের reference দিয়ে `hello` call করা, যা deref coercion-এর কারণে কাজ করে">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

</Listing>

এখানে আমরা `&m` argument দিয়ে `hello` function call করছি, যা একটি `MyBox<String>` মানের reference। Listing 15-10-এ `MyBox<T>`-এর ওপর `Deref` trait implement
করেছি বলে Rust `deref` call করে `&MyBox<String>`-কে `&String`-এ রূপান্তর করতে পারে। standard library `String`-এর ওপর `Deref`-এর একটি implementation দেয়, যা
একটি string slice return করে, আর এটি `Deref`-এর API documentation-এ আছে। Rust আবার `deref` call করে `&String`-কে `&str`-এ রূপান্তর করে, যা `hello` function-এর
সংজ্ঞার সঙ্গে মেলে।

Rust deref coercion implement না করলে `&MyBox<String>` type-এর একটি মান দিয়ে `hello` call করতে Listing 15-12-এর কোডের বদলে Listing 15-13-এর কোড লিখতে হতো।

<Listing number="15-13" file-name="src/main.rs" caption="Rust-এ deref coercion না থাকলে যে কোড লিখতে হতো">

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

</Listing>

`(*m)` `MyBox<String>`-কে dereference করে একটি `String`-এ পরিণত করে। তারপর `&` আর
`[..]` `hello`-এর signature-এর সঙ্গে মেলাতে `String`-এর এমন একটি string slice নেয়, যা পুরো string-এর সমান। deref coercion ছাড়া এই কোড এত সব চিহ্নের কারণে
পড়া, লেখা আর বোঝা কঠিন। deref coercion Rust-কে আমাদের হয়ে স্বয়ংক্রিয়ভাবে এই রূপান্তরগুলো সামলাতে দেয়।

সংশ্লিষ্ট type-গুলোর জন্য `Deref` trait সংজ্ঞায়িত থাকলে Rust type-গুলো বিশ্লেষণ করবে এবং parameter-এর type-এর সঙ্গে মেলে এমন reference পেতে যতবার দরকার
ততবার `Deref::deref` ব্যবহার করবে। `Deref::deref` কতবার বসাতে হবে তা compile time-এ ঠিক হয়, তাই deref coercion-এর সুবিধা নেওয়ায় runtime-এ কোনো মাশুল নেই!

<!-- Old headings. Do not remove or links may break. -->

<a id="how-deref-coercion-interacts-with-mutability"></a>

### mutable reference-এর সঙ্গে deref coercion {#handling-deref-coercion-with-mutable-references}

immutable reference-এর ওপর `*` operator override করতে যেমন `Deref` trait ব্যবহার করেন, তেমনি mutable reference-এর ওপর `*` operator override করতে `DerefMut` trait
ব্যবহার করতে পারেন।

Rust তিনটি ক্ষেত্রে type আর trait implementation খুঁজে পেলে deref coercion করে:

1. `&T` থেকে `&U`-তে, যখন `T: Deref<Target=U>`
2. `&mut T` থেকে `&mut U`-তে, যখন `T: DerefMut<Target=U>`
3. `&mut T` থেকে `&U`-তে, যখন `T: Deref<Target=U>`

প্রথম দুটি ক্ষেত্র একই, শুধু দ্বিতীয়টিতে mutability আছে। প্রথম ক্ষেত্র বলে, আপনার কাছে যদি একটি `&T` থাকে, আর `T` কোনো type `U`-তে `Deref` implement করে, তাহলে
আপনি অদৃশ্যভাবে একটি `&U` পেতে পারেন। দ্বিতীয় ক্ষেত্র বলে, mutable reference-এর জন্যও একই deref coercion ঘটে।

তৃতীয় ক্ষেত্রটি একটু জটিল: Rust একটি mutable reference-কে immutable reference-এও রূপান্তর করবে। কিন্তু উল্টোটা সম্ভব _নয়_: immutable reference কখনো mutable
reference-এ রূপান্তরিত হবে না। borrowing-এর নিয়মের কারণে, আপনার কাছে একটি mutable reference থাকলে সেই mutable reference-ই হতে হবে ওই ডেটার একমাত্র reference
(নইলে প্রোগ্রাম compile হতো না)। একটি mutable reference-কে একটি immutable reference-এ রূপান্তর করলে borrowing-এর নিয়ম কখনো ভাঙে না। একটি immutable reference-কে
mutable reference-এ রূপান্তর করতে হলে প্রাথমিক immutable reference-টিকে ওই ডেটার একমাত্র immutable reference হতে হতো, কিন্তু borrowing-এর নিয়ম তার নিশ্চয়তা দেয়
না। তাই Rust ধরে নিতে পারে না যে একটি immutable reference-কে mutable reference-এ রূপান্তর করা সম্ভব।

{{#quiz ../quizzes/ch15-02-deref.toml}}

[impl-trait]: ch10-02-traits.html#implementing-a-trait-on-a-type
[tuple-structs]: ch05-01-defining-structs.html#creating-different-types-with-tuple-structs
