## slice type {#the-slice-type}

*slice* (collection-এর একটানা একটি অংশের reference) দিয়ে পুরো [collection](ch08-00-common-collections.md)-এর বদলে collection-এর উপাদানগুলোর একটি একটানা ক্রমকে reference করা যায়। 
slice এক ধরনের reference, তাই এটি একটি non-owning pointer (ডেটার মালিক নয় এমন pointer)।

slice কেন কাজের, তা বোঝাতে চলুন একটি ছোট প্রোগ্রামিং সমস্যা সমাধান করি: 
এমন একটি function লিখুন, যা space দিয়ে আলাদা করা শব্দের
একটি string নেয় এবং সেই string-এ খুঁজে পাওয়া প্রথম শব্দটি return করে।
function-টি string-এ কোনো space খুঁজে না পেলে পুরো string-টিই একটি শব্দ, তাই পুরো
string-টিই return করা উচিত। slice ছাড়া আমরা হয়তো function-টির signature
এভাবে লিখতাম:

```rust,ignore
fn first_word(s: &String) -> ?
```

`first_word` function-এর parameter একটি `&String`। আমরা string-টির
ownership চাই না, তাই এটা ঠিক আছে। কিন্তু return করব কী? string-এর একটি *অংশ* নিয়ে কথা বলার
কোনো উপায় আসলে আমাদের হাতে নেই। তবে শব্দের শেষ, অর্থাৎ space-এর অবস্থান বোঝানো
index-টি আমরা return করতে পারি। চলুন সেটাই চেষ্টা করি, Listing 4-7-এ যেমন দেখানো হয়েছে।

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

<span class="caption">Listing 4-7: `first_word` function, যা `String` parameter-এর
ভেতরের একটি byte index মান return করে</span>

যেহেতু `String`-এর উপাদানগুলো একটি একটি করে দেখে যাচাই করতে হবে কোনো মান space
কি না, তাই `as_bytes` method ব্যবহার করে আমরা আমাদের `String`-কে byte-এর একটি
array-তে রূপান্তর করব:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

এরপর `iter` method ব্যবহার করে byte-এর array-টির ওপর একটি iterator তৈরি করি:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

iterator নিয়ে আমরা [অধ্যায় 13][ch13]<!-- ignore -->-এ আরও বিস্তারিত আলোচনা করব।
আপাতত জেনে রাখুন, `iter` এমন একটি method, যা একটি collection-এর প্রতিটি উপাদান
return করে, আর `enumerate` `iter`-এর ফলাফলকে মুড়ে প্রতিটি উপাদানকে একটি tuple-এর
অংশ হিসেবে return করে। `enumerate` থেকে return হওয়া tuple-এর প্রথম উপাদান হলো
index, আর দ্বিতীয় উপাদান হলো উপাদানটির একটি reference।
নিজেরা index হিসাব করার চেয়ে এটি একটু বেশি সুবিধাজনক।

`enumerate` method যেহেতু একটি tuple return করে, তাই সেই tuple-কে destructure
করতে আমরা pattern ব্যবহার করতে পারি। pattern নিয়ে আমরা [অধ্যায়
6][ch6]<!-- ignore -->-এ আরও আলোচনা করব। `for` loop-এ আমরা একটি pattern দিই, যাতে
tuple-এর index-এর জন্য আছে `i`, আর tuple-এর একক byte-টির জন্য `&item`।
যেহেতু `.iter().enumerate()` থেকে আমরা উপাদানের একটি reference পাই, তাই pattern-এ
`&` ব্যবহার করি।

`for` loop-এর ভেতরে byte literal syntax ব্যবহার করে আমরা space বোঝানো byte-টি
খুঁজি। space পেলে তার অবস্থান return করি।
না পেলে `s.len()` ব্যবহার করে string-এর দৈর্ঘ্য return করি:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

এখন string-এর প্রথম শব্দের শেষের index খুঁজে বের করার একটি উপায় আমাদের আছে,
কিন্তু একটি সমস্যা আছে। আমরা আলাদাভাবে একটি `usize` return করছি, কিন্তু সংখ্যাটি
শুধু `&String`-এর প্রেক্ষাপটেই অর্থবহ। অন্যভাবে বললে, এটি `String` থেকে একটি
আলাদা মান হওয়ায় ভবিষ্যতেও যে এটি বৈধ থাকবে, তার কোনো নিশ্চয়তা নেই। Listing 4-8-এর
প্রোগ্রামটি দেখুন, যা Listing 4-7-এর `first_word` function ব্যবহার করে।

<span class="filename">Filename: src/main.rs</span>

```aquascope,interpreter+permissions,boundaries,stepper,horizontal
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");`(focus)`
    let word = first_word(&s);`[]`
    s.clear();`[]``{}`    
}
``` 

<span class="caption">Listing 4-8: `first_word` function call করার ফলাফল রেখে
তারপর `String`-এর বিষয়বস্তু বদলানো</span>

এই প্রোগ্রাম কোনো error ছাড়াই compile হয়, কারণ `first_word` call করার পরও `s`-এর
write permission থেকে যায়। `word` যেহেতু `s`-এর অবস্থার সঙ্গে মোটেও যুক্ত নয়,
তাই `word`-এ এখনো `5` মানটি আছে। প্রথম শব্দটি বের করতে আমরা সেই `5` মানটি `s`
variable-এর সঙ্গে ব্যবহারের চেষ্টা করতে পারতাম, কিন্তু তা হতো একটি বাগ,
কারণ `word`-এ `5` রাখার পর থেকে `s`-এর বিষয়বস্তু বদলে গেছে।

`word`-এর index `s`-এর ডেটার সঙ্গে অমিল হয়ে যাচ্ছে কি না, তা নিয়ে চিন্তা করা
ক্লান্তিকর এবং ভুলপ্রবণ! আমরা যদি একটি `second_word` function লিখি, তাহলে এই index
সামলানো আরও ভঙ্গুর হয়ে যায়। এর signature দেখতে হতো এরকম:

```rust,ignore
fn second_word(s: &String) -> (usize, usize) {
```

এখন আমরা শুরুর *এবং* শেষের একটি index-এর হিসাব রাখছি, আর আমাদের কাছে এমন আরও
মান আছে, যেগুলো একটি নির্দিষ্ট অবস্থার ডেটা থেকে হিসাব করা হয়েছে, কিন্তু সেই অবস্থার
সঙ্গে মোটেও বাঁধা নয়। আমাদের কাছে তিনটি সম্পর্কহীন variable ঘুরে বেড়াচ্ছে, যেগুলো
মিলিয়ে রাখতে হবে।

সৌভাগ্যক্রমে, এই সমস্যার একটি সমাধান Rust-এ আছে: string slice।

### string slice {#string-slices}

*string slice* (string-এর একটি অংশের reference) হলো একটি `String`-এর অংশের reference, আর এটি দেখতে এরকম:

```aquascope,interpreter
#fn main() {
let s = String::from("hello world");

let hello: &str = &s[0..5];
let world: &str = &s[6..11];
let s2: &String = &s; `[]`
#}
```

পুরো `String`-এর reference (যেমন `s2`)-এর বদলে `hello` হলো `String`-এর একটি
অংশের reference, যা বাড়তি `[0..5]` অংশে নির্দিষ্ট করা হয়েছে। ব্র্যাকেটের ভেতরে একটি
range দিয়ে আমরা slice তৈরি করি: `[starting_index..ending_index]`,
যেখানে `starting_index` হলো slice-এর প্রথম অবস্থান, আর `ending_index` হলো
slice-এর শেষ অবস্থানের চেয়ে এক বেশি।

slice বিশেষ ধরনের reference, কারণ এরা "fat" pointer, অর্থাৎ মেটাডেটাসহ
pointer। এখানে মেটাডেটা হলো slice-এর দৈর্ঘ্য। Rust-এর data structure-এর ভেতরে উঁকি
দেওয়ার জন্য আমাদের ভিজ্যুয়ালাইজেশন বদলে এই মেটাডেটা দেখতে পারি:

```aquascope,interpreter,concreteTypes,hideCode
fn main() {
    let s = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &String = &s; // not a slice, for comparison
    `[]`
}
```

লক্ষ করুন, `hello` আর `world` variable দুটিরই একটি `ptr` আর একটি `len` field আছে, যেগুলো মিলে heap-এ থাকা string-এর
নিচে দাগ দেওয়া অংশগুলো নির্ধারণ করে। এখানে আপনি এটাও দেখতে পাচ্ছেন যে একটি `String` আসলে দেখতে কেমন: string হলো byte-এর একটি vector (`Vec<u8>`),
যাতে আছে একটি দৈর্ঘ্য `len` আর একটি বাফার `buf`, আর বাফারটিতে আছে একটি pointer `ptr` ও একটি ধারণক্ষমতা `cap`।

slice যেহেতু reference, তাই এরাও reference করা ডেটার permission বদলায়। যেমন, নিচে লক্ষ করুন,
`hello`-কে `s`-এর একটি slice হিসেবে তৈরি করার সময় `s` write আর own permission হারায়:

```aquascope,permissions,stepper,boundaries
fn main() {
    let mut s = String::from("hello");
    let hello: &str = &s[0..5];
    println!("{hello}");
    s.push_str(" world");
}
```

#### range syntax {#range-syntax}

Rust-এর `..` range syntax-এ index শূন্য থেকে শুরু করতে চাইলে দুটি বিন্দুর আগের মানটি
বাদ দিতে পারেন। অন্যভাবে বললে, এগুলো সমান:

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

একইভাবে, আপনার slice-এ যদি `String`-এর শেষ byte-টি থাকে, তাহলে শেষের সংখ্যাটি
বাদ দিতে পারেন। অর্থাৎ এগুলো সমান:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

পুরো string-এর একটি slice নিতে দুটি মানই বাদ দিতে পারেন। তাই এগুলো
সমান:

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> Note: string slice-এর range index-গুলোকে অবশ্যই বৈধ UTF-8 অক্ষরের
> সীমানায় পড়তে হবে। একাধিক byte-এর কোনো অক্ষরের মাঝখানে string slice তৈরির চেষ্টা
> করলে আপনার প্রোগ্রাম একটি error দিয়ে বন্ধ হয়ে যাবে। string slice-এর সঙ্গে পরিচয়
> করানোর উদ্দেশ্যে এই অংশে আমরা শুধু ASCII ধরে নিচ্ছি; UTF-8 সামলানো নিয়ে আরও
> বিস্তারিত আলোচনা আছে অধ্যায় 8-এর [“string দিয়ে UTF-8 এনকোড করা
> লেখা রাখা”][strings]<!-- ignore --> অংশে।

#### string slice দিয়ে `first_word` নতুন করে লেখা {#rewriting-first_word-with-string-slices}

এই সব তথ্য মাথায় রেখে চলুন `first_word`-কে নতুন করে লিখি, যাতে এটি একটি
slice return করে। “string slice” বোঝানো type-টি লেখা হয় `&str`:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

শব্দের শেষের index আমরা Listing
4-7-এর মতো একইভাবে পাই, অর্থাৎ প্রথম space-টি খুঁজে। space পেলে string-এর শুরু আর
space-এর index-কে শুরু ও শেষের index হিসেবে ব্যবহার করে একটি string slice return
করি।

এখন `first_word` call করলে আমরা একটিমাত্র মান ফেরত পাই, যা মূল ডেটার সঙ্গে
বাঁধা। মানটি গঠিত slice-এর শুরুর বিন্দুর একটি reference আর slice-এর উপাদানের
সংখ্যা দিয়ে।

`second_word` function-এর ক্ষেত্রেও slice return করা কাজ করবে:

```rust,ignore
fn second_word(s: &String) -> &str {
```

এখন আমাদের কাছে একটি সরল API আছে, যাতে গোলমাল পাকানো অনেক কঠিন, কারণ
compiler নিশ্চিত করবে যে `String`-এর ভেতরের reference-গুলো বৈধ থাকে। Listing 4-8-এর
প্রোগ্রামের বাগটি মনে আছে, যেখানে আমরা প্রথম শব্দের শেষের index পেয়েছিলাম, কিন্তু
তারপর string-টি খালি করে দেওয়ায় index-টি অবৈধ হয়ে গিয়েছিল? ওই কোড যৌক্তিকভাবে
ভুল ছিল, কিন্তু সঙ্গে সঙ্গে কোনো error দেখায়নি। খালি করা string-এর সঙ্গে প্রথম শব্দের
index ব্যবহারের চেষ্টা চালিয়ে গেলে সমস্যাগুলো পরে দেখা দিত। slice এই বাগকে অসম্ভব
করে দেয় এবং আমাদের কোডে সমস্যা থাকলে অনেক আগেই তা জানিয়ে দেয়। যেমন:

<span class="filename">Filename: src/main.rs</span>

```aquascope,permissions,boundaries,stepper,shouldFail
#fn first_word(s: &String) -> &str {
#    let bytes = s.as_bytes();
#
#    for (i, &item) in bytes.iter().enumerate() {
#        if item == b' ' {
#            return &s[0..i];
#        }
#    }
#
#    &s[..]
#}
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);`(focus,paths:s)`
    s.clear();`{}`
    println!("the first word is: {}", word);
}
```

দেখতে পাচ্ছেন, `first_word` call করলে এখন `s` থেকে write permission সরে যায়,
যা আমাদের `s.clear()` call করতে বাধা দেয়। এই হলো compiler error:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```

borrowing-এর নিয়ম মনে করুন: কোনো কিছুর একটি immutable reference থাকলে
একই সঙ্গে আমরা তার mutable reference নিতে পারি না। `clear`-কে যেহেতু `String`
ছোট করে ফেলতে হয়, তাই এর একটি mutable reference দরকার। `clear` call-এর পরের
`println!` `word`-এর reference ব্যবহার করে, তাই ওই জায়গাতেও immutable
reference-টি সক্রিয় থাকতে হবে। Rust `clear`-এর mutable reference আর `word`-এর
immutable reference-কে একই সময়ে থাকতে দেয় না, ফলে compile ব্যর্থ হয়। Rust শুধু
আমাদের API ব্যবহার সহজই করেনি, compile time-এ পুরো এক শ্রেণির error-ও দূর করে দিয়েছে!

#### string literal হলো slice {#string-literals-are-slices}

মনে আছে, আমরা বলেছিলাম string literal binary-র ভেতরে রাখা থাকে। এখন
slice সম্পর্কে জানার পর আমরা string literal ঠিকমতো বুঝতে পারি:

```rust
let s = "Hello, world!";
```

এখানে `s`-এর type হলো `&str`: এটি binary-র সেই নির্দিষ্ট জায়গাকে নির্দেশ করা
একটি slice। এ কারণেই string literal immutable; `&str` একটি
immutable reference।

#### parameter হিসেবে string slice {#string-slices-as-parameters}

literal আর `String` মানের slice নেওয়া যায় জানলে `first_word`-এ আরও একটি উন্নতি
করা যায়, আর তা হলো এর signature:

```rust,ignore
fn first_word(s: &String) -> &str {
```

আরও অভিজ্ঞ একজন Rustacean বরং Listing 4-9-এ দেখানো signature লিখবেন,
কারণ এতে একই function `&String` মান আর `&str` মান দুটোর ওপরই ব্যবহার করা যায়।

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

<span class="caption">Listing 4-9: `s` parameter-এর type হিসেবে string slice
ব্যবহার করে `first_word` function উন্নত করা</span>

আমাদের কাছে একটি string slice থাকলে সেটি সরাসরি পাঠাতে পারি। একটি `String` থাকলে
`String`-এর একটি slice বা `String`-এর একটি reference পাঠাতে পারি। এই
নমনীয়তা *deref coercion* (স্বয়ংক্রিয় dereference-রূপান্তর)-এর সুবিধা নেয়, যে বৈশিষ্ট্যটি আমরা অধ্যায় 15-এর
[“Implicit Deref Coercions with Functions and
Methods”][deref-coercions]<!--ignore--> অংশে আলোচনা করব।

`String`-এর reference-এর বদলে string slice নেওয়ার মতো করে function সংজ্ঞায়িত করলে
কোনো কার্যকারিতা না হারিয়েই আমাদের API আরও সাধারণ ও কাজের হয়:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

### অন্যান্য slice {#other-slices}

আপনি যেমন ভাবছেন, string slice শুধু string-এর জন্য। কিন্তু আরও একটি সাধারণ
slice type আছে। এই array-টি দেখুন:

```rust
let a = [1, 2, 3, 4, 5];
```

যেমন আমরা একটি string-এর অংশ reference করতে চাইতে পারি, তেমনি একটি array-রও অংশ
reference করতে চাইতে পারি। সেটা আমরা এভাবে করব:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

এই slice-এর type হলো `&[i32]`। এটি string slice-এর মতোই কাজ করে, অর্থাৎ
প্রথম উপাদানের একটি reference আর একটি দৈর্ঘ্য রেখে। অন্যান্য নানা ধরনের collection-এর
জন্য আপনি এই ধরনের slice ব্যবহার করবেন। এই collection-গুলো নিয়ে আমরা অধ্যায় 8-এ
vector নিয়ে কথা বলার সময় বিস্তারিত আলোচনা করব।

{{#quiz ../quizzes/ch04-04-slices.toml}}

## সারসংক্ষেপ {#summary}

slice হলো এক বিশেষ ধরনের reference, যা string বা vector-এর মতো কোনো ক্রমের উপ-পরিসরকে 
নির্দেশ করে। runtime-এ একটি slice প্রকাশ করা হয় একটি "fat pointer" (metadata-সহ pointer) হিসেবে, যাতে থাকে
পরিসরের শুরুর একটি pointer আর পরিসরের দৈর্ঘ্য। index-ভিত্তিক পরিসরের তুলনায় slice-এর
একটি সুবিধা হলো, ব্যবহারে থাকা অবস্থায় slice অবৈধ হয়ে যেতে পারে না।

[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#patterns-that-bind-to-values
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[deref-coercions]: ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
