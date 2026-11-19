## refutability: একটি pattern মেলাতে ব্যর্থ হতে পারে কি না {#refutability-whether-a-pattern-might-fail-to-match}

pattern দুই রকম: refutable আর irrefutable। পাঠানো যেকোনো সম্ভাব্য মানের সঙ্গে মিলবে এমন pattern হলো _irrefutable_ (ব্যর্থ হতে পারে না এমন)। একটি উদাহরণ হলো `let x = 5;` statement-এর `x`, কারণ
`x` যেকোনো কিছুর সঙ্গে মেলে এবং তাই মেলাতে ব্যর্থ হতে পারে না। কোনো কোনো সম্ভাব্য মানের ক্ষেত্রে মেলাতে ব্যর্থ হতে পারে এমন pattern হলো _refutable_ (ব্যর্থ হতে পারে এমন)। কয়েকটি উদাহরণ:

<!-- BEGIN INTERVENTION: 3c29eb2d-cbe9-4a2c-99b8-aa5c6467c8b4 -->
* `if let Some(x) = a_value` expression-এ `Some(x)` refutable। `a_value` variable-এর মান যদি `Some`-এর বদলে
`None` হয়, তাহলে `Some(x)` pattern মিলবে না। 
* `if let &[x, ..] = a_slice` expression-এ `&[x, ..]` refutable। `a_slice` variable-এর মানে যদি একটিও উপাদান না থাকে, তাহলে `&[x, ..]` pattern মিলবে না।
<!-- END INTERVENTION: 3c29eb2d-cbe9-4a2c-99b8-aa5c6467c8b4 -->

function parameter, `let` statement আর `for` loop শুধু irrefutable pattern গ্রহণ করতে পারে, কারণ মান না মিললে প্রোগ্রাম অর্থবহ কিছু করতে পারে না। `if let` আর `while let` expression এবং
`let...else` statement refutable আর irrefutable দুই ধরনের pattern-ই গ্রহণ করে, কিন্তু compiler irrefutable pattern-এর বিরুদ্ধে সতর্ক করে, কারণ সংজ্ঞা অনুযায়ী এগুলো সম্ভাব্য ব্যর্থতা সামলানোর
জন্যই: একটি শর্তসাপেক্ষ গঠনের কার্যকারিতা হলো সাফল্য বা ব্যর্থতার ওপর নির্ভর করে ভিন্নভাবে কাজ করতে পারা।

সাধারণত refutable আর irrefutable pattern-এর পার্থক্য নিয়ে আপনাকে চিন্তা করতে হবে না; তবে refutability-র ধারণার সঙ্গে পরিচিত থাকতে হবে, যাতে error message-এ এটি দেখলে সাড়া দিতে পারেন। সেসব
ক্ষেত্রে কোডের উদ্দেশ্যিত আচরণ অনুযায়ী হয় pattern, নয়তো যে গঠনের সঙ্গে pattern ব্যবহার করছেন তা বদলাতে হবে।

চলুন একটি উদাহরণ দেখি, যেখানে Rust irrefutable pattern চায় সেখানে refutable pattern ব্যবহারের চেষ্টা করলে, এবং উল্টোটা করলে কী ঘটে। Listing 19-8-এ একটি `let` statement দেখানো হয়েছে, কিন্তু
pattern হিসেবে আমরা দিয়েছি `Some(x)`, একটি refutable pattern। আশা করতেই পারেন, এই কোড compile হবে না।

<Listing number="19-8" caption="`let`-এর সঙ্গে একটি refutable pattern ব্যবহারের চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-08/src/main.rs:here}}
```

</Listing>

`some_option_value` যদি একটি `None` মান হতো, তাহলে এটি `Some(x)` pattern-এর সঙ্গে মেলাতে ব্যর্থ হতো, অর্থাৎ pattern-টি refutable। কিন্তু `let` statement শুধু একটি irrefutable pattern গ্রহণ করতে পারে,
কারণ একটি `None` মান নিয়ে কোড বৈধ কিছুই করতে পারে না। compile time-এ Rust আপত্তি জানাবে যে irrefutable pattern দরকার এমন জায়গায় আমরা একটি refutable pattern ব্যবহারের চেষ্টা করেছি:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-08/output.txt}}
```

`Some(x)` pattern দিয়ে আমরা প্রতিটি বৈধ মান ঢাকিনি (এবং ঢাকতে পারতামও না!), তাই Rust ন্যায্যভাবেই একটি compiler error দেয়।

irrefutable pattern দরকার এমন জায়গায় refutable pattern থাকলে pattern ব্যবহার করা কোড বদলে তা ঠিক করা যায়: `let`-এর বদলে `let...else` ব্যবহার করতে পারি। তখন pattern না মিললে curly
bracket-এর ভেতরের কোড মানটি সামলাবে। Listing 19-9-এ দেখানো হয়েছে Listing 19-8-এর কোড কীভাবে ঠিক করা যায়।

<Listing number="19-9" caption="`let`-এর বদলে refutable pattern-সহ `let...else` আর একটি block ব্যবহার করা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-09/src/main.rs:here}}
```

</Listing>

আমরা কোডকে বেরোনোর একটি পথ দিয়েছি! এই কোড পুরোপুরি বৈধ, যদিও এর মানে হলো warning না পেয়ে irrefutable pattern ব্যবহার করা যায় না। Listing
19-10-এ দেখানোর মতো `let...else`-কে যদি এমন একটি pattern দিই যা সবসময় মিলবে, যেমন `x`, তাহলে compiler একটি warning দেবে।

<Listing number="19-10" caption="`let...else`-এর সঙ্গে একটি irrefutable pattern ব্যবহারের চেষ্টা">

```rust
{{#rustdoc_include ../listings/ch19-patterns-and-matching/listing-19-10/src/main.rs:here}}
```

</Listing>

Rust আপত্তি জানায় যে irrefutable pattern-এর সঙ্গে `let...else` ব্যবহার অর্থহীন:

```console
{{#include ../listings/ch19-patterns-and-matching/listing-19-10/output.txt}}
```

এ কারণে match arm-গুলোকে refutable pattern ব্যবহার করতে হয়, শেষ arm ছাড়া, যার একটি irrefutable pattern দিয়ে বাকি সব মানের সঙ্গে মেলা উচিত। Rust শুধু একটি arm-যুক্ত `match`-এ একটি irrefutable
pattern ব্যবহার করতে দেয়, কিন্তু এই syntax খুব একটা কাজের নয় এবং একটি সরলতর `let` statement দিয়ে বদলে ফেলা যায়।

কোথায় pattern ব্যবহার করতে হয় এবং refutable আর irrefutable pattern-এর পার্থক্য জানার পর চলুন pattern তৈরিতে ব্যবহারযোগ্য সব syntax দেখি।

{{#quiz ../quizzes/ch18-02-refutability.toml}}