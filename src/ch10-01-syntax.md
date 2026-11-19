## generic data type {#generic-data-types}

function signature বা struct-এর মতো item-এর এমন সংজ্ঞা তৈরি করতে আমরা generics ব্যবহার করি, যা পরে অনেক রকম
concrete (নির্দিষ্ট) data type-এর সঙ্গে ব্যবহার করা যায়। চলুন প্রথমে দেখি generics দিয়ে কীভাবে function, struct, enum
ও method সংজ্ঞায়িত করা যায়। তারপর আলোচনা করব generics কোডের performance-এ কী প্রভাব ফেলে।

### function-এর সংজ্ঞায় {#in-function-definitions}

generics ব্যবহার করে এমন function সংজ্ঞায়িত করার সময় আমরা generics-কে function-এর signature-এ সেই জায়গায় বসাই,
যেখানে সাধারণত parameter আর return মানের data type লিখতাম। এতে আমাদের কোড আরও নমনীয় হয়, function-এর
caller-রা আরও বেশি সুবিধা পায়, আর কোডের পুনরাবৃত্তিও এড়ানো যায়।

আমাদের `largest` function নিয়েই এগিয়ে যাই। Listing 10-4-এ দুটি function দেখানো হয়েছে, যারা দুজনেই একটি slice-এ সবচেয়ে
বড় মান খোঁজে। তারপর আমরা এগুলোকে মিলিয়ে generics ব্যবহার করে এমন একটি function বানাব।

<Listing number="10-4" file-name="src/main.rs" caption="দুটি function, যাদের পার্থক্য শুধু নামে আর signature-এর type-এ">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

</Listing>

`largest_i32` function-টি আমরা Listing 10-3-এ বের করেছিলাম; এটি একটি slice-এ সবচেয়ে বড় `i32` খুঁজে বের করে।
`largest_char` function-টি একটি slice-এ সবচেয়ে বড় `char` খোঁজে। দুটি function-এর body-তে একই কোড, তাই চলুন একটিমাত্র
function-এ একটি generic type parameter (যেকোনো type বসানো যায় এমন জায়গা-রাখা নাম) এনে পুনরাবৃত্তিটি দূর করি।

নতুন একক function-এ type-গুলোকে parameter হিসেবে নিতে হলে type parameter-টির একটি নাম দিতে হবে, ঠিক যেমন function-এর
মানের parameter-এর নাম দিই। type parameter-এর নাম হিসেবে যেকোনো identifier ব্যবহার করা যায়। তবে আমরা `T` ব্যবহার
করব, কারণ প্রচলিত রীতি অনুযায়ী Rust-এ type parameter-এর নাম ছোট হয়, প্রায়ই মাত্র একটি অক্ষর, আর Rust-এ type-এর
নামকরণের রীতি হলো UpperCamelCase (প্রতিটি শব্দের প্রথম অক্ষর বড় হাতের, শব্দের মাঝে ফাঁক নেই)। _type_ শব্দের
সংক্ষেপ `T`-ই বেশিরভাগ Rust প্রোগ্রামারের স্বাভাবিক পছন্দ।

function-এর body-তে কোনো parameter ব্যবহার করলে signature-এ parameter-এর নামটি ঘোষণা করতে হয়, যাতে compiler জানে ওই
নামের মানে কী। একইভাবে, function signature-এ কোনো type parameter-এর নাম ব্যবহার করলে ব্যবহারের আগে সেই নামটি ঘোষণা
করতে হয়। generic `largest` function সংজ্ঞায়িত করতে আমরা type-এর নামের ঘোষণা angle bracket `<>`-এর ভেতরে,
function-এর নাম আর parameter তালিকার মাঝখানে বসাই, এভাবে:

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

এই সংজ্ঞাটি আমরা পড়ি এভাবে: “`largest` function-টি কোনো এক type `T`-এর ওপর generic।” এই function-এর `list` নামে একটি
parameter আছে, যা `T` type-এর মানের একটি slice। `largest` function একই `T` type-এর একটি মানের reference return করবে।

Listing 10-5-এ একত্রিত `largest` function-এর সংজ্ঞা দেখানো হয়েছে, যার signature-এ generic data type ব্যবহার করা হয়েছে।
listing-টিতে এটাও দেখানো হয়েছে যে function-টি `i32` মানের slice বা `char` মানের slice, দুটো দিয়েই কীভাবে call করা
যায়। খেয়াল করুন, এই কোড এখনো compile হবে না।

<Listing number="10-5" file-name="src/main.rs" caption="generic type parameter ব্যবহার করা `largest` function; এটি এখনো compile হয় না">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

</Listing>

এখনই এই কোড compile করলে আমরা এই error পাব:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

<!-- BEGIN INTERVENTION: 0aad53ff-89d7-4d14-8e3d-c17809220252 -->
ওপরের সমস্যাটি হলো, `largest` যখন input হিসেবে একটি slice `&[T]` নেয়, তখন function-টি `T` type সম্পর্কে *কিছুই* ধরে নিতে পারে না। এটি `i32` হতে পারে, `String` হতে পারে, আবার [`File`](https://doc.rust-lang.org/std/fs/struct.File.html)-ও হতে পারে। অথচ `largest`-এর দরকার যে `T` এমন কিছু হবে যাকে `>` দিয়ে তুলনা করা যায় (অর্থাৎ `T` `PartialOrd` implement করে; এই trait নিয়ে পরের অংশে আলোচনা করব)। `i32` আর `String`-এর মতো কিছু type তুলনা করা যায়, কিন্তু `File`-এর মতো অন্য type তুলনা করা যায় না।

[templates](https://en.cppreference.com/w/cpp/language/templates) (C++-এর generic কোড লেখার ব্যবস্থা) আছে এমন C++-এর মতো ভাষায় compiler `largest`-এর implementation নিয়ে আপত্তি করত না, বরং আপত্তি করত যখন কেউ, ধরুন, একটি file slice `&[File]`-এর ওপর `largest` call করার চেষ্টা করত। Rust এর বদলে চায় আপনি generic type-এর প্রত্যাশিত ক্ষমতাগুলো শুরুতেই জানিয়ে দিন। `T`-কে তুলনাযোগ্য হতে হলে `largest`-কে সেটা বলে দিতে হবে। তাই এই compiler error বলছে, `T`-কে সীমাবদ্ধ না করা পর্যন্ত `largest` compile হবে না।

তা ছাড়া Java-র মতো ভাষায় সব object-এর [`Object.toString()`](https://docs.oracle.com/javase/7/docs/api/java/lang/Object.html#toString())-এর মতো কিছু মৌলিক method থাকে, কিন্তু Rust-এ এমন কোনো মৌলিক method নেই। কোনো বাধ্যবাধকতা ছাড়া একটি generic type `T`-এর কোনো ক্ষমতাই নেই: একে print, clone বা পরিবর্তন করা যায় না (যদিও drop করা যায়)।
<!-- END INTERVENTION -->

### struct-এর সংজ্ঞায় {#in-struct-definitions}

`<>` syntax ব্যবহার করে আমরা struct-এর এক বা একাধিক field-এ generic type parameter ব্যবহার করতেও পারি। Listing 10-6-এ
যেকোনো type-এর `x` ও `y` স্থানাঙ্ক রাখার জন্য একটি `Point<T>` struct সংজ্ঞায়িত করা হয়েছে।

<Listing number="10-6" file-name="src/main.rs" caption="`T` type-এর `x` ও `y` মান রাখে এমন একটি `Point<T>` struct">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

</Listing>

struct-এর সংজ্ঞায় generics ব্যবহারের syntax function-এর সংজ্ঞার মতোই। প্রথমে struct-এর নামের ঠিক পরে angle
bracket-এর ভেতরে type parameter-এর নাম ঘোষণা করি। তারপর struct-এর সংজ্ঞায় যেখানে নইলে concrete data type লিখতাম,
সেখানে generic type-টি ব্যবহার করি।

খেয়াল করুন, `Point<T>` সংজ্ঞায়িত করতে আমরা মাত্র একটি generic type ব্যবহার করেছি, তাই এই সংজ্ঞা বলছে `Point<T>` struct
কোনো এক type `T`-এর ওপর generic, আর `x` ও `y` field _দুটিই_ সেই একই type-এর, type-টি যা-ই হোক না কেন। Listing 10-7-এর
মতো ভিন্ন ভিন্ন type-এর মান দিয়ে `Point<T>`-এর একটি instance (কোনো type-এর একটি নির্দিষ্ট মান) তৈরি করলে আমাদের কোড
compile হবে না।

<Listing number="10-7" file-name="src/main.rs" caption="`x` ও `y` field-কে একই type-এর হতে হবে, কারণ দুটিরই generic data type একই `T`।">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

</Listing>

এই উদাহরণে `x`-এ integer মান `5` বসিয়ে আমরা compiler-কে জানিয়ে দিই যে `Point<T>`-এর এই instance-এর জন্য generic type
`T` হবে একটি integer। তারপর `y`-এর জন্য `4.0` দিলে, যাকে আমরা `x`-এর মতো একই type-এর বলে সংজ্ঞায়িত করেছি, এমন
একটি type mismatch error পাব:

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

এমন একটি `Point` struct সংজ্ঞায়িত করতে, যেখানে `x` ও `y` দুটিই generic কিন্তু ভিন্ন type-এর হতে পারে, আমরা একাধিক
generic type parameter ব্যবহার করতে পারি। যেমন Listing 10-8-এ আমরা `Point`-এর সংজ্ঞা বদলে একে `T` ও `U` type-এর ওপর
generic করেছি, যেখানে `x`-এর type `T` আর `y`-এর type `U`।

<Listing number="10-8" file-name="src/main.rs" caption="দুটি type-এর ওপর generic একটি `Point<T, U>`, যাতে `x` ও `y` ভিন্ন type-এর মান হতে পারে">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

</Listing>

এখন দেখানো `Point`-এর সব instance-ই বৈধ! একটি সংজ্ঞায় যত খুশি generic type parameter ব্যবহার করতে পারেন, তবে কয়েকটির
বেশি হলে কোড পড়া কঠিন হয়ে যায়। যদি দেখেন আপনার কোডে অনেক generic type লাগছে, তাহলে হয়তো কোডটিকে ছোট ছোট অংশে
নতুন করে সাজানো দরকার।

### enum-এর সংজ্ঞায় {#in-enum-definitions}

struct-এর মতোই আমরা enum-এর variant-এ generic data type রাখতে পারি। standard library-র দেওয়া `Option<T>` enum-টি আরেকবার
দেখা যাক, যা আমরা অধ্যায় 6-এ ব্যবহার করেছিলাম:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

এই সংজ্ঞাটি এখন আপনার কাছে আরও অর্থবহ মনে হওয়ার কথা। দেখতেই পাচ্ছেন, `Option<T>` enum `T` type-এর ওপর generic, আর এর
দুটি variant আছে: `Some`, যা `T` type-এর একটি মান রাখে, আর `None` variant, যা কোনো মান রাখে না। `Option<T>` enum ব্যবহার
করে আমরা একটি ঐচ্ছিক মানের বিমূর্ত ধারণা প্রকাশ করতে পারি, আর `Option<T>` generic হওয়ায় ঐচ্ছিক মানটির type যা-ই হোক,
এই abstraction ব্যবহার করা যায়।

enum একাধিক generic type-ও ব্যবহার করতে পারে। অধ্যায় 9-এ ব্যবহার করা `Result` enum-এর সংজ্ঞা এর একটি উদাহরণ:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Result` enum দুটি type, `T` ও `E`-এর ওপর generic, আর এর দুটি variant আছে: `Ok`, যা `T` type-এর একটি মান রাখে, আর `Err`,
যা `E` type-এর একটি মান রাখে। এই সংজ্ঞার ফলে যেখানেই এমন কোনো operation আছে যা সফল হতে পারে (কোনো `T` type-এর মান
return করে) বা ব্যর্থ হতে পারে (কোনো `E` type-এর error return করে), সেখানেই `Result` enum সহজে ব্যবহার করা যায়। আসলে
Listing 9-3-এ ফাইল খুলতে আমরা এটাই ব্যবহার করেছিলাম: ফাইল সফলভাবে খুললে `T`-এর জায়গায় বসেছিল `std::fs::File` type,
আর ফাইল খুলতে সমস্যা হলে `E`-এর জায়গায় বসেছিল `std::io::Error` type।

যখন দেখবেন আপনার কোডে একাধিক struct বা enum-এর সংজ্ঞা আছে যাদের পার্থক্য শুধু তারা কোন type-এর মান রাখে তাতে, তখন
তার বদলে generic type ব্যবহার করে পুনরাবৃত্তি এড়াতে পারেন।

### method-এর সংজ্ঞায় {#in-method-definitions}

আমরা struct ও enum-এর ওপর method implement করতে পারি (যেমন অধ্যায় 5-এ করেছিলাম), আর তাদের সংজ্ঞাতেও generic type
ব্যবহার করতে পারি। Listing 10-9-এ Listing 10-6-এ সংজ্ঞায়িত `Point<T>` struct দেখানো হয়েছে, যার ওপর `x` নামের একটি
method implement করা হয়েছে।

<Listing number="10-9" file-name="src/main.rs" caption="`Point<T>` struct-এর ওপর `x` নামের একটি method implement করা, যা `T` type-এর `x` field-এর একটি reference return করবে">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

</Listing>

এখানে আমরা `Point<T>`-এর ওপর `x` নামের একটি method সংজ্ঞায়িত করেছি, যা `x` field-এর ডেটার একটি reference return করে।

খেয়াল করুন, `impl`-এর ঠিক পরে আমাদের `T` ঘোষণা করতে হয়, যাতে `T` ব্যবহার করে বোঝাতে পারি যে আমরা `Point<T>` type-এর
ওপর method implement করছি। `impl`-এর পরে `T`-কে generic type হিসেবে ঘোষণা করায় Rust বুঝতে পারে যে `Point`-এর angle
bracket-এর ভেতরের type-টি concrete type নয়, বরং একটি generic type। struct-এর সংজ্ঞায় ঘোষিত generic parameter-এর চেয়ে
এই generic parameter-এর জন্য অন্য নাম বেছে নিতে পারতাম, তবে একই নাম ব্যবহার করাই প্রচলিত রীতি। generic type ঘোষণা করে
এমন একটি `impl`-এর ভেতরে কোনো method লিখলে, সেই method type-টির যেকোনো instance-এর ওপর সংজ্ঞায়িত থাকবে, শেষ পর্যন্ত
generic type-এর জায়গায় যে concrete type-ই বসুক।

type-এর ওপর method সংজ্ঞায়িত করার সময় আমরা generic type-এর ওপর শর্তও আরোপ করতে পারি। যেমন, যেকোনো generic type-সহ
`Point<T>` instance-এর বদলে শুধু `Point<f32>` instance-এর ওপর method implement করতে পারি। Listing 10-10-এ আমরা concrete
type `f32` ব্যবহার করেছি, অর্থাৎ `impl`-এর পরে কোনো type ঘোষণা করিনি।

<Listing number="10-10" file-name="src/main.rs" caption="একটি `impl` block, যা শুধু generic type parameter `T`-এর জায়গায় একটি নির্দিষ্ট concrete type থাকা struct-এর ক্ষেত্রে প্রযোজ্য">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

</Listing>

এই কোডের মানে হলো `Point<f32>` type-এর একটি `distance_from_origin` method থাকবে; `Point<T>`-এর অন্য যেসব instance-এ `T`
`f32` type নয়, তাদের জন্য এই method সংজ্ঞায়িত থাকবে না। method-টি মাপে আমাদের বিন্দুটি (0.0, 0.0) স্থানাঙ্কের বিন্দু
থেকে কত দূরে, আর এতে এমন গাণিতিক operation ব্যবহার করা হয় যা শুধু floating-point type-এর জন্যই পাওয়া যায়।

<!-- BEGIN INTERVENTION: 694bb2d0-f2e6-4b0b-a3e7-2d9f9e8b3d09 -->
এভাবে একই নামের নির্দিষ্ট *এবং* generic method একসঙ্গে implement করা যায় না। যেমন, যদি সব type `T`-এর জন্য একটি সাধারণ `distance_from_origin` এবং `f32`-এর জন্য একটি নির্দিষ্ট `distance_from_origin` implement করেন, তাহলে compiler আপনার প্রোগ্রাম বাতিল করবে: `Point<f32>::distance_from_origin` call করলে কোন implementation ব্যবহার করবে, Rust তা জানে না। আরও সাধারণভাবে বললে, object-oriented ভাষায় যেমন method-কে specialize (নির্দিষ্ট type-এর জন্য আলাদা রূপ দেওয়া) করার জন্য inheritance (উত্তরাধিকার)-এর মতো ব্যবস্থা থাকে, Rust-এ তেমন কিছু নেই, শুধু একটি ব্যতিক্রম ছাড়া (trait-এর default method), যা নিয়ে পরের অংশে আলোচনা করা হয়েছে।
<!-- END INTERVENTION -->

struct-এর সংজ্ঞার generic type parameter আর সেই struct-এর method signature-এ ব্যবহৃত generic type parameter সবসময় এক হয়
না। উদাহরণটি আরও পরিষ্কার করতে Listing 10-11-এ `Point` struct-এর জন্য `X1` ও `Y1` আর `mixup` method-এর signature-এর
জন্য `X2` ও `Y2` generic type ব্যবহার করা হয়েছে। method-টি একটি নতুন `Point` instance তৈরি করে, যার `x` মান আসে `self`
`Point` থেকে (type `X1`) আর `y` মান আসে পাঠানো `Point` থেকে (type `Y2`)।

<Listing number="10-11" file-name="src/main.rs" caption="এমন একটি method, যা তার struct-এর সংজ্ঞা থেকে ভিন্ন generic type ব্যবহার করে">

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

</Listing>

`main`-এ আমরা এমন একটি `Point` সংজ্ঞায়িত করেছি যার `x` একটি `i32` (মান `5`) আর `y` একটি `f64` (মান `10.4`)। `p2`
variable-টি এমন একটি `Point` struct যার `x` একটি string slice (মান `"Hello"`) আর `y` একটি `char` (মান `c`)। `p1`-এর ওপর
`p2` argument দিয়ে `mixup` call করলে আমরা পাই `p3`, যার `x` হবে `i32`, কারণ `x` এসেছে `p1` থেকে। `p3` variable-এর `y`
হবে `char`, কারণ `y` এসেছে `p2` থেকে। `println!` macro call-টি print করবে `p3.x = 5, p3.y = c`।

এই উদাহরণের উদ্দেশ্য এমন একটি পরিস্থিতি দেখানো যেখানে কিছু generic parameter `impl`-এর সঙ্গে ঘোষণা করা হয় আর কিছু
method-এর সংজ্ঞার সঙ্গে। এখানে `X1` ও `Y1` generic parameter ঘোষণা করা হয়েছে `impl`-এর পরে, কারণ এগুলো struct-এর
সংজ্ঞার সঙ্গে যায়। `X2` ও `Y2` generic parameter ঘোষণা করা হয়েছে `fn mixup`-এর পরে, কারণ এগুলো শুধু method-টির জন্যই
প্রাসঙ্গিক।

### generics ব্যবহার করা কোডের performance {#performance-of-code-using-generics}

হয়তো ভাবছেন, generic type parameter ব্যবহার করলে runtime-এ কোনো খরচ আছে কি না। সুখবর হলো, generic type ব্যবহার করলে
আপনার প্রোগ্রাম concrete type ব্যবহারের চেয়ে একটুও ধীরে চলবে না।

Rust এটি করে compile time-এ generics ব্যবহার করা কোডের monomorphization (generic কোডকে নির্দিষ্ট type-এর কোডে রূপান্তর)
করে। _Monomorphization_ হলো compile করার সময় ব্যবহৃত concrete type বসিয়ে generic কোডকে নির্দিষ্ট কোডে রূপান্তর করার
প্রক্রিয়া। এই প্রক্রিয়ায় compiler Listing 10-5-এর generic function তৈরি করতে আমাদের নেওয়া ধাপগুলোর উল্টোটা করে:
compiler সেই সব জায়গা দেখে যেখানে generic কোড call করা হয়েছে, আর generic কোডটি যেসব concrete type দিয়ে call করা হয়েছে
সেগুলোর জন্য কোড তৈরি করে।

standard library-র generic `Option<T>` enum ব্যবহার করে দেখা যাক এটি কীভাবে কাজ করে:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Rust এই কোড compile করার সময় monomorphization করে। সেই প্রক্রিয়ায় compiler `Option<T>` instance-গুলোতে ব্যবহৃত মানগুলো
পড়ে আর দুই ধরনের `Option<T>` শনাক্ত করে: একটি `i32`, অন্যটি `f64`। সেই অনুযায়ী এটি `Option<T>`-এর generic সংজ্ঞাকে
`i32` ও `f64`-এর জন্য নির্দিষ্ট দুটি সংজ্ঞায় বিস্তৃত করে, ফলে generic সংজ্ঞাটির জায়গা নেয় নির্দিষ্ট সংজ্ঞাগুলো।

কোডের monomorphize করা রূপটি দেখতে মোটামুটি নিচের মতো (বোঝানোর জন্য আমরা এখানে যে নাম ব্যবহার করছি, compiler তার
চেয়ে ভিন্ন নাম ব্যবহার করে):

<Listing file-name="src/main.rs">

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

</Listing>

generic `Option<T>`-এর জায়গা নেয় compiler-এর তৈরি নির্দিষ্ট সংজ্ঞাগুলো। Rust যেহেতু generic কোডকে এমন কোডে compile
করে যা প্রতিটি ক্ষেত্রে type নির্দিষ্ট করে দেয়, তাই generics ব্যবহারের জন্য আমাদের runtime-এ কোনো খরচ দিতে হয় না। কোড
চলার সময় এটি ঠিক ততটাই ভালো চলে, যতটা চলত যদি আমরা প্রতিটি সংজ্ঞা হাতে নকল করতাম। monomorphization প্রক্রিয়ার
কারণেই Rust-এর generics runtime-এ অত্যন্ত দক্ষ।

{{#quiz ../quizzes/ch10-01-generics.toml}}
