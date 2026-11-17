## `if let` আর `let...else` দিয়ে সংক্ষিপ্ত control flow {#concise-control-flow-with-if-let-and-letelse}

`if let` syntax দিয়ে `if` আর `let` মিলিয়ে কম কথায় এমন মান সামলানো যায়, যা একটি pattern
(মানের গঠনের নকশা)-এর সঙ্গে মেলে, আর বাকিগুলো উপেক্ষা করা যায়। Listing 6-6-এর প্রোগ্রামটি
দেখুন, যা `config_max` variable-এর একটি `Option<u8>` মানের ওপর match করে, কিন্তু শুধু মানটি
`Some` variant হলেই কোড চালাতে চায়।

<Listing number="6-6" caption="একটি `match`, যা শুধু মান `Some` হলেই কোড চালাতে আগ্রহী">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

</Listing>

মানটি `Some` হলে pattern-এ মানটিকে `max` variable-এর সঙ্গে bind (কোনো নামের সঙ্গে মান জুড়ে
দেওয়া) করে আমরা `Some` variant-এর ভেতরের মানটি প্রিন্ট করি। `None` মান নিয়ে আমরা কিছুই
করতে চাই না। `match` expression-কে সন্তুষ্ট করতে শুধু একটি variant সামলানোর পর আমাদের `_ =>
()` যোগ করতে হয়, যা যোগ করা বিরক্তিকর boilerplate (বারবার লিখতে হয় এমন গৎবাঁধা কোড) কোড।

এর বদলে `if let` ব্যবহার করে আমরা এটি আরও ছোট করে লিখতে পারি। নিচের কোডটি Listing 6-6-এর
`match`-এর মতোই আচরণ করে:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

`if let` syntax একটি সমান চিহ্ন দিয়ে আলাদা করা একটি pattern আর একটি expression নেয়। এটি
একটি `match`-এর মতোই কাজ করে, যেখানে expression-টি `match`-কে দেওয়া হয় আর pattern-টি হলো
এর প্রথম arm। এই ক্ষেত্রে pattern হলো `Some(max)`, আর `max` `Some`-এর ভেতরের মানের সঙ্গে
bind হয়। তারপর সংশ্লিষ্ট `match` arm-এ যেভাবে `max` ব্যবহার করেছিলাম, সেভাবেই `if let`
block-এর body-তে `max` ব্যবহার করতে পারি। `if let` block-এর কোড শুধু তখনই চলে, যখন মানটি
pattern-এর সঙ্গে মেলে।

`if let` ব্যবহার করা মানে কম টাইপ করা, কম ইনডেন্ট, আর কম boilerplate কোড। তবে `match`
যে exhaustive (সব সম্ভাবনা সামলানো) যাচাই বলবৎ করে, যা নিশ্চিত করে আপনি কোনো ক্ষেত্র সামলাতে
ভুলে যাচ্ছেন না, সেটি হারান। `match` আর `if
let`-এর মধ্যে কোনটি বেছে নেবেন, তা নির্ভর করে আপনার নির্দিষ্ট পরিস্থিতিতে আপনি কী করছেন, আর
exhaustive যাচাই হারিয়ে সংক্ষিপ্ততা পাওয়াটা উপযুক্ত বিনিময় কি না তার ওপর।

অন্যভাবে বললে, `if let`-কে আপনি এমন একটি `match`-এর syntax sugar হিসেবে ভাবতে পারেন, যা
মানটি একটি pattern-এর সঙ্গে মিললে কোড চালায় এবং বাকি সব মান উপেক্ষা করে।

`if let`-এর সঙ্গে আমরা একটি `else` রাখতে পারি। `else`-এর সঙ্গের কোডের block হলো সেই কোডের
block, যা `if let` আর `else`-এর সমতুল্য `match` expression-এ `_` ক্ষেত্রটির সঙ্গে থাকত।
Listing 6-4-এর `Coin` enum-এর সংজ্ঞা মনে করুন, যেখানে `Quarter` variant একটি `UsState` মানও
রাখত। আমরা যদি quarter-গুলোর অঙ্গরাজ্য ঘোষণা করার পাশাপাশি quarter নয় এমন সব মুদ্রা গুনতে
চাইতাম, তাহলে একটি `match` expression দিয়ে তা করতে পারতাম, এভাবে:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

অথবা একটি `if let` আর `else` expression ব্যবহার করতে পারতাম, এভাবে:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

## `let...else` দিয়ে “happy path”-এ থাকা {#staying-on-the-happy-path-with-letelse}

প্রচলিত ধরনটি হলো, একটি মান উপস্থিত থাকলে কিছু হিসাব-নিকাশ করা, আর অন্যথায় একটি ডিফল্ট মান
return করা। `UsState` মানসহ মুদ্রার উদাহরণটি চালিয়ে গেলে, quarter-এর অঙ্গরাজ্যটি কত পুরোনো
তার ওপর নির্ভর করে আমরা যদি মজার কিছু বলতে চাইতাম, তাহলে হয়তো একটি অঙ্গরাজ্যের বয়স যাচাই
করতে `UsState`-এর ওপর একটি method যোগ করতাম, এভাবে:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

তারপর মুদ্রার ধরনের ওপর match করতে আমরা হয়তো `if let` ব্যবহার করতাম, শর্তের body-র ভেতরে
একটি `state` variable এনে, Listing 6-7-এর মতো।

<Listing number="6-7" caption="একটি `if let`-এর ভেতরে রাখা শর্তমূলক কোড ব্যবহার করে যাচাই করা যে একটি অঙ্গরাজ্য 1900 সালে ছিল কি না">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:describe}}
```

</Listing>

এতে কাজ হয়ে যায়, কিন্তু কাজটি `if
let` statement-এর body-র ভেতরে ঢুকে গেছে, আর করণীয় কাজ আরও জটিল হলে ওপরের স্তরের শাখাগুলো
ঠিক কীভাবে সম্পর্কিত, তা অনুসরণ করা কঠিন হতে পারে। expression যে একটি মান তৈরি করে, এই
বাস্তবতার সুবিধা নিয়ে আমরা হয় `if let` থেকে `state` তৈরি করতে পারি, নয়তো আগেভাগে return করতে
পারি, Listing 6-8-এর মতো। (একটি `match` দিয়েও আপনি একই রকম কিছু করতে পারতেন।)

<Listing number="6-8" caption="একটি মান তৈরি করতে বা আগেভাগে return করতে `if let` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

</Listing>

তবে নিজের মতো করে এটিও অনুসরণ করা একটু বিরক্তিকর! `if
let`-এর একটি শাখা একটি মান তৈরি করে, আর অন্যটি function থেকে পুরোপুরি return করে।

এই প্রচলিত ধরনটি আরও সুন্দরভাবে প্রকাশ করতে Rust-এ আছে `let...else`।
`let...else` syntax বাঁ পাশে একটি pattern আর ডান পাশে একটি expression নেয়, অনেকটা `if let`-এর
মতো, কিন্তু এর কোনো `if` শাখা নেই, শুধু একটি `else` শাখা আছে। pattern মিললে এটি pattern থেকে
মানটিকে বাইরের scope-এ bind করবে। pattern _না_ মিললে প্রোগ্রাম `else` arm-এ চলে যাবে, যাকে
অবশ্যই function থেকে return করতে হবে।

Listing 6-9-এ আপনি দেখতে পাবেন, `if let`-এর বদলে `let...else` ব্যবহার করলে Listing 6-8 দেখতে
কেমন হয়।

<Listing number="6-9" caption="function-এর ভেতর দিয়ে চলার পথ স্পষ্ট করতে `let...else` ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

</Listing>

লক্ষ করুন, এভাবে এটি function-এর মূল body-তে “happy path” (সব ঠিকঠাক থাকলে কোড যে পথে চলে)-এ
থাকে, `if let`-এর মতো দুটি শাখার জন্য উল্লেখযোগ্যভাবে আলাদা control flow ছাড়াই।

আপনার প্রোগ্রামে যদি এমন যুক্তি থাকে, যা একটি `match` দিয়ে প্রকাশ করলে খুব লম্বা-চওড়া হয়ে
যায়, তাহলে মনে রাখবেন, `if let` আর `let...else`-ও আপনার Rust হাতিয়ারের বাক্সে আছে।

{{#quiz ../quizzes/ch06-03-if-let.toml}}

## সারসংক্ষেপ {#summary}

এবার আমরা দেখলাম, কীভাবে enum ব্যবহার করে এমন নিজস্ব type তৈরি করা যায়, যা তালিকাভুক্ত
কয়েকটি মানের যেকোনো একটি হতে পারে। দেখিয়েছি, standard library-র `Option<T>` type কীভাবে
error আটকাতে type system (ভাষার type-সংক্রান্ত নিয়মের ব্যবস্থা) ব্যবহারে আপনাকে সাহায্য করে।
enum মানের ভেতরে ডেটা থাকলে, কয়টি ক্ষেত্র সামলাতে হবে তার ওপর নির্ভর করে, সেই মানগুলো বের করে
ব্যবহার করতে আপনি `match` বা `if let` ব্যবহার করতে পারেন।

আপনার Rust প্রোগ্রাম এখন struct আর enum ব্যবহার করে আপনার বিষয়ক্ষেত্রের ধারণা প্রকাশ করতে
পারে। আপনার API-তে ব্যবহারের জন্য নিজস্ব type তৈরি করলে type safety (type-এর নিয়ম মানার
নিশ্চয়তা) নিশ্চিত হয়: compiler নিশ্চিত করবে যে আপনার function-গুলো শুধু সেই type-এর মানই
পায়, যা প্রতিটি function আশা করে।

আপনার ব্যবহারকারীদের সুসংগঠিত, সহজে ব্যবহারযোগ্য এবং ঠিক যতটুকু দরকার শুধু ততটুকুই প্রকাশ
করে এমন একটি API দিতে চলুন এবার Rust-এর module-এর দিকে যাই।
