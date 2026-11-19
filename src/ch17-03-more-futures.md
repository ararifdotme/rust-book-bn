
<!-- Old headings. Do not remove or links may break. -->

<a id="yielding"></a>

### runtime-এর হাতে নিয়ন্ত্রণ ছেড়ে দেওয়া {#yielding-control-to-the-runtime}

[“আমাদের প্রথম async প্রোগ্রাম”][async-program]<!-- ignore -->
অংশ থেকে মনে করুন, প্রতিটি await point-এ, await করা future প্রস্তুত না থাকলে Rust runtime-কে task থামিয়ে অন্য task-এ যাওয়ার সুযোগ দেয়। উল্টোটাও সত্য: Rust _শুধু_ await point-এই async
block থামায় এবং runtime-এর হাতে নিয়ন্ত্রণ ফিরিয়ে দেয়। দুটি await point-এর মাঝের সবকিছু synchronous।

এর মানে, একটি async block-এ await point ছাড়া অনেক কাজ করলে সেই future অন্য সব future-এর অগ্রগতি আটকে দেবে। কখনো কখনো একে বলা হয় একটি future অন্য future-গুলোকে _অভুক্ত রাখছে_
(starving)। কিছু ক্ষেত্রে এটি বড় কোনো ব্যাপার না-ও হতে পারে। তবে আপনি যদি ব্যয়বহুল কোনো প্রস্তুতি বা দীর্ঘ সময় ধরে চলা কাজ করেন, অথবা এমন একটি future থাকে যা অনির্দিষ্টকাল ধরে
কোনো নির্দিষ্ট কাজ করতে থাকবে, তাহলে ভাবতে হবে কখন ও কোথায় runtime-এর হাতে নিয়ন্ত্রণ ফিরিয়ে দেবেন।

অভুক্ত রাখার সমস্যাটি দেখাতে চলুন দীর্ঘ সময় ধরে চলা একটি operation-এর অনুকরণ করি, তারপর দেখি কীভাবে এর সমাধান করা যায়। Listing 17-14-এ একটি `slow` function দেখানো হয়েছে।

<Listing number="17-14" caption="ধীর operation-এর অনুকরণ করতে `thread::sleep` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-14/src/main.rs:slow}}
```

</Listing>

এই কোড `trpl::sleep`-এর বদলে `std::thread::sleep` ব্যবহার করে, যাতে `slow` call করলে বর্তমান thread কয়েক millisecond-এর জন্য block হয়ে থাকে। দীর্ঘ সময় ধরে চলা এবং block করা,
দুটোই এমন বাস্তব operation-এর বদলি হিসেবে `slow` ব্যবহার করতে পারি।

Listing 17-15-এ আমরা এক জোড়া future-এ এই ধরনের CPU-bound কাজের অনুকরণ করতে `slow` ব্যবহার করি।

<Listing number="17-15" caption="ধীর operation-এর অনুকরণ করতে `slow` function call করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-15/src/main.rs:slow-futures}}
```

</Listing>

প্রতিটি future অনেকগুলো ধীর operation সম্পন্ন করার _পরেই_ কেবল runtime-এর হাতে নিয়ন্ত্রণ ফিরিয়ে দেয়। এই কোড চালালে এই output দেখবেন:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-15/
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'a' ran for 10ms
'a' ran for 20ms
'b' started.
'b' ran for 75ms
'b' ran for 10ms
'b' ran for 15ms
'b' ran for 350ms
'a' finished.
```

Listing 17-5-এর মতো, যেখানে দুটি URL নিয়ে আসা future-এর মধ্যে প্রতিযোগিতা করাতে `trpl::select` ব্যবহার করেছিলাম, `a` শেষ হওয়ার সঙ্গে সঙ্গে `select` এখনো শেষ হয়। তবে দুটি future-এর
`slow` call-গুলোর মধ্যে কোনো মিশ্রণ নেই। `a` future তার `trpl::sleep` call-এর জন্য await না করা পর্যন্ত তার সব কাজ করে, তারপর `b` future তার নিজের `trpl::sleep` call-এর জন্য await না
করা পর্যন্ত তার সব কাজ করে, আর শেষে `a` future শেষ হয়। দুটি future-কেই তাদের ধীর কাজের মাঝে মাঝে এগোতে দিতে আমাদের await point দরকার, যাতে runtime-এর হাতে নিয়ন্ত্রণ ফিরিয়ে দিতে
পারি। অর্থাৎ এমন কিছু দরকার যার জন্য await করা যায়!

Listing 17-15-এ এই ধরনের হস্তান্তর ইতিমধ্যে ঘটতে দেখা যাচ্ছে: `a` future-এর শেষের `trpl::sleep` সরিয়ে দিলে `b` future _একটুও_ না চলে এটি শেষ হয়ে যেত। operation-গুলোকে পালা করে
এগোতে দেওয়ার একটি শুরুর বিন্দু হিসেবে চলুন `trpl::sleep` function ব্যবহার করে দেখি, যেমনটি Listing 17-16-এ দেখানো হয়েছে।

<Listing number="17-16" caption="operation-গুলোকে পালা করে এগোতে দিতে `trpl::sleep` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-16/src/main.rs:here}}
```

</Listing>

আমরা `slow`-এর প্রতিটি call-এর মাঝে await point-সহ `trpl::sleep` call যোগ করেছি। এখন দুটি future-এর কাজ মিশ্রিতভাবে চলে:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-16
cargo run
copy just the output
-->

```text
'a' started.
'a' ran for 30ms
'b' started.
'b' ran for 75ms
'a' ran for 10ms
'b' ran for 10ms
'a' ran for 20ms
'b' ran for 15ms
'a' finished.
```

`b`-এর হাতে নিয়ন্ত্রণ দেওয়ার আগে `a` future এখনো কিছুক্ষণ চলে, কারণ এটি `trpl::sleep` call করার আগেই `slow` call করে, তবে তারপর যখনই কোনো একটি await point-এ পৌঁছায়, future-গুলো
পালা বদল করে। এই ক্ষেত্রে আমরা `slow`-এর প্রতিটি call-এর পরে তা করেছি, তবে কাজটি আমাদের কাছে যেভাবে সবচেয়ে অর্থবহ মনে হয় সেভাবে ভাগ করতে পারতাম।

তবে এখানে আমরা আসলে _sleep_ করতে চাই না: আমরা যত দ্রুত সম্ভব এগোতে চাই। আমাদের শুধু runtime-এর হাতে নিয়ন্ত্রণ ফিরিয়ে দিতে হবে। `trpl::yield_now` function ব্যবহার করে সরাসরি তা
করা যায়। Listing 17-17-এ আমরা ওই সব `trpl::sleep` call-এর জায়গায় `trpl::yield_now` বসাই।

<Listing number="17-17" caption="operation-গুলোকে পালা করে এগোতে দিতে `yield_now` ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-17/src/main.rs:yields}}
```

</Listing>

এই কোড আসল উদ্দেশ্য সম্পর্কে আরও পরিষ্কার, আর `sleep` ব্যবহারের চেয়ে উল্লেখযোগ্যভাবে দ্রুতও হতে পারে, কারণ `sleep`-এর ব্যবহৃত timer-এর মতো timer-গুলো কতটা সূক্ষ্ম হতে পারে তার প্রায়ই সীমা
থাকে। যেমন, আমরা যে `sleep` সংস্করণ ব্যবহার করছি, তাকে এক nanosecond-এর একটি `Duration` দিলেও এটি সবসময় অন্তত এক millisecond sleep করবে। আবারও, আধুনিক কম্পিউটার _দ্রুত_: এক
millisecond-এ এরা অনেক কিছু করতে পারে!

এর মানে, আপনার প্রোগ্রাম আর কী করছে তার ওপর নির্ভর করে async compute-bound কাজের জন্যও কাজে লাগতে পারে, কারণ এটি প্রোগ্রামের বিভিন্ন অংশের সম্পর্ক সাজানোর একটি কাজের হাতিয়ার দেয় (তবে
async state machine-এর বাড়তি খরচের বিনিময়ে)। এটি _cooperative multitasking_ (সহযোগিতামূলক multitasking)-এর একটি রূপ, যেখানে প্রতিটি future-এর ক্ষমতা আছে await point-এর মাধ্যমে কখন
নিয়ন্ত্রণ হস্তান্তর করবে তা ঠিক করার। তাই প্রতিটি future-এর দায়িত্বও আছে খুব দীর্ঘ সময় block না করার। Rust-ভিত্তিক কিছু embedded operating system-এ এটিই _একমাত্র_ ধরনের multitasking!

অবশ্য বাস্তবের কোডে সাধারণত প্রতিটি লাইনে function call আর await point পালা করে রাখবেন না। এভাবে নিয়ন্ত্রণ ছেড়ে দেওয়া তুলনামূলকভাবে সস্তা হলেও বিনামূল্যে নয়। অনেক ক্ষেত্রে একটি
compute-bound কাজ ভাগ করার চেষ্টা একে উল্লেখযোগ্যভাবে ধীর করে দিতে পারে, তাই কখনো কখনো _সামগ্রিক_ performance-এর জন্য একটি operation-কে অল্প সময় block করতে দেওয়াই ভালো। আপনার
কোডের আসল performance-এর বাধা কোথায় তা সবসময় মেপে দেখুন। তবে আপনি যদি concurrent-ভাবে ঘটবে বলে আশা করা অনেক কাজ ক্রমানুসারে ঘটতে _দেখেন_, তাহলে এর ভেতরের গতিপ্রকৃতি মনে রাখা
গুরুত্বপূর্ণ!

### নিজস্ব async abstraction তৈরি করা {#building-our-own-async-abstractions}

future-গুলো একসঙ্গে জুড়ে নতুন pattern-ও তৈরি করা যায়। যেমন, আমাদের হাতে থাকা async উপাদান দিয়ে একটি `timeout` function বানাতে পারি। কাজ শেষ হলে ফলাফলটি হবে আরেকটি উপাদান, যা দিয়ে
আরও async abstraction তৈরি করা যাবে।

Listing 17-18-এ দেখানো হয়েছে একটি ধীর future-এর সঙ্গে এই `timeout` কীভাবে কাজ করবে বলে আমরা আশা করি।

<Listing number="17-18" caption="সময়সীমাসহ একটি ধীর operation চালাতে আমাদের কল্পিত `timeout` ব্যবহার করা" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-18/src/main.rs:here}}
```

</Listing>

চলুন এটি implement করি! শুরুতে `timeout`-এর API নিয়ে ভাবা যাক:

- একে নিজেই একটি async function হতে হবে, যাতে এর জন্য await করা যায়।
- এর প্রথম parameter হওয়া উচিত চালানোর জন্য একটি future। যেকোনো future-এর সঙ্গে কাজ করতে দিতে একে generic করতে পারি।
- এর দ্বিতীয় parameter হবে অপেক্ষার সর্বোচ্চ সময়। একটি `Duration` ব্যবহার করলে `trpl::sleep`-এ পাঠানো সহজ হবে।
- এটি একটি `Result` return করা উচিত। future সফলভাবে শেষ হলে `Result` হবে future-এর তৈরি মানসহ `Ok`। timeout আগে পেরিয়ে গেলে `Result` হবে timeout যতক্ষণ অপেক্ষা করেছে সেই
  duration-সহ `Err`।

Listing 17-19-এ এই ঘোষণাটি দেখানো হয়েছে।

<!-- This is not tested because it intentionally does not compile. -->

<Listing number="17-19" caption="`timeout`-এর signature সংজ্ঞায়িত করা" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-19/src/main.rs:declaration}}
```

</Listing>

এতে type-এর ক্ষেত্রে আমাদের লক্ষ্য পূরণ হয়। এবার আমাদের দরকারি _আচরণ_ নিয়ে ভাবা যাক: আমরা পাঠানো future আর duration-এর মধ্যে প্রতিযোগিতা করাতে চাই। duration থেকে একটি timer future
তৈরি করতে `trpl::sleep` ব্যবহার করতে পারি, আর caller-এর পাঠানো future-এর সঙ্গে সেই timer চালাতে `trpl::select` ব্যবহার করতে পারি।

Listing 17-20-এ আমরা `trpl::select`-এর জন্য await করার ফলের ওপর match করে `timeout` implement করি।

<Listing number="17-20" caption="`select` আর `sleep` দিয়ে `timeout` সংজ্ঞায়িত করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-20/src/main.rs:implementation}}
```

</Listing>

`trpl::select`-এর implementation ন্যায্য নয়: এটি সবসময় argument-গুলো যে ক্রমে পাঠানো হয়েছে সেই ক্রমে poll করে (অন্য `select` implementation এলোমেলোভাবে বেছে নেয় কোন argument আগে poll
করবে)। তাই আমরা `future_to_try`-কে প্রথমে `select`-এ পাঠাই, যাতে `max_time` খুব ছোট duration হলেও এটি শেষ হওয়ার সুযোগ পায়। `future_to_try` আগে শেষ হলে `select` `future_to_try`-এর
output-সহ `Left` return করবে। `timer` আগে শেষ হলে `select` timer-এর output `()`-সহ `Right` return করবে।

`future_to_try` সফল হলে এবং আমরা একটি `Left(output)` পেলে `Ok(output)` return করি। এর বদলে sleep timer পেরিয়ে গেলে এবং আমরা একটি `Right(())` পেলে `_` দিয়ে `()` উপেক্ষা করে বরং
`Err(max_time)` return করি।

এর মাধ্যমে আমাদের কাছে আরও দুটি async সহায়ক দিয়ে তৈরি একটি কাজ করা `timeout` আছে। কোডটি চালালে timeout-এর পরে এটি ব্যর্থতার অবস্থাটি print করবে:

```text
Failed after 2 seconds
```

future যেহেতু অন্য future-এর সঙ্গে জোড়া লাগে, তাই ছোট ছোট async উপাদান দিয়ে সত্যিই শক্তিশালী হাতিয়ার বানানো যায়। যেমন, এই একই পদ্ধতিতে timeout-কে retry (আবার চেষ্টা)-এর সঙ্গে মেলাতে
পারেন, আর সেগুলোকে আবার network call-এর মতো operation-এর সঙ্গে ব্যবহার করতে পারেন (যেমন Listing 17-5-এরগুলো)।

বাস্তবে আপনি সাধারণত সরাসরি `async` আর `await` নিয়ে কাজ করবেন, আর গৌণভাবে `select`-এর মতো function আর `join!` macro-র মতো macro নিয়ে, যাতে সবচেয়ে বাইরের future-গুলো কীভাবে চলবে তা
নিয়ন্ত্রণ করা যায়।

একই সময়ে একাধিক future নিয়ে কাজ করার কয়েকটি উপায় আমরা দেখলাম। এরপর দেখব _streams_ (stream) দিয়ে সময়ের সঙ্গে ক্রমানুসারে একাধিক future নিয়ে কীভাবে কাজ করা যায়।

{{#quiz ../quizzes/async-03-more-futures.toml}}

[async-program]: ch17-01-futures-and-syntax.html#our-first-async-program
