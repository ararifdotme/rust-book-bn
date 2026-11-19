## vector দিয়ে মানের তালিকা রাখা {#storing-lists-of-values-with-vectors}

প্রথম যে collection type আমরা দেখব, তা হলো `Vec<T>`, যাকে vector-ও বলা হয়। vector দিয়ে একটিমাত্র data
structure-এ একাধিক মান রাখা যায়, যা সব মান মেমোরিতে পাশাপাশি রাখে। vector শুধু একই type-এর মান রাখতে
পারে। এগুলো কাজে আসে যখন আপনার কাছে আইটেমের একটি তালিকা থাকে, যেমন একটি ফাইলের লেখার লাইনগুলো বা একটি
শপিং কার্টের আইটেমগুলোর দাম।

### একটি নতুন vector তৈরি করা {#creating-a-new-vector}

একটি নতুন, ফাঁকা vector তৈরি করতে আমরা `Vec::new` function call করি, Listing 8-1-এ যেমন দেখানো
হয়েছে।

<Listing number="8-1" caption="`i32` type-এর মান রাখার জন্য একটি নতুন, ফাঁকা vector তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

</Listing>

লক্ষ করুন, এখানে আমরা একটি type annotation (type স্পষ্ট করে লিখে দেওয়া) যোগ করেছি। যেহেতু এই vector-এ
আমরা কোনো মান ঢোকাচ্ছি না, তাই আমরা কী ধরনের উপাদান রাখতে চাই, Rust তা জানে না। এটি একটি গুরুত্বপূর্ণ
বিষয়। vector implement করা হয়েছে generics (একাধিক টাইপের জন্য একই কোড) ব্যবহার করে; নিজের type-এর সঙ্গে
generics কীভাবে ব্যবহার করতে হয়, তা আমরা অধ্যায় 10-এ দেখব। আপাতত জেনে রাখুন, standard library-র দেওয়া
`Vec<T>` type যেকোনো type রাখতে পারে। একটি নির্দিষ্ট type রাখার জন্য vector তৈরি করার সময় আমরা কৌণিক বন্ধনীর
ভেতরে type-টি লিখে দিতে পারি। Listing 8-1-এ আমরা Rust-কে বলেছি যে `v`-এর `Vec<T>` `i32` type-এর উপাদান রাখবে।

বেশিরভাগ সময় আপনি শুরুর মানসহ একটি `Vec<T>` তৈরি করবেন, আর Rust আপনি কী type-এর মান রাখতে চান তা infer
(type অনুমান করে নেওয়া) করে নেবে, তাই এই type annotation খুব কমই দরকার হয়। সুবিধার জন্য Rust `vec!` macro
(কোড তৈরি করে এমন কোড) দেয়, যা আপনার দেওয়া মানগুলো রাখে এমন একটি নতুন vector তৈরি করে। Listing 8-2 একটি নতুন
`Vec<i32>` তৈরি করে, যা `1`, `2` আর `3` মান রাখে। integer type হলো `i32`, কারণ এটিই ডিফল্ট integer type,
যেমনটি আমরা অধ্যায় 3-এর [“data
type”][data-types]<!-- ignore --> অংশে আলোচনা করেছি।

<Listing number="8-2" caption="মানসহ একটি নতুন vector তৈরি করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

</Listing>

যেহেতু আমরা শুরুর `i32` মান দিয়েছি, তাই Rust infer করতে পারে যে `v`-এর type হলো `Vec<i32>`, আর type
annotation-এর দরকার নেই। এরপর আমরা দেখব কীভাবে একটি vector বদলাতে হয়।

### একটি vector আপডেট করা {#updating-a-vector}

একটি vector তৈরি করে তারপর তাতে উপাদান যোগ করতে আমরা `push` method ব্যবহার করতে পারি, Listing 8-3-এ যেমন
দেখানো হয়েছে।

<Listing number="8-3" caption="একটি vector-এ মান যোগ করতে `push` method ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

</Listing>

অন্য যেকোনো variable-এর মতোই, এর মান বদলাতে চাইলে অধ্যায় 3-এ যেমন আলোচনা করা হয়েছে, `mut` keyword ব্যবহার
করে একে mutable করতে হবে। আমরা যে সংখ্যাগুলো ভেতরে রাখি, সেগুলো সবই `i32` type-এর, আর Rust ডেটা থেকেই তা
infer করে, তাই `Vec<i32>` annotation-এর দরকার নেই।

### vector-এর উপাদান পড়া {#reading-elements-of-vectors}

vector-এ রাখা একটি মানকে reference করার দুটি উপায় আছে: index দিয়ে, অথবা `get` method ব্যবহার করে। নিচের
উদাহরণগুলোতে বাড়তি স্পষ্টতার জন্য এই function-গুলো থেকে return হওয়া মানের type আমরা annotate করেছি।

Listing 8-4-এ vector-এর একটি মানে পৌঁছানোর দুটি পদ্ধতিই দেখানো হয়েছে: index syntax আর `get` method।

<Listing number="8-4" caption="vector-এর একটি আইটেমে পৌঁছাতে index syntax আর `get` method ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

</Listing>

এখানে কয়েকটি খুঁটিনাটি লক্ষ করুন। তৃতীয় উপাদানটি পেতে আমরা index মান `2` ব্যবহার করি, কারণ vector-এর index
শূন্য থেকে শুরু হওয়া সংখ্যা দিয়ে হয়। `&` আর `[]` ব্যবহার করলে আমরা ওই index-এর উপাদানের একটি reference পাই।
`get` method-এ index-কে argument হিসেবে দিলে আমরা একটি `Option<&T>` পাই, যা `match`-এর সঙ্গে ব্যবহার করা যায়।

Rust একটি উপাদানকে reference করার এই দুটি উপায় দেয়, যাতে বিদ্যমান উপাদানগুলোর পরিসরের বাইরের index মান
ব্যবহারের চেষ্টা করলে প্রোগ্রাম কেমন আচরণ করবে, তা আপনি বেছে নিতে পারেন। উদাহরণ হিসেবে, দেখা যাক পাঁচটি
উপাদানের একটি vector থাকলে প্রতিটি কৌশল দিয়ে index 100-এর উপাদানে পৌঁছানোর চেষ্টা করলে কী ঘটে, Listing 8-5-এ
যেমন দেখানো হয়েছে।

<Listing number="8-5" caption="পাঁচটি উপাদানযুক্ত একটি vector-এ index 100-এর উপাদানে পৌঁছানোর চেষ্টা">

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

</Listing>

এই কোড চালালে প্রথম `[]` পদ্ধতিটি প্রোগ্রামকে panic (প্রোগ্রামের হঠাৎ থেমে যাওয়া) করাবে, কারণ এটি এমন একটি
উপাদানকে reference করে, যার অস্তিত্ব নেই। আপনি যদি চান vector-এর শেষের পরের কোনো উপাদানে পৌঁছানোর চেষ্টা হলে
প্রোগ্রাম ক্র্যাশ করুক, তাহলে এই পদ্ধতিটিই সবচেয়ে ভালো।

`get` method-এ vector-এর বাইরের একটি index দিলে এটি panic না করে `None` return করে। স্বাভাবিক পরিস্থিতিতেও
মাঝে মাঝে vector-এর পরিসরের বাইরের উপাদানে পৌঁছানোর চেষ্টা হতে পারে এমন হলে আপনি এই পদ্ধতি ব্যবহার করবেন।
তখন আপনার কোডে `Some(&element)` বা `None` সামলানোর যুক্তি থাকবে, যেমনটি অধ্যায় 6-এ আলোচনা করা হয়েছে। যেমন,
index হয়তো আসছে একজন মানুষের লেখা একটি সংখ্যা থেকে। তিনি ভুল করে খুব বড় একটি সংখ্যা লিখলে আর প্রোগ্রাম
একটি `None` মান পেলে আপনি ব্যবহারকারীকে জানাতে পারেন বর্তমান vector-এ কয়টি আইটেম আছে, আর একটি বৈধ মান লেখার
আরেকটি সুযোগ দিতে পারেন। টাইপের ভুলের কারণে প্রোগ্রাম ক্র্যাশ করানোর চেয়ে এটি অনেক বেশি ব্যবহারকারী-বান্ধব!

প্রোগ্রামের কাছে একটি বৈধ reference থাকলে borrow checker (borrow-এর নিয়ম যাচাই করা compiler-এর অংশ)
ownership আর borrowing-এর নিয়ম (অধ্যায় 4-এ আলোচিত) বলবৎ করে, যাতে এই reference আর vector-এর বিষয়বস্তুর অন্য
সব reference বৈধ থাকে। সেই নিয়মটি মনে করুন, যা বলে একই scope-এ mutable আর immutable reference রাখা যায় না।
Listing 8-6-এ সেই নিয়মটি প্রযোজ্য, যেখানে আমরা একটি vector-এর প্রথম উপাদানের একটি immutable reference ধরে
রেখে শেষে একটি উপাদান যোগ করার চেষ্টা করি। function-এর পরের অংশে আমরা যদি সেই উপাদানটি আবার উল্লেখ করারও
চেষ্টা করি, তাহলে এই প্রোগ্রাম কাজ করবে না।

<Listing number="8-6" caption="একটি আইটেমের reference ধরে রাখা অবস্থায় vector-এ একটি উপাদান যোগ করার চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

</Listing>

এই কোড compile করলে এই error আসবে:

```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

Listing 8-6-এর কোড দেখে মনে হতে পারে এটি কাজ করা উচিত: প্রথম উপাদানের একটি reference কেন vector-এর শেষের
পরিবর্তন নিয়ে মাথা ঘামাবে? এই error vector কীভাবে কাজ করে তার কারণে: vector যেহেতু মানগুলো মেমোরিতে পাশাপাশি
রাখে, তাই vector-এর শেষে একটি নতুন উপাদান যোগ করতে হলে, vector বর্তমানে যেখানে আছে সেখানে সব উপাদান পাশাপাশি
রাখার মতো যথেষ্ট জায়গা না থাকলে, নতুন মেমোরি বরাদ্দ করে পুরোনো উপাদানগুলো নতুন জায়গায় কপি করতে হতে পারে।
সেই ক্ষেত্রে প্রথম উপাদানের reference deallocate হওয়া মেমোরিকে নির্দেশ করত। borrowing-এর নিয়মগুলো প্রোগ্রামকে
সেই পরিস্থিতিতে পৌঁছাতে দেয় না।

> Note: `Vec<T>` type-এর implementation-এর খুঁটিনাটি সম্পর্কে আরও জানতে দেখুন [“The
> Rustonomicon”][nomicon]।

### vector-এর মানগুলোর ওপর iterate করা {#iterating-over-the-values-in-a-vector}

একটি vector-এর প্রতিটি উপাদানে একে একে পৌঁছাতে আমরা index দিয়ে একটি করে উপাদানে না গিয়ে সব উপাদানের ওপর
iterate (একে একে প্রতিটি উপাদানে যাওয়া) করব। Listing 8-7-এ দেখানো হয়েছে, `i32` মানের একটি vector-এর প্রতিটি
উপাদানের immutable reference পেতে এবং সেগুলো প্রিন্ট করতে কীভাবে একটি `for` loop ব্যবহার করতে হয়।

<Listing number="8-7" caption="একটি `for` loop দিয়ে উপাদানগুলোর ওপর iterate করে vector-এর প্রতিটি উপাদান প্রিন্ট করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

</Listing>

`i` যে সংখ্যাটিকে নির্দেশ করে, তা পড়তে আমাদের `*` dereference operator ব্যবহার করে `i`-এর মানে পৌঁছাতে হয়, তারপরই এর সঙ্গে 1 যোগ করা যায়, যেমনটি ["pointer dereference করলে তার ডেটায় পৌঁছানো যায়"][deref]-এ আলোচনা করা হয়েছে।

সব উপাদানে পরিবর্তন আনতে আমরা একটি mutable vector-এর প্রতিটি উপাদানের mutable reference-এর ওপরও iterate করতে
পারি। Listing 8-8-এর `for` loop প্রতিটি উপাদানের সঙ্গে `50` যোগ করবে।

<Listing number="8-8" caption="একটি vector-এর উপাদানগুলোর mutable reference-এর ওপর iterate করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

</Listing>

mutable reference যে মানকে নির্দেশ করে, তা বদলাতে আবারও আমরা `*` dereference operator ব্যবহার করে `i`-এর মানে পৌঁছাই, তারপরই `+=` operator ব্যবহার করা যায়। 

{{#quiz ../quizzes/ch08-01-vec-sec1.toml}}

### নিরাপদে iterator ব্যবহার করা {#safely-using-iterators}

iterator কীভাবে কাজ করে, তা নিয়ে আমরা অধ্যায় 13.2-এর ["Processing a Series of Items with Iterators"](ch13-02-iterators.html)-এ আরও আলোচনা করব।
আপাতত একটি গুরুত্বপূর্ণ খুঁটিনাটি হলো, iterator-এর ভেতরে vector-এর ডেটার একটি pointer থাকে। একটি for-loop-কে
[`Vec::iter`] আর [`Iterator::next`]-এর সংশ্লিষ্ট method call-এ desugar (syntactic sugar খুলে মূল রূপে লেখা) করে আমরা দেখতে পারি iterator কীভাবে কাজ করে:

```aquascope,interpreter,horizontal
#fn main() {
#use std::slice::Iter;  
let mut v: Vec<i32>         = vec![1, 2];
let mut iter: Iter<'_, i32> = v.iter();`[]`
let n1: &i32                = iter.next().unwrap();`[]`
let n2: &i32                = iter.next().unwrap();`[]`
let end: Option<&i32>       = iter.next();`[]`
#}
```

লক্ষ করুন, iterator `iter` হলো একটি pointer, যা vector-এর প্রতিটি উপাদানের ওপর দিয়ে সরে যায়। `next` method iterator-কে
এক ধাপ এগিয়ে দেয় এবং আগের উপাদানের একটি ঐচ্ছিক reference return করে: হয় `Some` (যা আমরা unwrap করি), নয়তো vector-এর শেষে `None`।

এই খুঁটিনাটি নিরাপদে vector ব্যবহারের জন্য প্রাসঙ্গিক। যেমন, ধরুন আমরা একটি vector-কে নিজ জায়গাতেই দ্বিগুণ করতে চাই, যেমন `[1, 2]` হয়ে যাবে `[1, 2, 1, 2]`।
একটি সরল implementation দেখতে হয়তো এরকম হতো, যাতে compiler-এর infer করা permission (অনুমতি)-গুলো দেখানো হয়েছে:

```aquascope,permissions,stepper,boundaries,shouldFail
fn dup_in_place(v: &mut Vec<i32>) {
    for n_ref in v.iter() {`(focus,paths:*v)`
        v.push(*n_ref);`{}`
    }
}
```

লক্ষ করুন, `v.iter()` `*v` থেকে @Perm{write} permission সরিয়ে নেয়। তাই `v.push(..)` operation-এর প্রত্যাশিত @Perm{write} permission নেই। Rust compiler সংশ্লিষ্ট error message দিয়ে এই প্রোগ্রাম বাতিল করবে:

```text
error[E0502]: cannot borrow `*v` as mutable because it is also borrowed as immutable
 --> test.rs:3:9
  |
2 |     for n_ref in v.iter() {
  |                  --------
  |                  |
  |                  immutable borrow occurs here
  |                  immutable borrow later used here
3 |         v.push(*n_ref);
  |         ^^^^^^^^^^^^^^ mutable borrow occurs here
```

অধ্যায় 4-এ যেমন আলোচনা করেছি, এই error-এর পেছনের নিরাপত্তা-সমস্যা হলো deallocate হওয়া মেমোরি পড়া। `v.push(1)` ঘটামাত্র vector তার বিষয়বস্তু নতুন করে বরাদ্দ করবে এবং iterator-এর pointer অবৈধ করে দেবে। তাই নিরাপদে iterator ব্যবহারের জন্য Rust iteration চলার সময় vector-এ উপাদান যোগ বা বাদ দিতে দেয় না।

<!-- TODO: add loop support and make this diagram look reasonable -->
<!-- ```aquascope,interpreter,shouldFail,horizontal
fn dup_in_place(v: &mut Vec<i32>) {`[]`
    for n_ref in v.iter() {
        v.push(*n_ref);
    }`[]`
}
fn main() {
    let mut v = vec![1, 2, 3];
    dup_in_place(&mut v);
}
``` -->

pointer ব্যবহার না করে একটি vector-এর ওপর iterate করার একটি উপায় হলো range, যেমনটি আমরা [অধ্যায় 4.4](ch04-04-slices.html#range-syntax)-এ string slice-এর জন্য ব্যবহার করেছিলাম। যেমন, range `0 .. v.len()` হলো একটি vector `v`-এর সব index-এর ওপর একটি iterator, এখানে যেমন দেখা যাচ্ছে:

```aquascope,interpreter,horizontal
#fn main() {
#use std::ops::Range; 
let mut v: Vec<i32>        = vec![1, 2];
let mut iter: Range<usize> = 0 .. v.len();`[]`
let i1: usize              = iter.next().unwrap();
let n1: &i32               = &v[i1];`[]`
#}
```

### একাধিক type রাখতে enum ব্যবহার করা {#using-an-enum-to-store-multiple-types}

vector শুধু একই type-এর মান রাখতে পারে। এটি অসুবিধাজনক হতে পারে; বিভিন্ন type-এর আইটেমের তালিকা রাখার দরকার
হয় এমন ক্ষেত্র অবশ্যই আছে। সৌভাগ্যক্রমে, একটি enum-এর variant-গুলো একই enum type-এর অধীনে সংজ্ঞায়িত, তাই
বিভিন্ন type-এর উপাদান প্রকাশ করতে যখন একটি type দরকার, তখন আমরা একটি enum সংজ্ঞায়িত করে ব্যবহার করতে পারি!

যেমন, ধরুন আমরা একটি spreadsheet-এর একটি সারি থেকে মান নিতে চাই, যেখানে সারির কিছু কলামে integer, কিছুতে
floating-point সংখ্যা, আর কিছুতে string আছে। আমরা একটি enum সংজ্ঞায়িত করতে পারি, যার variant-গুলো বিভিন্ন type-এর
মান রাখবে, আর সব enum variant একই type হিসেবে গণ্য হবে: enum-টির type। তারপর আমরা সেই enum রাখার জন্য একটি
vector তৈরি করতে পারি, ফলে শেষ পর্যন্ত বিভিন্ন type রাখা যায়। Listing 8-9-এ আমরা এটি দেখিয়েছি।

<Listing number="8-9" caption="একটি vector-এ বিভিন্ন type-এর মান রাখতে একটি enum সংজ্ঞায়িত করা">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

</Listing>

compile time-এ Rust-কে জানতে হয় vector-এ কোন কোন type থাকবে, যাতে সে ঠিক জানে প্রতিটি উপাদান রাখতে heap-এ কতটা
মেমোরি লাগবে। এই vector-এ কোন কোন type রাখার অনুমতি আছে, তা নিয়েও আমাদের স্পষ্ট হতে হবে। Rust যদি একটি vector-কে
যেকোনো type রাখতে দিত, তাহলে vector-এর উপাদানগুলোর ওপর চালানো operation-এর সঙ্গে এক বা একাধিক type error ঘটানোর
সম্ভাবনা থাকত। enum আর একটি `match` expression ব্যবহার করা মানে, অধ্যায় 6-এ যেমন আলোচনা করা হয়েছে, Rust
compile time-এ নিশ্চিত করবে যে সব সম্ভাব্য ক্ষেত্র সামলানো হয়েছে।

runtime-এ একটি প্রোগ্রাম vector-এ রাখার জন্য কোন কোন type পাবে, তার পূর্ণ সেট যদি আপনি না জানেন, তাহলে enum-এর
কৌশল কাজ করবে না। এর বদলে আপনি একটি trait object (trait-এর মাধ্যমে বিভিন্ন type ব্যবহার করার মান) ব্যবহার করতে
পারেন, যা নিয়ে আমরা অধ্যায় 18-এ আলোচনা করব।

vector ব্যবহারের সবচেয়ে প্রচলিত কয়েকটি উপায় নিয়ে আলোচনা হলো, এবার standard library `Vec<T>`-এর ওপর যে অনেক
কাজের method সংজ্ঞায়িত করেছে, সেগুলো দেখতে অবশ্যই [API ডকুমেন্টেশন][vec-api]<!-- ignore --> দেখে নিন। যেমন,
`push`-এর পাশাপাশি একটি `pop` method আছে, যা শেষ উপাদানটি সরিয়ে return করে।

### vector drop করলে এর উপাদানগুলোও drop হয় {#dropping-a-vector-drops-its-elements}

অন্য যেকোনো `struct`-এর মতোই, একটি vector scope-এর বাইরে গেলে free হয়ে যায়, যেমনটি Listing 8-10-এ চিহ্নিত
করা হয়েছে।

<Listing number="8-10" caption="vector আর এর উপাদানগুলো কোথায় drop হয় তা দেখানো">

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

</Listing>

vector drop হলে এর সব বিষয়বস্তুও drop হয়, অর্থাৎ এতে রাখা integer-গুলো মুছে ফেলা হবে। borrow checker নিশ্চিত
করে যে vector-এর বিষয়বস্তুর যেকোনো reference শুধু vector-টি নিজে বৈধ থাকা অবস্থাতেই ব্যবহার হয়।

চলুন পরের collection type-এ যাই: `String`!

{{#quiz ../quizzes/ch08-01-vec-sec2.toml}}

[data-types]: ch03-02-data-types.html#data-types
[nomicon]: https://doc.rust-lang.org/nomicon/vec/vec.html
[vec-api]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[deref]: ch04-02-references-and-borrowing.html#dereferencing-a-pointer-accesses-its-data
[`Vec::iter`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter
[`Iterator::next`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#tymethod.next
