<!-- Old headings. Do not remove or links may break. -->

<a id="using-message-passing-to-transfer-data-between-threads"></a>

## message passing দিয়ে thread-এর মধ্যে ডেটা আদান-প্রদান {#transfer-data-between-threads-with-message-passing}

নিরাপদ concurrency নিশ্চিত করার একটি ক্রমশ জনপ্রিয় পদ্ধতি হলো message passing (বার্তা আদান-প্রদান), যেখানে thread বা actor-রা একে অপরকে ডেটাযুক্ত message পাঠিয়ে
যোগাযোগ করে। [Go ভাষার documentation](https://golang.org/doc/effective_go.html#concurrency)-এর একটি স্লোগানে ধারণাটি এমন:
“মেমোরি ভাগ করে যোগাযোগ করো না; বরং যোগাযোগ করে মেমোরি ভাগ করো।”

message পাঠানোভিত্তিক concurrency সম্ভব করতে Rust-এর standard library channel-এর একটি implementation দেয়। _channel_ হলো একটি সাধারণ প্রোগ্রামিং ধারণা, যার মাধ্যমে এক
thread থেকে আরেক thread-এ ডেটা পাঠানো হয়।

প্রোগ্রামিংয়ের channel-কে আপনি একমুখী জলপ্রবাহের মতো কল্পনা করতে পারেন, যেমন একটি খাল বা নদী। নদীতে একটি রাবারের হাঁসের মতো কিছু ছেড়ে দিলে সেটি স্রোতের সঙ্গে ভেসে
জলপথের শেষ পর্যন্ত চলে যাবে।

একটি channel-এর দুটি অংশ: একটি transmitter (প্রেরক) আর একটি receiver (গ্রাহক)। transmitter অংশটি হলো উজানের সেই জায়গা যেখানে আপনি নদীতে রাবারের হাঁসটি ছাড়েন, আর receiver
অংশটি হলো ভাটির সেই জায়গা যেখানে হাঁসটি গিয়ে পৌঁছায়। আপনার কোডের একটি অংশ যে ডেটা পাঠাতে চান তা দিয়ে transmitter-এর ওপর method call করে, আর আরেকটি অংশ আসা
message-এর জন্য গ্রহণের প্রান্তটি পরীক্ষা করে। transmitter বা receiver, যেকোনো অংশ drop হলে channel-টিকে _closed_ (বন্ধ) বলা হয়।

এখানে আমরা ধাপে ধাপে এমন একটি প্রোগ্রাম বানাব, যার একটি thread মান তৈরি করে channel দিয়ে পাঠায়, আর আরেকটি thread মানগুলো গ্রহণ করে print করে। বৈশিষ্ট্যটি দেখাতে আমরা
channel দিয়ে thread-গুলোর মধ্যে সাধারণ মান পাঠাব। কৌশলটির সঙ্গে পরিচিত হয়ে গেলে একে অপরের সঙ্গে যোগাযোগ করতে হয় এমন যেকোনো thread-এর জন্য channel ব্যবহার করতে পারবেন,
যেমন একটি chat system, বা এমন একটি system যেখানে অনেক thread একটি হিসাবের বিভিন্ন অংশ করে সেই অংশগুলো এমন একটি thread-এ পাঠায় যা ফলগুলো একত্র করে।

প্রথমে Listing 16-6-এ আমরা একটি channel তৈরি করব, কিন্তু এটি দিয়ে কিছু করব না। খেয়াল করুন, এটি এখনো compile হবে না, কারণ channel দিয়ে আমরা কোন type-এর মান পাঠাতে
চাই তা Rust বলতে পারে না।

<Listing number="16-6" file-name="src/main.rs" caption="একটি channel তৈরি করে এর দুটি অংশ `tx` আর `rx`-এ রাখা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-06/src/main.rs}}
```

</Listing>

আমরা `mpsc::channel` function ব্যবহার করে একটি নতুন channel তৈরি করি; `mpsc`-এর পূর্ণরূপ _multiple producer, single consumer_ (একাধিক উৎপাদক, একক ভোক্তা)। সংক্ষেপে,
Rust-এর standard library যেভাবে channel implement করে, তাতে একটি channel-এর মান তৈরি করে এমন একাধিক _পাঠানোর_ প্রান্ত থাকতে পারে, কিন্তু সেই মানগুলো গ্রহণ করে এমন
_গ্রহণের_ প্রান্ত মাত্র একটি। কল্পনা করুন একাধিক খাল একসঙ্গে মিশে একটি বড় নদীতে পড়ছে: যেকোনো খাল দিয়ে পাঠানো সবকিছু শেষে একটি নদীতে গিয়ে মিশবে। আপাতত আমরা একটি
producer দিয়ে শুরু করব, তবে উদাহরণটি কাজ করানোর পর একাধিক producer যোগ করব।

`mpsc::channel` function একটি tuple return করে, যার প্রথম উপাদান পাঠানোর প্রান্ত—transmitter—আর দ্বিতীয় উপাদান গ্রহণের প্রান্ত—receiver। অনেক ক্ষেত্রে _transmitter_
আর _receiver_-এর জন্য যথাক্রমে `tx` আর `rx` সংক্ষেপ প্রথাগতভাবে ব্যবহৃত হয়, তাই প্রতিটি প্রান্ত বোঝাতে আমরা variable-গুলোর নাম সেভাবেই দিই। আমরা tuple ভেঙে নেয়
(destructure করে) এমন একটি pattern-সহ `let` statement ব্যবহার করছি; `let` statement-এ pattern-এর ব্যবহার আর destructuring নিয়ে অধ্যায় 19-এ আলোচনা করব। আপাতত জেনে
রাখুন, `mpsc::channel`-এর return করা tuple-এর অংশগুলো বের করতে এভাবে `let` statement ব্যবহার করা একটি সুবিধাজনক পদ্ধতি।

চলুন পাঠানোর প্রান্তটি একটি চালু করা thread-এ সরিয়ে নিয়ে তাকে দিয়ে একটি string পাঠাই, যাতে চালু করা thread main thread-এর সঙ্গে যোগাযোগ করে, যেমনটি Listing 16-7-এ দেখানো
হয়েছে। এটি উজানে নদীতে একটি রাবারের হাঁস ছেড়ে দেওয়া, বা এক thread থেকে আরেক thread-এ chat message পাঠানোর মতো।

<Listing number="16-7" file-name="src/main.rs" caption='`tx`-কে একটি চালু করা thread-এ সরিয়ে নিয়ে `"hi"` পাঠানো'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-07/src/main.rs}}
```

</Listing>

আবারও আমরা নতুন thread তৈরি করতে `thread::spawn` ব্যবহার করছি, তারপর `move` ব্যবহার করে `tx`-কে closure-এর ভেতরে সরিয়ে নিচ্ছি, যাতে চালু করা thread `tx`-এর মালিক হয়।
channel দিয়ে message পাঠাতে পারতে চালু করা thread-কে transmitter-এর মালিক হতে হবে।

transmitter-এর একটি `send` method আছে, যা আমরা যে মান পাঠাতে চাই তা নেয়। `send` method একটি `Result<T, E>` type return করে, তাই receiver ইতিমধ্যে drop হয়ে গিয়ে থাকলে
এবং মান পাঠানোর কোনো জায়গা না থাকলে send operation একটি error return করবে। এই উদাহরণে error হলে panic করাতে আমরা `unwrap` call করছি। কিন্তু বাস্তব application-এ আমরা
একে ঠিকমতো সামলাতাম: যথাযথ error handling-এর কৌশলগুলো আবার দেখতে অধ্যায় 9-এ ফিরে যান।

Listing 16-8-এ আমরা main thread-এ receiver থেকে মানটি নেব। এটি নদীর শেষে পানি থেকে রাবারের হাঁসটি তুলে নেওয়া, বা একটি chat message গ্রহণ করার মতো।

<Listing number="16-8" file-name="src/main.rs" caption='main thread-এ `"hi"` মানটি গ্রহণ করে print করা'>

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-08/src/main.rs}}
```

</Listing>

receiver-এর দুটি কাজের method আছে: `recv` আর `try_recv`। আমরা ব্যবহার করছি `recv`, _receive_-এর সংক্ষেপ, যা main thread-এর চলা block করে রাখবে এবং channel দিয়ে একটি
মান পাঠানো না হওয়া পর্যন্ত অপেক্ষা করবে। একটি মান পাঠানো হলে `recv` সেটি একটি `Result<T, E>`-এ return করবে। transmitter বন্ধ হয়ে গেলে `recv` একটি error return করবে, যা
জানায় যে আর কোনো মান আসবে না।

`try_recv` method block করে না, বরং সঙ্গে সঙ্গে একটি `Result<T, E>` return করবে: একটি message পাওয়া গেলে সেটি রাখা একটি `Ok` মান, আর এই মুহূর্তে কোনো message না থাকলে
একটি `Err` মান। message-এর জন্য অপেক্ষা করার সময় এই thread-এর অন্য কাজ থাকলে `try_recv` ব্যবহার কাজে লাগে: আমরা এমন একটি loop লিখতে পারি যা মাঝে মাঝে `try_recv`
call করে, message পাওয়া গেলে সেটি সামলায়, আর না পাওয়া গেলে আবার যাচাই করার আগে কিছুক্ষণ অন্য কাজ করে।

সরলতার জন্য এই উদাহরণে আমরা `recv` ব্যবহার করেছি; message-এর জন্য অপেক্ষা করা ছাড়া main thread-এর আর কোনো কাজ নেই, তাই main thread block করা উপযুক্ত।

Listing 16-8-এর কোড চালালে main thread থেকে মানটি print হতে দেখব:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
```

চমৎকার!

<!-- Old headings. Do not remove or links may break. -->

<a id="channels-and-ownership-transference"></a>

### channel দিয়ে ownership হস্তান্তর {#transferring-ownership-through-channels}

message পাঠানোয় ownership-এর নিয়ম গুরুত্বপূর্ণ ভূমিকা রাখে, কারণ এরা আপনাকে নিরাপদ, concurrent কোড লিখতে সাহায্য করে। আপনার পুরো Rust প্রোগ্রাম জুড়ে ownership নিয়ে ভাবার
সুফল হলো concurrent programming-এ error ঠেকানো। channel আর ownership কীভাবে একসঙ্গে কাজ করে সমস্যা ঠেকায় তা দেখাতে চলুন একটি পরীক্ষা করি: channel দিয়ে পাঠানোর _পরে_
আমরা চালু করা thread-এ একটি `val` মান ব্যবহারের চেষ্টা করব। কেন এই কোড অনুমোদিত নয় তা দেখতে Listing 16-9-এর কোড compile করে দেখুন।

<Listing number="16-9" file-name="src/main.rs" caption="channel দিয়ে পাঠানোর পরে `val` ব্যবহারের চেষ্টা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-09/src/main.rs}}
```

</Listing>

এখানে `tx.send` দিয়ে channel-এ পাঠানোর পরে আমরা `val` print করার চেষ্টা করছি। এটি অনুমোদন করা হবে খারাপ ধারণা: মানটি একবার অন্য thread-এ পাঠানো হলে, আমরা আবার মানটি
ব্যবহারের চেষ্টা করার আগেই সেই thread এটি পরিবর্তন বা drop করে দিতে পারে। অসংগত বা অস্তিত্বহীন ডেটার কারণে অন্য thread-এর পরিবর্তন হয়তো error বা অপ্রত্যাশিত ফল ঘটাতে
পারত। তবে Listing 16-9-এর কোড compile করার চেষ্টা করলে Rust আমাদের একটি error দেয়:

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-09/output.txt}}
```

আমাদের concurrency-র ভুল একটি compile-time error ঘটিয়েছে। `send` function তার parameter-এর ownership নেয়, আর মানটি সরে গেলে receiver এর ownership নেয়। এটি পাঠানোর পরে
ভুল করে আবার মানটি ব্যবহার করা থেকে আমাদের আটকায়; ownership system যাচাই করে যে সব ঠিক আছে।

<!-- Old headings. Do not remove or links may break. -->

<a id="sending-multiple-values-and-seeing-the-receiver-waiting"></a>

### একাধিক মান পাঠানো {#sending-multiple-values}

Listing 16-8-এর কোড compile হয়ে চলেছে, কিন্তু এটি পরিষ্কারভাবে দেখায়নি যে দুটি আলাদা thread channel দিয়ে একে অপরের সঙ্গে কথা বলছিল।

Listing 16-10-এ আমরা কিছু পরিবর্তন করেছি, যা প্রমাণ করবে যে Listing 16-8-এর কোড concurrent-ভাবে চলছে: চালু করা thread এখন একাধিক message পাঠাবে এবং প্রতিটি message-এর
মাঝে এক সেকেন্ড থামবে।

<Listing number="16-10" file-name="src/main.rs" caption="একাধিক message পাঠানো এবং প্রতিটির মাঝে থামা">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-10/src/main.rs}}
```

</Listing>

এবার চালু করা thread-এর কাছে string-এর একটি vector আছে, যা আমরা main thread-এ পাঠাতে চাই। আমরা সেগুলোর ওপর দিয়ে iterate করে প্রতিটি আলাদাভাবে পাঠাই, আর এক সেকেন্ডের
`Duration` মান দিয়ে `thread::sleep` function call করে প্রতিটির মাঝে থামি।

main thread-এ আমরা আর স্পষ্টভাবে `recv` function call করছি না: বরং `rx`-কে একটি iterator হিসেবে ব্যবহার করছি। গ্রহণ করা প্রতিটি মান print করছি। channel বন্ধ হলে iteration
শেষ হবে।

Listing 16-10-এর কোড চালালে প্রতিটি লাইনের মাঝে এক সেকেন্ডের বিরতিসহ এই output দেখার কথা:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: from
Got: the
Got: thread
```

main thread-এর `for` loop-এ থামায় বা দেরি করায় এমন কোনো কোড নেই বলে আমরা বুঝতে পারি যে main thread চালু করা thread থেকে মান গ্রহণের জন্য অপেক্ষা করছে।

<!-- Old headings. Do not remove or links may break. -->

<a id="creating-multiple-producers-by-cloning-the-transmitter"></a>

### একাধিক producer তৈরি করা {#creating-multiple-producers}

আগে আমরা বলেছিলাম `mpsc` হলো _multiple producer, single
consumer_-এর সংক্ষেপ। চলুন `mpsc` কাজে লাগিয়ে Listing 16-10-এর কোড বাড়িয়ে একাধিক thread তৈরি করি, যারা সবাই একই receiver-এ মান পাঠায়। transmitter clone করে এটা করা
যায়, যেমনটি Listing 16-11-এ দেখানো হয়েছে।

<Listing number="16-11" file-name="src/main.rs" caption="একাধিক producer থেকে একাধিক message পাঠানো">

```rust,noplayground
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-11/src/main.rs:here}}
```

</Listing>

এবার প্রথম চালু করা thread তৈরির আগে আমরা transmitter-এর ওপর `clone` call করি। এতে একটি নতুন transmitter পাব, যা প্রথম চালু করা thread-এ পাঠাতে পারি। মূল transmitter-টি
পাঠাই দ্বিতীয় একটি চালু করা thread-এ। এতে আমরা দুটি thread পাই, যারা প্রত্যেকে একই receiver-এ ভিন্ন ভিন্ন message পাঠায়।

কোডটি চালালে আপনার output দেখতে মোটামুটি এমন হওয়ার কথা:

<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -->

```text
Got: hi
Got: more
Got: from
Got: messages
Got: for
Got: the
Got: thread
Got: you
```

আপনার system-এর ওপর নির্ভর করে হয়তো মানগুলো অন্য ক্রমে দেখবেন। এটিই concurrency-কে মজার এবং কঠিন করে তোলে। বিভিন্ন thread-এ `thread::sleep`-কে ভিন্ন ভিন্ন মান দিয়ে
পরীক্ষা করলে প্রতিবার চালানো আরও অনিশ্চিত হবে এবং প্রতিবার ভিন্ন output তৈরি করবে।

channel কীভাবে কাজ করে তা দেখা হলো, এবার concurrency-র একটি ভিন্ন পদ্ধতি দেখা যাক।

{{#quiz ../quizzes/ch16-02-message-passing.toml}}