## variable ও mutability {#variables-and-mutability}

[“variable-এ মান
রাখা”][storing-values-with-variables]<!-- ignore --> অংশে যেমন বলা হয়েছে,
ডিফল্টভাবে variable immutable (অপরিবর্তনীয়)। Rust আপনাকে যেসব ছোট ছোট উপায়ে
এমনভাবে কোড লিখতে উৎসাহ দেয়, যাতে Rust-এর দেওয়া নিরাপত্তা আর সহজ concurrency
(একসঙ্গে একাধিক কাজ সামলানো)-র সুবিধা নেওয়া যায়, এটি তার একটি। তবু আপনার
variable-কে mutable (পরিবর্তনযোগ্য) করার সুযোগও আছে। চলুন দেখি, Rust কীভাবে ও কেন
আপনাকে immutability পছন্দ করতে উৎসাহ দেয়, আর কেন কখনো কখনো আপনি এর বাইরে যেতে
চাইতে পারেন।

কোনো variable immutable হলে, একবার একটি নামের সঙ্গে একটি মান bind (কোনো নামের
সঙ্গে মান জুড়ে দেওয়া) হয়ে গেলে সেই মান আর বদলানো যায় না। এটি দেখাতে আপনার
_projects_ ডিরেক্টরিতে `cargo new variables` দিয়ে _variables_ নামে একটি নতুন
প্রজেক্ট তৈরি করুন।

তারপর নতুন _variables_ ডিরেক্টরিতে _src/main.rs_ খুলে এর কোড নিচের কোড দিয়ে
বদলে দিন; এই কোড এখনই compile (সংকলন) হবে না:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

ফাইলটি সেভ করে `cargo run` দিয়ে প্রোগ্রামটি চালান। নিচের আউটপুটের মতো একটি
immutability-সংক্রান্ত error message পাওয়ার কথা:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

এই উদাহরণটি দেখায়, compiler (সংকলক) কীভাবে আপনার প্রোগ্রামের error (ত্রুটি) খুঁজে
পেতে সাহায্য করে। compiler error বিরক্তিকর লাগতে পারে, কিন্তু আসলে এর মানে শুধু
এটুকু যে আপনার প্রোগ্রাম এখনো নিরাপদভাবে আপনার চাওয়া কাজটি করছে না; এর মানে
মোটেও _এই নয়_ যে আপনি ভালো প্রোগ্রামার নন! অভিজ্ঞ Rustacean (Rust প্রোগ্রামার)-রাও
compiler error পান।

আপনি `` cannot assign twice to immutable variable `x` `` error message পেয়েছেন, কারণ আপনি immutable `x` variable-এ দ্বিতীয় একটি মান assign (মান বসানো) করার চেষ্টা করেছেন।

immutable হিসেবে চিহ্নিত কোনো মান বদলানোর চেষ্টা করলে compile time (সংকলনের
সময়)-এ error পাওয়া জরুরি, কারণ ঠিক এই পরিস্থিতি থেকেই বাগ তৈরি হতে পারে। আমাদের
কোডের এক অংশ যদি ধরে নিয়ে কাজ করে যে একটি মান কখনো বদলাবে না, আর কোডের অন্য
অংশ সেই মান বদলে দেয়, তাহলে কোডের প্রথম অংশটি যে কাজের জন্য লেখা, তা না-ও করতে
পারে। এ ধরনের বাগের কারণ পরে খুঁজে বের করা কঠিন হতে পারে, বিশেষ করে যখন কোডের
দ্বিতীয় অংশটি মানটি শুধু _মাঝে মাঝে_ বদলায়। Rust compiler নিশ্চয়তা দেয় যে আপনি
যখন বলছেন একটি মান বদলাবে না, তখন সেটি সত্যিই বদলাবে না, তাই আপনাকে নিজে এর খোঁজ
রাখতে হয় না। ফলে আপনার কোড বুঝে নেওয়া সহজ হয়।

তবে mutability (পরিবর্তনযোগ্যতা) খুব কাজের হতে পারে এবং কোড লেখা আরও সুবিধাজনক
করতে পারে। variable ডিফল্টভাবে immutable হলেও [অধ্যায়
2][storing-values-with-variables]<!-- ignore -->-এ যেমন করেছিলেন, variable-এর
নামের আগে `mut` যোগ করে সেটিকে mutable করতে পারেন। `mut` যোগ করলে ভবিষ্যতে যারা
কোডটি পড়বেন, তারাও উদ্দেশ্যটা বুঝতে পারেন: কোডের অন্য অংশ এই variable-এর মান
বদলাবে।

যেমন, _src/main.rs_ বদলে এরকম করি:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

এবার প্রোগ্রামটি চালালে আমরা পাই:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

`mut` ব্যবহার করায় `x`-এর সঙ্গে bind হওয়া মান `5` থেকে `6`-এ বদলানোর অনুমতি পাই।
শেষ পর্যন্ত mutability ব্যবহার করবেন কি না, সেই সিদ্ধান্ত আপনার, আর তা নির্ভর করে
নির্দিষ্ট পরিস্থিতিতে আপনার কাছে কোনটি সবচেয়ে স্পষ্ট মনে হয় তার ওপর।

{{#quiz ../quizzes/ch03-01-variables-and-mutability-sec1-variables.toml}}

<!-- Old headings. Do not remove or links may break. -->
<a id="constants"></a>

### constant ঘোষণা করা {#declaring-constants}

immutable variable-এর মতোই _constant_ (ধ্রুবক, কখনো বদলায় না এমন মান)-ও এমন মান,
যা একটি নামের সঙ্গে bind থাকে এবং যার পরিবর্তনের অনুমতি নেই; তবে constant আর
variable-এর মধ্যে কয়েকটি পার্থক্য আছে।

প্রথমত, constant-এর সঙ্গে `mut` ব্যবহার করা যায় না। constant শুধু ডিফল্টভাবে
immutable নয়—এরা সবসময়ই immutable। constant ঘোষণা করতে `let` keyword-এর বদলে
`const` keyword ব্যবহার করতে হয়, আর মানটির type _অবশ্যই_ annotate (type স্পষ্ট করে
লিখে দেওয়া) করতে হয়। type আর type annotation নিয়ে আমরা পরের অংশ
[“data type”][data-types]<!-- ignore -->-এ আলোচনা করব, তাই এখনই খুঁটিনাটি নিয়ে
চিন্তা করবেন না। শুধু জেনে রাখুন, type সবসময় annotate করতে হবে।

constant যেকোনো scope (কোডের যে অংশে একটি নাম ব্যবহার করা যায়)-এ ঘোষণা করা যায়,
global scope (পুরো প্রোগ্রামজুড়ে দৃশ্যমান scope)-সহ; তাই কোডের অনেক অংশের জানা
দরকার এমন মানের জন্য constant কাজের।

শেষ পার্থক্যটি হলো, constant-এ শুধু একটি constant expression (compile-এর সময়েই
মান নির্ণয় করা যায় এমন expression) বসানো যায়, এমন কোনো মানের ফলাফল নয়, যা শুধু
runtime (প্রোগ্রাম চলার সময়)-এ হিসাব করা সম্ভব।

constant ঘোষণার একটি উদাহরণ:

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

constant-টির নাম `THREE_HOURS_IN_SECONDS`, আর এর মান হলো 60 (এক মিনিটে সেকেন্ডের
সংখ্যা) গুণ 60 (এক ঘণ্টায় মিনিটের সংখ্যা) গুণ 3 (এই প্রোগ্রামে আমরা যত ঘণ্টা গুনতে
চাই)-এর ফলাফল। Rust-এ constant-এর নামকরণের রীতি হলো সব অক্ষর বড় হাতের, আর শব্দের
মাঝে আন্ডারস্কোর। compiler compile time-এ সীমিত কিছু operation evaluate (মান
নির্ণয়) করতে পারে; এর ফলে এই constant-এর মান সরাসরি 10,800 না লিখে এমনভাবে লেখা
যায়, যা বোঝা আর যাচাই করা সহজ। constant ঘোষণার সময় কোন কোন operation ব্যবহার করা
যায়, তা জানতে দেখুন [Rust Reference-এর constant evaluation অংশ][const-eval]।

constant যে scope-এ ঘোষণা করা হয়, সেই scope-এর ভেতরে প্রোগ্রাম যতক্ষণ চলে, পুরো
সময়টাই সেটি বৈধ থাকে। এই বৈশিষ্ট্যের কারণে আপনার অ্যাপ্লিকেশনের বিষয়বস্তুর এমন
মানের জন্য constant কাজের, যা প্রোগ্রামের একাধিক অংশের জানা দরকার হতে পারে, যেমন
কোনো খেলায় একজন খেলোয়াড় সর্বোচ্চ কত পয়েন্ট পেতে পারেন, কিংবা আলোর গতি।

প্রোগ্রামজুড়ে ব্যবহৃত hardcoded মান (কোডে সরাসরি লিখে দেওয়া মান)-গুলোকে constant
হিসেবে নাম দিলে ভবিষ্যতে যারা কোডটি রক্ষণাবেক্ষণ করবেন, তাদের কাছে মানটির অর্থ
পরিষ্কার হয়। এতে আরেকটি সুবিধা হয়: ভবিষ্যতে hardcoded মানটি বদলানোর দরকার হলে
আপনার কোডের শুধু একটি জায়গায় বদলালেই চলবে।

{{#quiz ../quizzes/ch03-01-variables-and-mutability-sec2-constants.toml}}

### shadowing {#shadowing}

[অধ্যায়
2][comparing-the-guess-to-the-secret-number]<!-- ignore -->-এর সংখ্যা আন্দাজের
খেলার টিউটোরিয়ালে যেমন দেখেছেন, আগের কোনো variable-এর নামেই একটি নতুন variable
ঘোষণা করা যায়। Rustacean-রা বলেন, প্রথম variable-টি দ্বিতীয়টির দ্বারা _shadowed_
হয়েছে, অর্থাৎ variable-এর নামটি ব্যবহার করলে compiler দ্বিতীয় variable-টিকেই দেখবে।
কার্যত দ্বিতীয় variable-টি প্রথমটিকে ঢেকে দেয়, এবং variable-এর নামের সব ব্যবহার
নিজের দিকে টেনে নেয়, যতক্ষণ না সেটি নিজেও shadow হয় অথবা scope শেষ হয়। একই
variable-এর নাম ব্যবহার করে এবং `let` keyword আবার লিখে আমরা একটি variable-কে
shadow (একই নামে নতুন variable দিয়ে আগেরটিকে আড়াল করা) করতে পারি, এভাবে:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

এই প্রোগ্রামটি প্রথমে `x`-কে `5` মানের সঙ্গে bind করে। তারপর `let x =` আবার লিখে
একটি নতুন variable `x` তৈরি করে, যা মূল মানের সঙ্গে `1` যোগ করে, ফলে `x`-এর মান হয়
`6`। এরপর কার্লি ব্র্যাকেট দিয়ে তৈরি একটি ভেতরের scope-এ তৃতীয় `let` statement (কাজ করে কিন্তু মান তৈরি করে না এমন কোড-নির্দেশ)-টিও
`x`-কে shadow করে একটি নতুন variable তৈরি করে, আগের মানকে `2` দিয়ে গুণ করে `x`-কে
`12` মান দেয়। ওই scope শেষ হলে ভেতরের shadowing-ও শেষ হয়, আর `x` আবার `6` হয়ে
যায়। প্রোগ্রামটি চালালে নিচের আউটপুট আসবে:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

shadowing কোনো variable-কে `mut` হিসেবে চিহ্নিত করার থেকে আলাদা, কারণ ভুল করে
`let` keyword ছাড়া এই variable-এ আবার মান assign করার চেষ্টা করলে আমরা compile-time
error পাব। `let` ব্যবহার করে আমরা একটি মানের ওপর কয়েকটি রূপান্তর চালাতে পারি,
কিন্তু রূপান্তরগুলো শেষ হওয়ার পর variable-টি immutable থাকে।

`mut` আর shadowing-এর আরেকটি পার্থক্য হলো, `let` keyword আবার ব্যবহার করলে আমরা
কার্যত একটি নতুন variable তৈরি করি, তাই একই নাম আবার ব্যবহার করেও মানটির type বদলাতে
পারি। যেমন, ধরুন আমাদের প্রোগ্রাম ব্যবহারকারীকে বলে, কিছু লেখার মাঝে তিনি কতগুলো
ফাঁকা জায়গা চান, তা space অক্ষর টাইপ করে দেখাতে, আর তারপর আমরা সেই ইনপুট একটি
সংখ্যা হিসেবে রাখতে চাই:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

প্রথম `spaces` variable-টি একটি string (লেখা) type, আর দ্বিতীয় `spaces` variable-টি একটি
সংখ্যার type। এভাবে shadowing আমাদের `spaces_str` আর `spaces_num`-এর মতো আলাদা
আলাদা নাম ভেবে বের করার ঝামেলা থেকে বাঁচায়; তার বদলে আমরা সহজ `spaces` নামটিই
আবার ব্যবহার করতে পারি। তবে এখানে যেমন দেখানো হয়েছে, এর জন্য `mut` ব্যবহার করার
চেষ্টা করলে compile-time error পাব:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

error বলছে, কোনো variable-এর type বদলানোর অনুমতি নেই:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

variable কীভাবে কাজ করে তা দেখা হলো, এবার চলুন variable-এর আরও কী কী data type
হতে পারে, তা দেখি।

{{#quiz ../quizzes/ch03-01-variables-and-mutability-sec3-shadowing.toml}}

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[data-types]: ch03-02-data-types.html#data-types
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[const-eval]: https://doc.rust-lang.org/reference/const_eval.html
