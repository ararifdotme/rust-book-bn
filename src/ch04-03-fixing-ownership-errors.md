## ownership error ঠিক করা {#fixing-ownership-errors}

ownership error কীভাবে ঠিক করতে হয় তা শেখা Rust-এর একটি মূল দক্ষতা। borrow checker যখন আপনার কোড বাতিল করে দেয়, তখন আপনার কী করা উচিত? এই অংশে আমরা ownership-এর প্রচলিত error-এর কয়েকটি উদাহরণ বিশ্লেষণ করব। প্রতিটি উদাহরণে compiler-এর বাতিল করা একটি function দেখানো হবে। তারপর আমরা ব্যাখ্যা করব Rust কেন function-টি বাতিল করে, আর সেটি ঠিক করার কয়েকটি উপায় দেখাব।

একটি বিষয় বারবার আসবে: একটি function *আসলেই* নিরাপদ নাকি অনিরাপদ, তা বোঝা। Rust সবসময় অনিরাপদ প্রোগ্রাম বাতিল করবে[^safe-subset]। কিন্তু কখনো কখনো Rust নিরাপদ প্রোগ্রামও বাতিল করবে। এই উদাহরণগুলো দেখাবে দুই পরিস্থিতিতেই error-এর জবাব কীভাবে দিতে হয়।

<!-- The last two sections have shown how a Rust program can be **unsafe** if it triggers undefined behavior. The ownership guarantee is that Rust will reject all unsafe programs. However, Rust will also reject *some* safe programs. Fixing an ownership error will depend on whether your program is *actually* safe or unsafe. -->

### একটি অনিরাপদ প্রোগ্রাম ঠিক করা: stack-এর reference return করা {#fixing-an-unsafe-program-returning-a-reference-to-the-stack}

আমাদের প্রথম উদাহরণটি stack-এর reference return করা নিয়ে, ঠিক যেমনটি আগের অংশের ["ডেটাকে তার সব reference-এর চেয়ে বেশিদিন টিকে থাকতে হবে"](ch04-02-references-and-borrowing.html#data-must-outlive-all-of-its-references)-এ আলোচনা করেছি। এই সেই function, যা আমরা দেখেছিলাম:

```rust,ignore,does_not_compile
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
```

এই function কীভাবে ঠিক করা যায় তা ভাবতে গিয়ে আমাদের প্রশ্ন করতে হবে: **এই প্রোগ্রামটি অনিরাপদ কেন?** এখানে সমস্যা হলো যে ডেটার reference দেওয়া হচ্ছে, তার lifetime (reference কতক্ষণ বৈধ থাকে)। একটি string-এর reference এদিক-ওদিক পাঠাতে চাইলে আপনাকে নিশ্চিত করতে হবে যে মূল string-টি যথেষ্ট সময় টিকে থাকে। 

পরিস্থিতি অনুযায়ী string-টির lifetime বাড়ানোর চারটি উপায় এখানে দেওয়া হলো। একটি হলো string-টির ownership function থেকে বাইরে move করা, অর্থাৎ `&String`-কে `String`-এ বদলে দেওয়া:

```rust
fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}
```

আরেকটি সম্ভাবনা হলো একটি string literal (উদ্ধৃতিচিহ্নের ভেতরে সরাসরি লেখা string) return করা, যা চিরকাল টিকে থাকে (`'static` দিয়ে বোঝানো হয়)। আমরা যদি কখনো string-টি বদলাতে না চাই, তাহলে এই সমাধান খাটে, আর তখন heap-এ বরাদ্দের দরকারই নেই:

```rust
fn return_a_string() -> &'static str {
    "Hello world"    
}
```

আরেকটি সম্ভাবনা হলো garbage collection ব্যবহার করে borrow-যাচাইকে runtime-এ পিছিয়ে দেওয়া। যেমন, আপনি একটি [reference-counted pointer][rc] ব্যবহার করতে পারেন:

```rust
use std::rc::Rc;
fn return_a_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}
```

reference counting (কতগুলো reference আছে গুনে মেমোরি ফেরতের ব্যবস্থা) নিয়ে আমরা অধ্যায় 15.4-এর ["`Rc<T>`, the Reference Counted Smart Pointer"](ch15-04-rc.html)-এ আরও আলোচনা করব। সংক্ষেপে, `Rc::clone` শুধু `s`-এর একটি pointer clone করে, ডেটাটি নয়। runtime-এ `Rc` যাচাই করে কখন ডেটাকে নির্দেশ করা শেষ `Rc`-টি drop হয়েছে, তারপর ডেটা deallocate করে।

আরও একটি সম্ভাবনা হলো caller-কে দিয়ে একটি mutable reference-এর মাধ্যমে string রাখার একটি "ঘর" দেওয়ানো:

```rust
fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}
```

এই কৌশলে string-এর জন্য জায়গা তৈরির দায়িত্ব caller-এর। এই ধরনটি একটু লম্বা-চওড়া হতে পারে, তবে caller-কে যদি কখন বরাদ্দ হবে তা সতর্কভাবে নিয়ন্ত্রণ করতে হয়, তাহলে এটি মেমোরির দিক থেকে বেশি দক্ষও হতে পারে।

কোন কৌশলটি সবচেয়ে উপযুক্ত, তা নির্ভর করবে আপনার অ্যাপ্লিকেশনের ওপর। তবে মূল কথা হলো ওপর-ওপর দেখা ownership error-এর পেছনের আসল সমস্যাটি চিনে নেওয়া। আমার string-টি কতক্ষণ টিকে থাকা উচিত? এটি deallocate করার দায়িত্ব কার হওয়া উচিত? এই প্রশ্নগুলোর পরিষ্কার উত্তর পেয়ে গেলে বাকি কাজ শুধু সেই অনুযায়ী আপনার API বদলানো।


### একটি অনিরাপদ প্রোগ্রাম ঠিক করা: যথেষ্ট permission নেই {#fixing-an-unsafe-program-not-enough-permissions}

আরেকটি প্রচলিত সমস্যা হলো শুধু-পড়ার ডেটা mutate করার চেষ্টা, অথবা reference-এর পেছনে থাকা ডেটা drop করার চেষ্টা। যেমন, ধরুন আমরা `stringify_name_with_title` নামে একটি function লেখার চেষ্টা করলাম। এই function-এর কাজ নামের অংশগুলোর একটি vector থেকে একজন মানুষের পুরো নাম তৈরি করা, সঙ্গে একটি বাড়তি পদবিও যোগ করা।

```aquascope,permissions,stepper,boundaries,shouldFail
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));`{}`
    let full = name.join(" ");
    full
}

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
```

borrow checker এই প্রোগ্রামটি বাতিল করে, কারণ `name` একটি immutable reference, কিন্তু `name.push(..)`-এর জন্য @Perm{write} permission দরকার। প্রোগ্রামটি অনিরাপদ, কারণ `push` `stringify_name_with_title`-এর বাইরে থাকা `name`-এর অন্য reference-গুলোকে অবৈধ করে দিতে পারে, এভাবে:

```aquascope,interpreter,shouldFail,horizontal
#fn stringify_name_with_title(name: &Vec<String>) -> String {
#    name.push(String::from("Esq."));
#    let full = name.join(" ");
#    full
#}
fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];`[]`
    stringify_name_with_title(&name);`[]`
    println!("{}", first);`[]`
}
```

এই উদাহরণে `stringify_name_with_title` call করার আগে `name[0]`-এর একটি reference `first` তৈরি করা হয়। `name.push(..)` function `name`-এর বিষয়বস্তু নতুন করে বরাদ্দ করে, যা `first`-কে অবৈধ করে দেয়, ফলে `println` deallocate হওয়া মেমোরি পড়ে।

তাহলে এই API কীভাবে ঠিক করব? একটি সরল সমাধান হলো `name`-এর type `&Vec<String>` থেকে বদলে `&mut Vec<String>` করা:

```rust,ignore
fn stringify_name_with_title(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
```

কিন্তু এটি ভালো সমাধান নয়! **caller আশা না করলে function-এর তার ইনপুট mutate করা উচিত নয়।** যিনি `stringify_name_with_title` call করছেন, তিনি সম্ভবত আশা করেন না যে এই function তার vector বদলে দেবে। `add_title_to_name`-এর মতো অন্য কোনো function-এর কাছ থেকে হয়তো ইনপুট mutate করার আশা করা যায়, কিন্তু আমাদের function-এর কাছ থেকে নয়।

আরেকটি বিকল্প হলো `&Vec<String>`-কে `Vec<String>`-এ বদলে নামটির ownership নিয়ে নেওয়া:

```rust,ignore
fn stringify_name_with_title(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}
```

কিন্তু এটিও ভালো সমাধান নয়! **Rust function-এর পক্ষে `Vec` আর `String`-এর মতো heap-এর মালিক data structure-এর ownership নেওয়া খুবই বিরল।**  `stringify_name_with_title`-এর এই সংস্করণটি ইনপুট `name`-কে ব্যবহারের অযোগ্য করে দেবে, যা caller-এর জন্য খুবই বিরক্তিকর, যেমনটি আমরা ["reference ও borrowing"](ch04-02-references-and-borrowing.html)-এর শুরুতে আলোচনা করেছি।

তাহলে `&Vec` বেছে নেওয়াটা আসলে ভালো সিদ্ধান্তই, যা আমরা বদলাতে চাই *না*। বরং আমরা function-এর body বদলাতে পারি। সম্ভাব্য অনেক সমাধান আছে, যেগুলো কতটা মেমোরি ব্যবহার করে তাতে পার্থক্য আছে। একটি সম্ভাবনা হলো ইনপুট `name` clone করা:

```rust,ignore
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}
```

`name` clone করলে vector-এর লোকাল কপিটি আমরা mutate করতে পারি। তবে clone ইনপুটের প্রতিটি string কপি করে। পদবিটি পরে যোগ করে আমরা অপ্রয়োজনীয় কপি এড়াতে পারি:

```rust,ignore
fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}
```

এই সমাধান কাজ করে, কারণ [`slice::join`] আগেই `name`-এর ডেটা `full` string-এ কপি করে।

সাধারণভাবে, Rust function লেখা মানে *সঠিক* মাত্রার permission চাওয়ার একটি সতর্ক ভারসাম্য। এই উদাহরণে `name`-এর ওপর শুধু read permission আশা করাই সবচেয়ে Rust-সুলভ (idiomatic)।

{{#quiz ../quizzes/ch04-03-fixing-ownership-errors-sec1-idioms.toml}}

### একটি অনিরাপদ প্রোগ্রাম ঠিক করা: একটি data structure-কে alias ও mutate করা {#fixing-an-unsafe-program-aliasing-and-mutating-a-data-structure}

আরেকটি অনিরাপদ operation হলো heap ডেটার এমন একটি reference ব্যবহার করা, যে ডেটা অন্য একটি alias deallocate করে দেয়। যেমন, এখানে একটি function আছে, যা একটি vector-এর সবচেয়ে বড় string-এর একটি reference নেয়, তারপর vector-টি mutate করার সময় সেটি ব্যবহার করে:

```aquascope,permissions,stepper,boundaries,shouldFail
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {`(focus,paths:*dst)`
    let largest: &String = 
      dst.iter().max_by_key(|s| s.len()).unwrap();`(focus,paths:*dst)`
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());`{}`
        }
    }
}
```

> *দ্রষ্টব্য:* সবচেয়ে বড় string-এর reference সংক্ষেপে খুঁজে বের করতে এই উদাহরণে [iterator][iterators] (একটির পর একটি উপাদান দেয় এমন বস্তু) আর [closure][closures] (পারিপার্শ্বিক মান ধরে রাখতে পারে এমন নামহীন function) ব্যবহার করা হয়েছে। এই বৈশিষ্ট্যগুলো নিয়ে আমরা পরের অধ্যায়গুলোতে আলোচনা করব, আপাতত এখানে এগুলো কীভাবে কাজ করে তার একটি সহজ ধারণা দেব।

borrow checker এই প্রোগ্রামটি বাতিল করে, কারণ `let largest = ..` `dst`-এর ওপর থেকে @Perm{write} permission সরিয়ে নেয়। কিন্তু `dst.push(..)`-এর জন্য @Perm{write} permission দরকার। আবারও আমাদের প্রশ্ন করা উচিত: **এই প্রোগ্রামটি অনিরাপদ কেন?** কারণ `dst.push(..)` `dst`-এর বিষয়বস্তু deallocate করে দিতে পারে, ফলে reference `largest` অবৈধ হয়ে যায়।

প্রোগ্রামটি ঠিক করার মূল কথা হলো, `largest`-এর lifetime এমনভাবে ছোট করতে হবে, যাতে তা `dst.push(..)`-এর সঙ্গে না মেলে। একটি সম্ভাবনা হলো `largest` clone করা:

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}
```

তবে string-এর ডেটা বরাদ্দ ও কপি করার কারণে এতে পারফরম্যান্সে কিছুটা আঘাত লাগতে পারে।

আরেকটি সম্ভাবনা হলো আগে দৈর্ঘ্যের সব তুলনা সেরে নেওয়া, তারপর `dst` mutate করা:

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = 
        src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
}
```

তবে `to_add` vector বরাদ্দের কারণে এতেও পারফরম্যান্সে আঘাত লাগে।

শেষ সম্ভাবনাটি হলো `largest`-এর দৈর্ঘ্য কপি করে বের করে নেওয়া, কারণ আমাদের আসলে `largest`-এর বিষয়বস্তু দরকার নেই, শুধু দৈর্ঘ্যটি দরকার। 
এই সমাধানটিকে সম্ভবত সবচেয়ে Rust-সুলভ এবং সবচেয়ে দক্ষ বলা যায়:

```rust
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
```

এই সব সমাধানের মূল ভাবনা একই: `dst`-এর ওপর borrow-গুলোর lifetime এমনভাবে ছোট করা, যাতে তা `dst`-এর mutation-এর সঙ্গে না মেলে।

### একটি অনিরাপদ প্রোগ্রাম ঠিক করা: collection থেকে কপি বনাম move করা {#fixing-an-unsafe-program-copying-vs-moving-out-of-a-collection}

Rust শিক্ষার্থীদের একটি প্রচলিত বিভ্রান্তি ঘটে vector-এর মতো কোনো collection থেকে ডেটা কপি করে বের করার সময়। যেমন, এখানে একটি নিরাপদ প্রোগ্রাম আছে, যা একটি vector থেকে একটি সংখ্যা কপি করে বের করে:

```aquascope,permissions,stepper,boundaries
#fn main() {
let v: Vec<i32> = vec![0, 1, 2];
let n_ref: &i32 = &v[0];`(focus,paths:*n_ref)`
let n: i32 = *n_ref;`{}`
#}
```

dereference operation `*n_ref` শুধু @Perm{read} permission আশা করে, যা path `*n_ref`-এর আছে। কিন্তু vector-এর উপাদানের type `i32` থেকে বদলে `String` করলে কী হয়? তখন দেখা যায়, আমাদের আর প্রয়োজনীয় permission নেই:

```aquascope,permissions,stepper,boundaries,shouldFail
#fn main() {
let v: Vec<String> = 
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];`(focus,paths:*s_ref)`
let s: String = *s_ref;`[]``{}`
#}
```

প্রথম প্রোগ্রামটি compile হবে, কিন্তু দ্বিতীয়টি হবে না। Rust নিচের error message দেয়:

```text
error[E0507]: cannot move out of `*s_ref` which is behind a shared reference
 --> test.rs:4:9
  |
4 | let s = *s_ref;
  |         ^^^^^^
  |         |
  |         move occurs because `*s_ref` has type `String`, which does not implement the `Copy` trait
```

সমস্যা হলো `v` vector-টি "Hello world" string-এর owner। আমরা যখন `s_ref` dereference করি, তখন তা vector-এর কাছ থেকে string-টির ownership নেওয়ার চেষ্টা করে। কিন্তু reference হলো non-owning pointer &mdash; reference-এর *মাধ্যমে* আমরা ownership নিতে পারি না। তাই Rust আপত্তি জানায় যে আমরা "cannot move out of \[...\] a shared reference"।

কিন্তু এটা অনিরাপদ কেন? বাতিল হওয়া প্রোগ্রামটি অনুকরণ করে সমস্যাটি দেখানো যায়:

```aquascope,interpreter,shouldFail,horizontal
#fn main() {
let v: Vec<String> = 
  vec![String::from("Hello world")];
let s_ref: &String = &v[0];`(focus,paths:*s_ref)`
let s: String = *s_ref;`[]``{}`

// These drops are normally implicit, but we've added them for clarity.
drop(s);`[]`
drop(v);`[]`
#}
```

এখানে যা ঘটে, তা হলো একটি **double-free** (একই মেমোরি দুবার free করা)। `let s = *s_ref` চালানোর পর `v` আর `s` দুজনেই মনে করে তারা "Hello world"-এর owner। `s` drop হওয়ার পর "Hello world" deallocate হয়। তারপর `v` drop হয়, আর string-টি দ্বিতীয়বার free হওয়ার সময় undefined behavior ঘটে।

> *দ্রষ্টব্য:* `s = *s_ref` চালানোর পর double-free-এর মাধ্যমে undefined behavior ঘটাতে আমাদের `v` বা `s` ব্যবহার করারও দরকার নেই। `s_ref` থেকে string-টি move করে বের করামাত্রই, উপাদানগুলো drop হলে undefined behavior ঘটবে।

তবে vector-এ `i32` উপাদান থাকলে এই undefined behavior ঘটে না। পার্থক্য হলো, একটি `String` কপি করলে heap ডেটার একটি pointer কপি হয়। একটি `i32` কপি করলে তা হয় না।
প্রযুক্তিগত ভাষায়, Rust বলে `i32` type `Copy` trait (টাইপের সাধারণ আচরণের সংজ্ঞা) implement করে, কিন্তু `String` `Copy` implement করে না (trait নিয়ে আমরা পরের একটি অধ্যায়ে আলোচনা করব)।

সংক্ষেপে, **কোনো মান যদি heap ডেটার owner না হয়, তাহলে move ছাড়াই সেটি কপি করা যায়।** যেমন:

* একটি `i32` heap ডেটার owner **নয়**, তাই move ছাড়াই এটি কপি করা **যায়**। 
* একটি `String` heap ডেটার owner, তাই move ছাড়া এটি কপি করা **যায় না**।
* একটি `&String` heap ডেটার owner **নয়**, তাই move ছাড়াই এটি কপি করা **যায়**।

> *দ্রষ্টব্য:* এই নিয়মের একটি ব্যতিক্রম হলো mutable reference। যেমন, `&mut i32` একটি copyable (move ছাড়াই কপি করা যায় এমন) type নয়। তাই আপনি যদি এরকম কিছু করেন:
> ```rust,ignore
> let mut n = 0;
> let a = &mut n;
> let b = a;
> ```
> তাহলে `b`-তে assign করার পর `a` আর ব্যবহার করা যাবে না। এতে একই ডেটার দুটি mutable reference একই সময়ে ব্যবহার হওয়া আটকায়।

তাহলে আমাদের কাছে যদি `String`-এর মতো non-`Copy` type-এর একটি vector থাকে, তাহলে vector-এর কোনো উপাদানে নিরাপদে কীভাবে পৌঁছাব? এখানে তা নিরাপদে করার কয়েকটি আলাদা উপায় দেওয়া হলো। প্রথমত, string-টির ownership না নিয়ে শুধু একটি immutable reference ব্যবহার করতে পারেন:

```rust,ignore
# fn main() {
let v: Vec<String> = vec![String::from("Hello world")];
let s_ref: &String = &v[0];
println!("{s_ref}!");
# }
```

দ্বিতীয়ত, vector-টিকে অক্ষত রেখে string-টির ownership পেতে চাইলে ডেটা clone করতে পারেন:

```rust,ignore
# fn main() {
let v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v[0].clone();
s.push('!');
println!("{s}");
# }
```

সবশেষে, [`Vec::remove`]-এর মতো একটি method ব্যবহার করে string-টিকে vector থেকে move করে বের করে আনতে পারেন:

```rust,ignore
# fn main() {
let mut v: Vec<String> = vec![String::from("Hello world")];
let mut s: String = v.remove(0);
s.push('!');
println!("{s}");
assert!(v.len() == 0);
# }
```


### একটি নিরাপদ প্রোগ্রাম ঠিক করা: tuple-এর আলাদা field mutate করা {#fixing-a-safe-program-mutating-different-tuple-fields}

ওপরের উদাহরণগুলোতে প্রোগ্রাম অনিরাপদ ছিল। Rust নিরাপদ প্রোগ্রামও বাতিল করতে পারে। একটি প্রচলিত সমস্যা হলো, Rust খুব সূক্ষ্ম মাত্রায় permission-এর হিসাব রাখার চেষ্টা করে। তবে Rust দুটি আলাদা place-কে একই place বলে গুলিয়ে ফেলতে পারে। 
 
প্রথমে সূক্ষ্ম মাত্রায় permission হিসাবের এমন একটি উদাহরণ দেখা যাক, যা borrow checker পেরিয়ে যায়। এই প্রোগ্রাম দেখায় কীভাবে একটি tuple-এর একটি field borrow করে একই tuple-এর অন্য একটি field-এ লেখা যায়:

```aquascope,permissions,stepper,boundaries
#fn main() {
let mut name = (
    String::from("Ferris"), 
    String::from("Rustacean")
);`(focus,paths:name)`
let first = &name.0;`(focus,paths:name)`
name.1.push_str(", Esq.");`{}`
println!("{first} {}", name.1);
#}
```

`let first = &name.0` statement `name.0`-কে borrow করে। এই borrow `name.0` থেকে @Perm{write}@Perm{own} permission সরিয়ে নেয়। এটি `name` থেকেও @Perm{write}@Perm{own} permission সরিয়ে নেয়। (যেমন, `(String, String)` type-এর মান ইনপুট হিসেবে নেয় এমন কোনো function-এ `name` পাঠানো যাবে না।) কিন্তু `name.1`-এর @Perm{write} permission এখনো আছে, তাই `name.1.push_str(...)` করা একটি বৈধ operation।

তবে কোন place-গুলো ঠিক borrow হয়েছে, Rust তার হিসাব হারিয়ে ফেলতে পারে। যেমন, ধরুন আমরা `&name.0` expression-টিকে refactor করে `get_first` নামের একটি function-এ নিয়ে গেলাম। লক্ষ করুন, `get_first(&name)` call করার পর Rust এখন `name.1`-এর ওপর থেকে @Perm{write} permission সরিয়ে নেয়:

```aquascope,permissions,stepper,boundaries,shouldFail
fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn main() {
    let mut name = (
        String::from("Ferris"), 
        String::from("Rustacean")
    );
    let first = get_first(&name);`(focus,paths:name)`
    name.1.push_str(", Esq.");`{}`
    println!("{first} {}", name.1);
}
```

এখন আমরা `name.1.push_str(..)` করতে পারি না! Rust এই error দেবে:

```text
error[E0502]: cannot borrow `name.1` as mutable because it is also borrowed as immutable
  --> test.rs:11:5
   |
10 |     let first = get_first(&name);
   |                           ----- immutable borrow occurs here
11 |     name.1.push_str(", Esq.");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
12 |     println!("{first} {}", name.1);
   |                ----- immutable borrow later used here
```

অদ্ভুত ব্যাপার, কারণ সম্পাদনার আগে প্রোগ্রামটি নিরাপদ ছিল। আমাদের করা সম্পাদনা runtime-এর আচরণে উল্লেখযোগ্য কোনো পরিবর্তন আনে না। তাহলে `&name.0`-কে একটি function-এ রাখায় কী এসে যায়?

সমস্যা হলো, `get_first(&name)` কী borrow করবে তা ঠিক করার সময় Rust `get_first`-এর implementation দেখে না। Rust শুধু type signature দেখে, যা কেবল বলে "ইনপুটের কোনো একটি `String` borrow হয়"। তাই Rust সাবধানতা অবলম্বন করে ধরে নেয় যে `name.0` আর `name.1` দুটোই borrow হয়েছে, এবং দুটো থেকেই write আর own permission সরিয়ে নেয়। 

মনে রাখবেন, মূল কথা হলো **ওপরের প্রোগ্রামটি নিরাপদ।** এতে কোনো undefined behavior নেই! Rust-এর কোনো ভবিষ্যৎ সংস্করণ হয়তো এটিকে compile হতে দেওয়ার মতো বুদ্ধিমান হবে, কিন্তু আজকের দিনে এটি বাতিল হয়। তাহলে আজকে borrow checker-কে কীভাবে পাশ কাটাব? একটি সম্ভাবনা হলো মূল প্রোগ্রামের মতো `&name.0` expression-টি সরাসরি সেখানেই লিখে দেওয়া (inline করা)। আরেকটি সম্ভাবনা হলো [cell][cells] ব্যবহার করে borrow-যাচাই runtime-এ পিছিয়ে দেওয়া, যা নিয়ে আমরা ভবিষ্যতের অধ্যায়গুলোতে আলোচনা করব।

### একটি নিরাপদ প্রোগ্রাম ঠিক করা: array-র আলাদা উপাদান mutate করা {#fixing-a-safe-program-mutating-different-array-elements}

array-র উপাদান borrow করার সময়ও একই ধরনের সমস্যা দেখা দেয়। যেমন, একটি array-র mutable reference নিলে কোন place-গুলো borrow হয় তা লক্ষ করুন:

```aquascope,permissions,stepper,boundaries
#fn main() {
let mut a = [0, 1, 2, 3];
let x = &mut a[1];`(focus,paths:a[_])`
*x += 1;`(focus,paths:a[_])`
println!("{a:?}");
#}
```

Rust-এর borrow checker-এ `a[0]`, `a[1]` ইত্যাদির জন্য আলাদা আলাদা place নেই। এটি একটিমাত্র place `a[_]` ব্যবহার করে, যা `a`-এর *সব* index বোঝায়। Rust এটা করে, কারণ index-এর মান সবসময় নির্ধারণ করা সম্ভব হয় না। যেমন, এরকম একটি জটিলতর পরিস্থিতি কল্পনা করুন:

```rust,ignore
let idx = a_complex_function();
let x = &mut a[idx];
```

`idx`-এর মান কী? Rust অনুমান করতে যাবে না, তাই ধরে নেয় `idx` যা খুশি হতে পারে। যেমন, ধরুন আমরা array-র একটি index থেকে পড়ার সময় অন্য একটি index-এ লেখার চেষ্টা করলাম:

```aquascope,permissions,boundaries,stepper,shouldFail
#fn main() {
let mut a = [0, 1, 2, 3];
let x = &mut a[1];`(focus,paths:a[_])`
let y = &a[2];`{}`
*x += *y;
#}
```

তবে Rust এই প্রোগ্রামটি বাতিল করবে, কারণ `a` তার read permission `x`-কে দিয়ে দিয়েছে। compiler-এর error message-ও একই কথা বলে:

```text
error[E0502]: cannot borrow `a[_]` as immutable because it is also borrowed as mutable
 --> test.rs:4:9
  |
3 | let x = &mut a[1];
  |         --------- mutable borrow occurs here
4 | let y = &a[2];
  |         ^^^^^ immutable borrow occurs here
5 | *x += *y;
  | -------- mutable borrow later used here
```

<!-- However, Rust will reject this program because `a` gave its read permission to `x`. -->


আবারও, **এই প্রোগ্রামটি নিরাপদ।** এ ধরনের ক্ষেত্রে Rust প্রায়ই standard library (আদর্শ লাইব্রেরি)-তে এমন একটি function দেয়, যা borrow checker-কে পাশ কাটাতে পারে। যেমন, আমরা [`slice::split_at_mut`][split_at_mut] ব্যবহার করতে পারি:

```rust,ignore
# fn main() {
let mut a = [0, 1, 2, 3];
let (a_l, a_r) = a.split_at_mut(2);
let x = &mut a_l[1];
let y = &a_r[0];
*x += *y;
# }
```

আপনি হয়তো ভাবছেন, `split_at_mut` তাহলে implement করা হয়েছে কীভাবে? কিছু Rust library-তে, বিশেষ করে `Vec` বা `slice`-এর মতো মূল type-গুলোতে, প্রায়ই **`unsafe` block** (নিরাপত্তা-যাচাই শিথিল করা কোড-অংশ) দেখতে পাবেন। `unsafe` block "raw" pointer (borrow checker যাচাই করে না এমন pointer) ব্যবহারের অনুমতি দেয়, যেগুলোর নিরাপত্তা borrow checker যাচাই করে না। যেমন, আমাদের কাজটি করতে আমরা একটি unsafe block ব্যবহার করতে পারতাম:

```rust,ignore
# fn main() {
let mut a = [0, 1, 2, 3];
let x = &mut a[1] as *mut i32;
let y = &a[2] as *const i32;
unsafe { *x += *y; } // কী করছেন তা ঠিকঠাক না জানলে এটি করবেন না!
# }
```

borrow checker-এর সীমাবদ্ধতা পাশ কাটাতে কখনো কখনো unsafe কোড দরকার হয়। সাধারণ কৌশল হিসেবে ধরুন, borrow checker এমন একটি প্রোগ্রাম বাতিল করছে, যা আপনার মতে আসলে নিরাপদ। তখন এমন standard library function (যেমন `split_at_mut`) খুঁজে দেখুন, যেগুলোর ভেতরে `unsafe` block আছে এবং যা আপনার সমস্যার সমাধান করে। unsafe কোড নিয়ে আমরা [অধ্যায় 20][unsafe]-এ আরও আলোচনা করব। আপাতত শুধু জেনে রাখুন, অন্যথায় অসম্ভব এমন কিছু ধরন Rust unsafe কোড দিয়েই implement করে।

{{#quiz ../quizzes/ch04-03-fixing-ownership-errors-sec2-safety.toml}}

### সারসংক্ষেপ {#summary}

ownership error ঠিক করার সময় নিজেকে প্রশ্ন করুন: আমার প্রোগ্রাম কি আসলেই অনিরাপদ? যদি হ্যাঁ হয়, তাহলে অনিরাপদ হওয়ার মূল কারণটি বুঝতে হবে। যদি না হয়, তাহলে borrow checker-এর সীমাবদ্ধতাগুলো বুঝে সেগুলো পাশ কাটাতে হবে।

[rc]: https://doc.rust-lang.org/std/rc/index.html
[cells]: https://doc.rust-lang.org/std/cell/index.html
[split_at_mut]: https://doc.rust-lang.org/std/primitive.slice.html#method.split_at_mut
[unsafe]: ch19-01-unsafe-rust.html
[`Vec::remove`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove
[`slice::join`]: https://doc.rust-lang.org/std/primitive.slice.html#method.join
[iterators]: ch13-02-iterators.html
[closures]: ch13-01-closures.html

[^safe-subset]: এই নিশ্চয়তা Rust-এর "নিরাপদ অংশ" (safe subset)-এ লেখা প্রোগ্রামের জন্য প্রযোজ্য। আপনি যদি `unsafe` কোড ব্যবহার করেন বা অনিরাপদ উপাদান call করেন (যেমন কোনো C library call করা), তাহলে undefined behavior এড়াতে আপনাকে বাড়তি সতর্ক থাকতে হবে।
