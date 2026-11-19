## উন্নত function ও closure {#advanced-functions-and-closures}

এই অংশে function আর closure সংক্রান্ত কিছু উন্নত বৈশিষ্ট্য দেখা হবে, function pointer আর closure return করাসহ।

### function pointer {#function-pointers}

function-এ কীভাবে closure পাঠাতে হয় তা নিয়ে কথা বলেছি; function-এ সাধারণ function-ও পাঠানো যায়! নতুন closure সংজ্ঞায়িত না করে আগে থেকে সংজ্ঞায়িত একটি function পাঠাতে চাইলে এই কৌশল কাজে লাগে।
function `fn` type-এ (ছোট হাতের _f_ দিয়ে) রূপান্তরিত হয়, যাকে `Fn` closure trait-এর সঙ্গে গুলিয়ে ফেলবেন না। `fn` type-কে বলা হয় _function pointer_। function pointer দিয়ে function পাঠালে function-কে
অন্য function-এর argument হিসেবে ব্যবহার করা যায়।

একটি parameter যে function pointer তা উল্লেখ করার syntax closure-এর মতোই, যেমনটি Listing 20-28-এ দেখানো হয়েছে, যেখানে আমরা `add_one` নামের একটি function সংজ্ঞায়িত করেছি, যা তার parameter-এর সঙ্গে 1 যোগ
করে। `do_twice` function দুটি parameter নেয়: `i32` parameter নিয়ে `i32` return করে এমন যেকোনো function-এর একটি function pointer, আর একটি `i32` মান। `do_twice` function `f` function-কে দুবার call
করে, প্রতিবার `arg` মান পাঠিয়ে, তারপর দুটি function call-এর ফল যোগ করে। `main` function `add_one` আর `5` argument দিয়ে `do_twice` call করে।

<Listing number="20-28" file-name="src/main.rs" caption="argument হিসেবে একটি function pointer গ্রহণ করতে `fn` type ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-28/src/main.rs}}
```

</Listing>

এই কোড print করে `The answer is: 12`। আমরা উল্লেখ করি যে `do_twice`-এর `f` parameter একটি `fn`, যা `i32` type-এর একটি parameter নিয়ে একটি `i32` return করে। তারপর `do_twice`-এর body-তে `f` call করা
যায়। `main`-এ `do_twice`-এর প্রথম argument হিসেবে function-এর নাম `add_one` পাঠাতে পারি।

closure-এর মতো নয়, `fn` একটি trait নয়, বরং একটি type, তাই `Fn` trait-গুলোর একটিকে trait bound হিসেবে রেখে generic type parameter ঘোষণা না করে আমরা সরাসরি parameter-এর type হিসেবে `fn` উল্লেখ করি।

function pointer তিনটি closure trait-ই (`Fn`, `FnMut` আর `FnOnce`) implement করে, অর্থাৎ closure আশা করে এমন একটি function-এর argument হিসেবে সবসময় একটি function pointer পাঠাতে পারেন। একটি generic type আর
closure trait-গুলোর একটি ব্যবহার করে function লেখাই ভালো, যাতে আপনার function function বা closure, দুটিই গ্রহণ করতে পারে।

তবে যেখানে closure নয়, শুধু `fn` গ্রহণ করতে চাইবেন তার একটি উদাহরণ হলো closure নেই এমন বাইরের কোডের সঙ্গে যুক্ত হওয়া: C function argument হিসেবে function গ্রহণ করতে পারে, কিন্তু C-তে closure নেই।

যেখানে inline সংজ্ঞায়িত একটি closure বা নাম দেওয়া একটি function, যেকোনোটি ব্যবহার করা যায়, তার উদাহরণ হিসেবে চলুন standard library-র `Iterator` trait-এর দেওয়া `map` method-এর একটি ব্যবহার দেখি।
সংখ্যার একটি vector-কে string-এর vector-এ রূপান্তর করতে `map` method ব্যবহার করতে আমরা একটি closure ব্যবহার করতে পারি, যেমনটি Listing 20-29-এ।

<Listing number="20-29" caption="সংখ্যাকে string-এ রূপান্তর করতে `map` method-এর সঙ্গে একটি closure ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-29/src/main.rs:here}}
```

</Listing>

অথবা closure-এর বদলে `map`-এর argument হিসেবে একটি function-এর নাম দিতে পারি। Listing 20-30-এ দেখানো হয়েছে এটি দেখতে কেমন হবে।

<Listing number="20-30" caption="সংখ্যাকে string-এ রূপান্তর করতে `map` method-এর সঙ্গে `String::to_string` function ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-30/src/main.rs:here}}
```

</Listing>

খেয়াল করুন, [“উন্নত trait”][advanced-traits]<!-- ignore --> অংশে আলোচিত fully qualified syntax আমাদের ব্যবহার করতেই হবে, কারণ `to_string` নামে একাধিক function পাওয়া যায়।

এখানে আমরা `ToString` trait-এ সংজ্ঞায়িত `to_string` function ব্যবহার করছি, যা standard library `Display` implement করা যেকোনো type-এর জন্য implement করেছে।

অধ্যায়
6-এর [“enum-এর মান”][enum-values]<!-- ignore --> অংশ থেকে মনে করুন, আমাদের সংজ্ঞায়িত প্রতিটি enum variant-এর নাম একটি initializer function-ও হয়ে যায়। এই initializer function-গুলোকে closure trait implement করা
function pointer হিসেবে ব্যবহার করা যায়, অর্থাৎ closure নেয় এমন method-এর argument হিসেবে initializer function উল্লেখ করা যায়, যেমনটি Listing 20-31-এ দেখা যাচ্ছে।

<Listing number="20-31" caption="সংখ্যা থেকে একটি `Status` instance তৈরি করতে `map` method-এর সঙ্গে একটি enum initializer ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-31/src/main.rs:here}}
```

</Listing>

এখানে আমরা `Status::Value`-এর initializer function ব্যবহার করে, `map` যে range-এর ওপর call হয়েছে তার প্রতিটি `u32` মান দিয়ে `Status::Value` instance তৈরি করি। কেউ কেউ এই ধাঁচ পছন্দ করেন, আবার কেউ
closure ব্যবহার করতে পছন্দ করেন। এরা একই কোডে compile হয়, তাই আপনার কাছে যেটি বেশি পরিষ্কার সেটি ব্যবহার করুন।

### closure return করা {#returning-closures}

closure trait দিয়ে প্রকাশ করা হয়, অর্থাৎ সরাসরি closure return করা যায় না। যেসব ক্ষেত্রে একটি trait return করতে চাইতে পারেন, তার বেশিরভাগে এর বদলে function-এর return মান হিসেবে trait implement করা concrete
type ব্যবহার করা যায়। কিন্তু closure-এর ক্ষেত্রে সাধারণত তা করা যায় না, কারণ এদের return করার মতো concrete type নেই; যেমন, closure যদি তার scope থেকে কোনো মান capture করে, তাহলে return type হিসেবে
function pointer `fn` ব্যবহার করার অনুমতি নেই।

বরং আপনি সাধারণত অধ্যায় 10-এ শেখা `impl Trait` syntax ব্যবহার করবেন। `Fn`, `FnOnce` আর `FnMut` ব্যবহার করে যেকোনো function type return করা যায়। যেমন, Listing 20-32-এর কোড ঠিকঠাক compile হবে।

<Listing number="20-32" caption="`impl Trait` syntax ব্যবহার করে একটি function থেকে একটি closure return করা">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-32/src/lib.rs}}
```

</Listing>

তবে অধ্যায় 13-এর [“closure-এর type অনুমান ও annotate
করা”][closure-types]<!-- ignore --> অংশে যেমন বলেছিলাম, প্রতিটি closure-ও নিজস্ব আলাদা type। একই signature কিন্তু ভিন্ন implementation-এর একাধিক function নিয়ে কাজ করতে হলে সেগুলোর জন্য একটি
trait object ব্যবহার করতে হবে। Listing 20-33-এর মতো কোড লিখলে কী ঘটে ভাবুন।

<Listing file-name="src/main.rs" number="20-33" caption="`impl Fn` type return করা function দিয়ে সংজ্ঞায়িত closure-এর একটি `Vec<T>` তৈরি করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-33/src/main.rs}}
```

</Listing>

এখানে আমাদের দুটি function আছে, `returns_closure` আর `returns_initialized_closure`, যা দুটিই `impl Fn(i32) -> i32` return করে। খেয়াল করুন, এরা যে closure return করে সেগুলো ভিন্ন, যদিও এরা একই
type implement করে। এটি compile করার চেষ্টা করলে Rust জানায় যে এটি কাজ করবে না:

```text
{{#include ../listings/ch20-advanced-features/listing-20-33/output.txt}}
```

error message আমাদের বলে যে যখনই একটি `impl Trait` return করি, Rust একটি অনন্য _opaque type_ (অস্বচ্ছ type) তৈরি করে, এমন একটি type, যার ভেতরের খুঁটিনাটি আমরা দেখতে পাই না, আর Rust কোন type তৈরি
করবে তা আন্দাজ করে নিজেরাও লিখতে পারি না। তাই এই function-গুলো একই trait, `Fn(i32) -> i32`, implement করা closure return করলেও প্রতিটির জন্য Rust-এর তৈরি opaque type আলাদা। (এটি অনেকটা অধ্যায় 17-এর
[“`Pin` type ও `Unpin` trait”][future-types]<!-- ignore --> অংশে দেখা আচরণের মতো, যেখানে একই output type থাকলেও ভিন্ন ভিন্ন async block-এর জন্য Rust ভিন্ন concrete type তৈরি করে।) এই সমস্যার
একটি সমাধান আমরা কয়েকবার দেখেছি: একটি trait object ব্যবহার করা যায়, যেমনটি Listing 20-34-এ।

<Listing number="20-34" caption="`Box<dyn Fn>` return করা function দিয়ে সংজ্ঞায়িত closure-এর একটি `Vec<T>` তৈরি করা, যাতে এদের type একই হয়">

```rust
{{#rustdoc_include ../listings/ch20-advanced-features/listing-20-34/src/main.rs:here}}
```

</Listing>

এই কোড ঠিকঠাক compile হবে। trait object সম্পর্কে আরও জানতে অধ্যায় 18-এর [“সাধারণ আচরণের ওপর abstraction করতে trait object
ব্যবহার”][trait-objects]<!-- ignore --> অংশটি দেখুন।

এরপর macro দেখা যাক!

{{#quiz ../quizzes/ch19-05-advanced-functions-and-closures.toml}}

[advanced-traits]: ch20-02-advanced-traits.html#advanced-traits
[enum-values]: ch06-01-defining-an-enum.html#enum-values
[closure-types]: ch13-01-closures.html#closure-type-inference-and-annotation
[future-types]: ch17-03-more-futures.html
[trait-objects]: ch18-02-trait-objects.html
