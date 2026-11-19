<!-- Old headings. Do not remove or links may break. -->

<a id="streams"></a>

## stream: ক্রমানুসারে future {#streams-futures-in-sequence}

মনে করুন, এই অধ্যায়ের আগের দিকে [“message passing”][17-02-messages]<!-- ignore --> অংশে আমরা কীভাবে আমাদের async channel-এর receiver ব্যবহার করেছিলাম। async
`recv` method সময়ের সঙ্গে item-এর একটি ধারা তৈরি করে। এটি _stream_ নামে পরিচিত অনেক বেশি সাধারণ একটি pattern-এর উদাহরণ। অনেক ধারণা স্বাভাবিকভাবেই stream হিসেবে প্রকাশ পায়: একটি
queue-তে item পাওয়া যেতে থাকা, পুরো ডেটা কম্পিউটারের মেমোরির জন্য খুব বড় হলে filesystem থেকে ধাপে ধাপে ডেটার টুকরো টেনে আনা, বা সময়ের সঙ্গে network দিয়ে ডেটা আসা।
stream যেহেতু future, তাই এদের অন্য যেকোনো ধরনের future-এর সঙ্গে ব্যবহার করা যায় এবং মজার উপায়ে মেলানো যায়। যেমন, খুব বেশি network call চালু হওয়া এড়াতে event-গুলো দলবদ্ধ করা যায়,
দীর্ঘ সময় ধরে চলা operation-এর ধারায় timeout বসানো যায়, বা অপ্রয়োজনীয় কাজ এড়াতে user interface-এর event-এর গতি কমিয়ে দেওয়া যায় (throttle করা)।

item-এর ধারা আমরা অধ্যায় 13-এ দেখেছিলাম, যখন [“`Iterator` trait ও `next` method”][iterator-trait]<!--
ignore --> অংশে Iterator trait দেখেছিলাম, তবে iterator আর async channel receiver-এর মধ্যে দুটি পার্থক্য আছে। প্রথম পার্থক্য সময়: iterator synchronous, আর channel receiver
asynchronous। দ্বিতীয় পার্থক্য API। সরাসরি `Iterator` নিয়ে কাজ করার সময় আমরা এর synchronous `next` method call করি। বিশেষ করে `trpl::Receiver` stream-এর ক্ষেত্রে আমরা বরং একটি
asynchronous `recv` method call করেছিলাম। এ ছাড়া এই API-গুলো খুবই কাছাকাছি মনে হয়, আর এই মিল কাকতালীয় নয়। stream হলো iteration-এর একটি asynchronous রূপের মতো। তবে `trpl::Receiver`
যেখানে বিশেষভাবে message গ্রহণের জন্য অপেক্ষা করে, সাধারণ-উদ্দেশ্যের stream API সেখানে অনেক বেশি বিস্তৃত: এটি `Iterator`-এর মতোই পরের item দেয়, তবে asynchronous-ভাবে।

Rust-এ iterator আর stream-এর মিলের মানে হলো আমরা আসলে যেকোনো iterator থেকে একটি stream তৈরি করতে পারি। iterator-এর মতোই stream-এর `next` method call করে তারপর output-এর জন্য await
করে stream নিয়ে কাজ করা যায়, যেমনটি Listing
17-21-এ, যা এখনো compile হবে না।

<Listing number="17-21" caption="একটি iterator থেকে stream তৈরি করে এর মানগুলো print করা" file-name="src/main.rs">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch17-async-await/listing-17-21/src/main.rs:stream}}
```

</Listing>

আমরা সংখ্যার একটি array দিয়ে শুরু করি, যাকে একটি iterator-এ রূপান্তর করে তারপর সব মান দ্বিগুণ করতে `map` call করি। তারপর `trpl::stream_from_iter` function ব্যবহার করে iterator-কে
একটি stream-এ রূপান্তর করি। এরপর `while let` loop দিয়ে stream-এর item-গুলো আসার সঙ্গে সঙ্গে সেগুলোর ওপর দিয়ে loop করি।

দুর্ভাগ্যবশত, কোডটি চালানোর চেষ্টা করলে এটি compile হয় না, বরং জানায় যে কোনো `next` method পাওয়া যাচ্ছে না:

<!-- manual-regeneration
cd listings/ch17-async-await/listing-17-21
cargo build
copy only the error output
-->

```text
error[E0599]: no method named `next` found for struct `tokio_stream::iter::Iter` in the current scope
  --> src/main.rs:10:40
   |
10 |         while let Some(value) = stream.next().await {
   |                                        ^^^^
   |
   = help: items from traits can only be used if the trait is in scope
help: the following traits which provide `next` are implemented but not in scope; perhaps you want to import one of them
   |
1  + use crate::trpl::StreamExt;
   |
1  + use futures_util::stream::stream::StreamExt;
   |
1  + use std::iter::Iterator;
   |
1  + use std::str::pattern::Searcher;
   |
help: there is a method `try_next` with a similar name
   |
10 |         while let Some(value) = stream.try_next().await {
   |                                        ~~~~~~~~
```

এই output যেমন ব্যাখ্যা করে, compiler error-এর কারণ হলো `next` method ব্যবহার করতে পারতে সঠিক trait-টি scope-এ থাকতে হবে। এ পর্যন্ত আলোচনার ভিত্তিতে আপনি যুক্তিসঙ্গতভাবেই আশা করতে
পারেন যে trait-টি `Stream`, কিন্তু আসলে এটি `StreamExt`। _extension_-এর সংক্ষেপ `Ext` হলো Rust community-তে এক trait দিয়ে আরেক trait-এর ক্ষমতা বাড়ানোর একটি প্রচলিত pattern।

`Stream` trait একটি নিম্ন-স্তরের interface সংজ্ঞায়িত করে, যা কার্যত `Iterator` আর `Future` trait-কে মিলিয়ে দেয়। `StreamExt` `Stream`-এর ওপরে উচ্চ-স্তরের API-র একটি সেট দেয়, যার মধ্যে
আছে `next` method এবং `Iterator` trait-এর দেওয়া method-এর মতো আরও কিছু কাজের method। `Stream` আর `StreamExt` এখনো Rust-এর standard library-র অংশ নয়, তবে ecosystem-এর বেশিরভাগ crate একই রকম
সংজ্ঞা ব্যবহার করে।

compiler error-এর সমাধান হলো `trpl::StreamExt`-এর জন্য একটি `use` statement যোগ করা, যেমনটি Listing 17-22-এ।

<Listing number="17-22" caption="একটি stream-এর ভিত্তি হিসেবে সফলভাবে একটি iterator ব্যবহার করা" file-name="src/main.rs">

```rust
{{#rustdoc_include ../listings/ch17-async-await/listing-17-22/src/main.rs:all}}
```

</Listing>

এই সব অংশ একসঙ্গে জোড়া দেওয়ায় কোডটি আমাদের কাঙ্ক্ষিতভাবে কাজ করে! আরও কী, এখন `StreamExt` scope-এ থাকায় iterator-এর মতোই এর সব কাজের method ব্যবহার করতে পারি।

{{#quiz ../quizzes/async-04-streams.toml}}

[17-02-messages]: ch17-02-concurrency-with-async.html#message-passing
[iterator-trait]: ch13-02-iterators.html#the-iterator-trait-and-the-next-method
