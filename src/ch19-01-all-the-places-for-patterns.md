## যেসব জায়গায় pattern ব্যবহার করা যায় {#all-the-places-patterns-can-be-used}

Rust-এ অনেক জায়গায় pattern দেখা যায়, আর না বুঝেই আপনি এগুলো প্রচুর ব্যবহার করে আসছেন! এই অংশে সেই সব জায়গা নিয়ে আলোচনা করা হয়েছে, যেখানে pattern বৈধ।

### `match` arm {#match-arms}

অধ্যায় 6-এ যেমন আলোচনা করা হয়েছে, আমরা `match` expression-এর arm-এ pattern ব্যবহার করি। আনুষ্ঠানিকভাবে `match` expression সংজ্ঞায়িত হয় `match` keyword, মেলানোর জন্য একটি মান, আর এক বা
একাধিক match arm দিয়ে, যার প্রতিটি একটি pattern আর মানটি সেই arm-এর pattern-এর সঙ্গে মিললে চালানোর একটি expression নিয়ে গঠিত, এভাবে:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre><code>match <em>VALUE</em> {
    <em>PATTERN</em> => <em>EXPRESSION</em>,
    <em>PATTERN</em> => <em>EXPRESSION</em>,
    <em>PATTERN</em> => <em>EXPRESSION</em>,
}</code></pre>

যেমন, এখানে Listing 6-5-এর `match` expression দেওয়া হলো, যা `x` variable-এর একটি
`Option<i32>` মানের ওপর match করে:

```rust,ignore
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

এই `match` expression-এর pattern হলো প্রতিটি তীরচিহ্নের বাম দিকের `None` আর `Some(i)`।

`match` expression-এর একটি শর্ত হলো এদের exhaustive (সব সম্ভাবনা ঢেকে রাখা) হতে হবে, অর্থাৎ `match` expression-এর মানের সব সম্ভাবনা বিবেচনায় আনতে হবে। সব সম্ভাবনা ঢেকেছেন তা নিশ্চিত করার
একটি উপায় হলো শেষ arm-এর জন্য একটি সব-ধরা (catch-all) pattern রাখা: যেমন, যেকোনো মানের সঙ্গে মেলে এমন একটি variable নাম কখনো ব্যর্থ হতে পারে না এবং তাই বাকি সব ক্ষেত্র ঢেকে দেয়।

বিশেষ pattern `_` যেকোনো কিছুর সঙ্গে মেলে, কিন্তু কখনো কোনো variable-এর সঙ্গে bind হয় না, তাই প্রায়ই শেষ match arm-এ এটি ব্যবহার করা হয়। যেমন, উল্লেখ করা হয়নি এমন যেকোনো মান উপেক্ষা করতে
চাইলে `_` pattern কাজে লাগতে পারে। `_` pattern নিয়ে এই অধ্যায়ের পরে [“pattern-এ মান উপেক্ষা
করা”][ignoring-values-in-a-pattern]<!-- ignore -->-এ আরও বিস্তারিত দেখব।

### `let` statement {#let-statements}

এই অধ্যায়ের আগে আমরা শুধু `match` আর `if let`-এর সঙ্গে pattern ব্যবহার নিয়ে স্পষ্টভাবে আলোচনা করেছিলাম, কিন্তু আসলে আমরা অন্য জায়গাতেও pattern ব্যবহার করেছি, `let` statement-সহ। যেমন, `let`
দিয়ে এই সোজাসাপ্টা variable assignment-টি দেখুন:

```rust
let x = 5;
```

এভাবে যখনই `let` statement ব্যবহার করেছেন, তখনই আপনি pattern ব্যবহার করেছেন, যদিও হয়তো বোঝেননি! আরও আনুষ্ঠানিকভাবে, একটি `let` statement দেখতে এমন:

<!--
  Manually formatted rather than using Markdown intentionally: Markdown does not
  support italicizing code in the body of a block like this!
-->

<pre>
<code>let <em>PATTERN</em> = <em>EXPRESSION</em>;</code>
</pre>

`let x = 5;`-এর মতো statement-এ, যেখানে PATTERN-এর জায়গায় একটি variable নাম আছে, variable নামটি pattern-এর একটি বিশেষ সরল রূপ মাত্র। Rust expression-টিকে pattern-এর সঙ্গে তুলনা করে এবং যে নামগুলো
পায় সেগুলোতে মান বসায়। তাই `let x = 5;` উদাহরণে `x` এমন একটি pattern, যার মানে “এখানে যা মেলে তা `x` variable-এর সঙ্গে bind করো।” `x` নামটিই যেহেতু পুরো pattern, তাই এই pattern কার্যত বোঝায়
“মান যা-ই হোক, সবকিছু `x` variable-এর সঙ্গে bind করো।”

`let`-এর pattern-matching দিকটি আরও পরিষ্কারভাবে দেখতে Listing
19-1 দেখুন, যা একটি tuple destructure করতে `let`-এর সঙ্গে একটি pattern ব্যবহার করে।


<Listing number="19-1" caption="একটি tuple destructure করে একসঙ্গে তিনটি variable তৈরি করতে একটি pattern ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-01/src/main.rs:here}}
```

</Listing>

এখানে আমরা একটি tuple-কে একটি pattern-এর সঙ্গে মেলাই। Rust `(1, 2, 3)` মানটিকে `(x, y, z)` pattern-এর সঙ্গে তুলনা করে দেখে যে মানটি pattern-এর সঙ্গে মেলে—অর্থাৎ দেখে যে দুটিতেই উপাদানের সংখ্যা
একই—তাই Rust `1`-কে `x`-এর সঙ্গে, `2`-কে `y`-এর সঙ্গে আর `3`-কে `z`-এর সঙ্গে bind করে। এই tuple pattern-কে এর ভেতরে তিনটি আলাদা variable pattern রাখা হিসেবে ভাবতে পারেন।

pattern-এর উপাদানের সংখ্যা tuple-এর উপাদানের সংখ্যার সঙ্গে না মিললে সামগ্রিক type মিলবে না এবং আমরা compiler error পাব। যেমন, Listing 19-2-এ তিন উপাদানের একটি tuple-কে দুটি variable-এ destructure
করার একটি চেষ্টা দেখানো হয়েছে, যা কাজ করবে না।

<Listing number="19-2" caption="ভুলভাবে এমন একটি pattern তৈরি করা, যার variable-গুলো tuple-এর উপাদানের সংখ্যার সঙ্গে মেলে না">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-02/src/main.rs:here}}
```

</Listing>

এই কোড compile করার চেষ্টা করলে এই type error আসে:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-02/output.txt}}
```

error ঠিক করতে আমরা `_` বা `..` ব্যবহার করে tuple-এর এক বা একাধিক মান উপেক্ষা করতে পারি, যেমনটি [“pattern-এ মান উপেক্ষা
করা”][ignoring-values-in-a-pattern]<!-- ignore --> অংশে দেখবেন। সমস্যা যদি হয় pattern-এ খুব বেশি variable থাকা, তাহলে সমাধান হলো variable সরিয়ে type মেলানো, যাতে variable-এর সংখ্যা tuple-এর
উপাদানের সংখ্যার সমান হয়।

### শর্তসাপেক্ষ `if let` expression {#conditional-if-let-expressions}

অধ্যায় 6-এ আমরা আলোচনা করেছিলাম `if let` expression মূলত কীভাবে এমন একটি `match`-এর সমতুল্য সংক্ষেপে লেখার উপায় হিসেবে ব্যবহার করা যায়, যা শুধু একটি ক্ষেত্রের সঙ্গে মেলে। ঐচ্ছিকভাবে
`if let`-এর একটি সংশ্লিষ্ট `else` থাকতে পারে, যাতে `if let`-এর pattern না মিললে চালানোর কোড থাকে।

Listing 19-3 দেখায় যে `if let`, `else
if` আর `else if let` expression মিলিয়ে ব্যবহারও সম্ভব। এতে এমন একটি `match` expression-এর চেয়ে বেশি নমনীয়তা পাই, যেখানে pattern-গুলোর সঙ্গে তুলনার জন্য শুধু একটি মান প্রকাশ করা যায়। তা ছাড়া
Rust চায় না যে `if
let`, `else if` আর `else if let` arm-এর ধারার শর্তগুলো একে অপরের সঙ্গে সম্পর্কিত হোক।

Listing 19-3-এর কোড কয়েকটি শর্তের ধারাবাহিক যাচাইয়ের ভিত্তিতে ঠিক করে আপনার পটভূমি কোন রঙের হবে। এই উদাহরণে আমরা এমন hardcoded (কোডে সরাসরি লেখা) মানসহ variable তৈরি করেছি, যা একটি
বাস্তব প্রোগ্রাম হয়তো ব্যবহারকারীর input থেকে পেত।

<Listing number="19-3" file-name="src/main.rs" caption="`if let`, `else if`, `else if let` আর `else` মিলিয়ে ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-03/src/main.rs}}
```

</Listing>

ব্যবহারকারী একটি প্রিয় রং উল্লেখ করলে সেই রংই পটভূমি হিসেবে ব্যবহার হয়। কোনো প্রিয় রং উল্লেখ না থাকলে এবং আজ মঙ্গলবার হলে পটভূমির রং সবুজ। নইলে, ব্যবহারকারী যদি তার বয়স একটি string হিসেবে
উল্লেখ করে এবং আমরা সফলভাবে তা একটি সংখ্যা হিসেবে parse করতে পারি, তাহলে সংখ্যাটির মান অনুযায়ী রং হয় বেগুনি অথবা কমলা। এর কোনো শর্তই প্রযোজ্য না হলে পটভূমির রং নীল।

এই শর্তসাপেক্ষ গঠন আমাদের জটিল চাহিদা সমর্থন করতে দেয়। এখানকার hardcoded মানগুলো দিয়ে এই উদাহরণ print করবে `Using purple as the
background color`।

দেখতে পাচ্ছেন, `match` arm যেভাবে পারে, `if let`-ও সেভাবে নতুন variable আনতে পারে যা বিদ্যমান variable-কে shadow করে: `if let Ok(age) = age` লাইনটি একটি নতুন `age` variable আনে, যাতে `Ok`
variant-এর ভেতরের মান থাকে, আর এটি বিদ্যমান `age` variable-কে shadow করে। এর মানে `if age >
30` শর্তটি সেই block-এর ভেতরে রাখতে হবে: এই দুটি শর্ত মিলিয়ে `if
let Ok(age) = age && age > 30` লেখা যায় না। যে নতুন `age`-কে 30-এর সঙ্গে তুলনা করতে চাই, তা curly bracket দিয়ে নতুন scope শুরু না হওয়া পর্যন্ত বৈধ নয়।

`if let` expression ব্যবহারের অসুবিধা হলো compiler exhaustiveness (সব সম্ভাবনা ঢাকা হয়েছে কি না) যাচাই করে না, যেখানে `match` expression-এর ক্ষেত্রে করে। শেষ `else` block বাদ দিলে এবং ফলে কিছু
ক্ষেত্র সামলাতে বাদ পড়লে compiler সম্ভাব্য logic বাগ সম্পর্কে আমাদের সতর্ক করত না।

### `while let` শর্তসাপেক্ষ loop {#while-let-conditional-loops}

গঠনে `if let`-এর মতো, `while let` শর্তসাপেক্ষ loop একটি `while` loop-কে ততক্ষণ চলতে দেয়, যতক্ষণ একটি pattern মিলতে থাকে। Listing
19-4-এ আমরা একটি `while let` loop দেখাই, যা thread-এর মধ্যে পাঠানো message-এর জন্য অপেক্ষা করে, তবে এই ক্ষেত্রে একটি `Option`-এর বদলে একটি `Result` যাচাই করে।

<Listing number="19-4" caption="`rx.recv()` যতক্ষণ `Ok` return করে, ততক্ষণ মান print করতে একটি `while let` loop ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-04/src/main.rs:here}}
```

</Listing>

এই উদাহরণ print করে `1`, `2`, তারপর `3`। `recv` method channel-এর গ্রহণের দিক থেকে প্রথম message বের করে নিয়ে একটি `Ok(value)` return করে। অধ্যায় 16-এ যখন প্রথম `recv` দেখেছিলাম, তখন হয় আমরা
সরাসরি error unwrap করেছিলাম, নয়তো একটি `for` loop দিয়ে একে iterator হিসেবে ব্যবহার করেছিলাম। তবে Listing 19-4 যেমন দেখায়, `while let`-ও ব্যবহার করা যায়, কারণ sender যতক্ষণ আছে, প্রতিবার একটি
message এলে `recv` method একটি `Ok` return করে, তারপর sender-এর দিক বিচ্ছিন্ন হলে একটি `Err` তৈরি করে।

### `for` loop {#for-loops}

একটি `for` loop-এ `for` keyword-এর ঠিক পরের মানটি একটি pattern। যেমন, `for x in y`-এ `x` হলো pattern। Listing 19-5 দেখায় `for` loop-এর অংশ হিসেবে একটি tuple destructure করতে, বা ভেঙে নিতে,
`for` loop-এ কীভাবে pattern ব্যবহার করতে হয়।


<Listing number="19-5" caption="একটি tuple destructure করতে `for` loop-এ একটি pattern ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-05/src/main.rs:here}}
```

</Listing>

Listing 19-5-এর কোড এই output print করবে:


```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-05/output.txt}}
```

আমরা `enumerate` method ব্যবহার করে একটি iterator-কে রূপান্তর করি, যাতে এটি একটি মান আর সেই মানের index একটি tuple-এ রেখে তৈরি করে। তৈরি হওয়া প্রথম মান হলো tuple `(0, 'a')`। এই মান `(index,
value)` pattern-এর সঙ্গে মেলালে index হবে `0` আর value হবে `'a'`, যা output-এর প্রথম লাইন print করে।


### function-এর parameter {#function-parameters}

function-এর parameter-ও pattern হতে পারে। Listing 19-6-এর কোড, যা `foo` নামের একটি function ঘোষণা করে যা
`i32` type-এর `x` নামের একটি parameter নেয়, এতক্ষণে পরিচিত লাগার কথা।

<Listing number="19-6" caption="parameter-এ pattern ব্যবহার করা একটি function signature">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-06/src/main.rs:here}}
```

</Listing>

`x` অংশটিই একটি pattern! `let`-এর মতোই একটি function-এর argument-এ একটি tuple-কে pattern-এর সঙ্গে মেলাতে পারি। Listing 19-7 একটি function-এ tuple পাঠানোর সময় এর মানগুলো ভাগ করে নেয়।

<Listing number="19-7" file-name="src/main.rs" caption="একটি tuple destructure করে এমন parameter-যুক্ত একটি function">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-07/src/main.rs}}
```

</Listing>

এই কোড print করে `Current location: (3, 5)`। `&(3, 5)` মানগুলো `&(x, y)` pattern-এর সঙ্গে মেলে, তাই `x` হলো মান `3` আর `y` হলো মান `5`।

closure-এর parameter তালিকাতেও function-এর parameter তালিকার মতোই pattern ব্যবহার করা যায়, কারণ অধ্যায় 13-এ যেমন আলোচনা করা হয়েছে, closure function-এর মতোই।

এই পর্যায়ে আপনি pattern ব্যবহারের কয়েকটি উপায় দেখেছেন, তবে যেসব জায়গায় pattern ব্যবহার করা যায় তার প্রতিটিতে এরা একইভাবে কাজ করে না। কিছু জায়গায় pattern-কে irrefutable (ব্যর্থ হতে পারে না
এমন) হতে হয়; অন্য পরিস্থিতিতে এরা refutable (ব্যর্থ হতে পারে এমন) হতে পারে। এরপর আমরা এই দুটি ধারণা নিয়ে আলোচনা করব।

{{#quiz ../quizzes/ch18-01-all-the-places-for-patterns.toml}}

[ignoring-values-in-a-pattern]: ch19-03-pattern-syntax.html#ignoring-values-in-a-pattern
