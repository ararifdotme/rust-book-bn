## সুশৃঙ্খলভাবে বন্ধ করা ও পরিষ্কার করা {#graceful-shutdown-and-cleanup}

Listing 21-20-এর কোড আমাদের উদ্দেশ্য অনুযায়ী একটি thread pool ব্যবহার করে asynchronous-ভাবে request-এর সাড়া দিচ্ছে। `workers`, `id` আর `thread` field নিয়ে কিছু warning পাই, যেগুলো আমরা সরাসরি ব্যবহার করছি না; এগুলো
মনে করিয়ে দেয় যে আমরা কিছুই পরিষ্কার করছি না। main thread থামাতে যখন কম মার্জিত
<kbd>ctrl</kbd>-<kbd>C</kbd> পদ্ধতি ব্যবহার করি, তখন বাকি সব thread-ও সঙ্গে সঙ্গে থেমে যায়, এমনকি সেগুলো কোনো request সামলানোর মাঝখানে থাকলেও।

তাই এরপর আমরা `Drop` trait implement করব, যাতে pool-এর প্রতিটি thread-এর ওপর `join` call করা হয় এবং সেগুলো বন্ধ হওয়ার আগে চলমান request-গুলো শেষ করতে পারে। তারপর thread-গুলোকে নতুন request গ্রহণ বন্ধ করে
বন্ধ হয়ে যেতে বলার একটি উপায় implement করব। এই কোড কাজ করতে দেখতে আমরা server বদলে দেব, যাতে এটি মাত্র দুটি request গ্রহণ করে তারপর সুশৃঙ্খলভাবে thread pool বন্ধ করে।

এগোতে এগোতে একটি বিষয় খেয়াল করুন: এর কোনোটিই কোডের সেই অংশগুলোকে প্রভাবিত করে না, যেগুলো closure চালানো সামলায়, তাই async runtime-এর জন্য thread pool ব্যবহার করলেও এখানকার সবকিছু একই থাকত।

### `ThreadPool`-এ `Drop` trait implement করা {#implementing-the-drop-trait-on-threadpool}

চলুন আমাদের thread pool-এ `Drop` implement করা দিয়ে শুরু করি। pool drop হলে আমাদের সব thread-এর join হওয়া উচিত, যাতে নিশ্চিত হয় সেগুলো নিজেদের কাজ শেষ করেছে। Listing 21-22-এ `Drop` implementation-এর প্রথম
প্রচেষ্টা দেখানো হয়েছে; এই কোড এখনো পুরোপুরি কাজ করবে না।

<Listing number="21-22" file-name="src/lib.rs" caption="thread pool scope-এর বাইরে গেলে প্রতিটি thread join করা">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch21-web-server/listing-21-22/src/lib.rs:here}}
```

</Listing>

প্রথমে আমরা thread pool-এর প্রতিটি `workers`-এর ওপর দিয়ে loop করি। এর জন্য `&mut` ব্যবহার করি, কারণ `self` একটি mutable reference, আর আমাদের `worker`-কেও পরিবর্তন করতে পারতে হবে। প্রতিটি `worker`-এর জন্য
একটি message print করি যে এই নির্দিষ্ট `Worker` instance বন্ধ হচ্ছে, তারপর সেই `Worker` instance-এর thread-এর ওপর `join` call করি। `join` call ব্যর্থ হলে `unwrap` ব্যবহার করে Rust-কে panic করাই এবং
বিশৃঙ্খলভাবে বন্ধ হতে দিই।

এই কোড compile করলে আমরা এই error পাই:

```console
{{#include ../listings/ch21-web-server/listing-21-22/output.txt}}
```

error বলছে আমরা `join` call করতে পারি না, কারণ আমাদের কাছে প্রতিটি `worker`-এর শুধু একটি mutable borrow আছে, আর `join` তার argument-এর ownership নেয়। এই সমস্যার সমাধানে আমাদের `thread`-এর মালিক `Worker`
instance থেকে thread-টি বের করে নিতে হবে, যাতে `join` thread-টি consume করতে পারে। এটি করার একটি উপায় হলো Listing 18-15-এ নেওয়া পদ্ধতি অনুসরণ করা। `Worker` যদি একটি
`Option<thread::JoinHandle<()>>` রাখত, তাহলে `Option`-এর ওপর `take` method call করে `Some` variant থেকে মানটি বের করে নিয়ে তার জায়গায় একটি `None` variant রেখে দিতে পারতাম। অন্যভাবে বললে, চলমান
একটি `Worker`-এর `thread`-এ একটি `Some` variant থাকত, আর একটি `Worker` পরিষ্কার করতে চাইলে `Some`-কে `None` দিয়ে প্রতিস্থাপন করতাম, যাতে `Worker`-এর চালানোর মতো কোনো thread না থাকে।

তবে এটির দরকার পড়ত _শুধু_ `Worker` drop করার সময়। বিনিময়ে যেখানেই `worker.thread`-এ প্রবেশ করতাম, সেখানেই একটি
`Option<thread::JoinHandle<()>>` সামলাতে হতো। idiomatic Rust `Option` বেশ ব্যবহার করে, কিন্তু যখন দেখবেন এমন কিছু, যা সবসময় থাকবে বলে জানেন, তা এভাবে একটি কৌশল হিসেবে `Option`-এ মুড়ছেন, তখন কোড
আরও পরিচ্ছন্ন ও কম ভুলপ্রবণ করতে বিকল্প পদ্ধতি খোঁজা ভালো।

এই ক্ষেত্রে একটি ভালো বিকল্প আছে: `Vec::drain` method। এটি vector থেকে কোন item-গুলো সরাতে হবে তা নির্দিষ্ট করতে একটি range parameter নেয় এবং সেই item-গুলোর একটি iterator return করে। `..` range syntax
পাঠালে vector থেকে *প্রতিটি* মান সরে যাবে।

তাই `ThreadPool`-এর `drop` implementation এভাবে হালনাগাদ করতে হবে:

<Listing file-name="src/lib.rs">

```rust
{{#rustdoc_include ../listings/ch21-web-server/no-listing-04-update-drop-definition/src/lib.rs:here}}
```

</Listing>

এতে compiler error-এর সমাধান হয় এবং আমাদের কোডে আর কোনো পরিবর্তন লাগে না। খেয়াল করুন, panic করার সময় drop call হতে পারে বলে unwrap-ও panic করে একটি double panic ঘটাতে পারে, যা প্রোগ্রামকে সঙ্গে সঙ্গে
crash করায় এবং চলমান যেকোনো পরিষ্কারের কাজ থামিয়ে দেয়। একটি উদাহরণ প্রোগ্রামের জন্য এটি ঠিক আছে, তবে production কোডের জন্য এটি সুপারিশ করা হয় না।

### thread-গুলোকে job শোনা বন্ধ করার সংকেত দেওয়া {#signaling-to-the-threads-to-stop-listening-for-jobs}

আমাদের করা সব পরিবর্তনের পর কোড কোনো warning ছাড়াই compile হয়। তবে খারাপ খবর হলো, এই কোড এখনো আমাদের চাওয়া মতো কাজ করে না। মূল বিষয় হলো `Worker` instance-গুলোর thread যে closure চালায় তার যুক্তি:
এই মুহূর্তে আমরা `join` call করি, কিন্তু তাতে thread-গুলো বন্ধ হবে না, কারণ সেগুলো job খুঁজতে চিরকাল `loop` করে। বর্তমান `drop` implementation দিয়ে আমাদের `ThreadPool` drop করার চেষ্টা করলে main thread
প্রথম thread শেষ হওয়ার অপেক্ষায় চিরকাল block হয়ে থাকবে।

এই সমস্যা ঠিক করতে `ThreadPool`-এর `drop` implementation-এ একটি পরিবর্তন, তারপর `Worker`-এর loop-এ একটি পরিবর্তন লাগবে।

প্রথমে `ThreadPool`-এর `drop` implementation বদলাব, যাতে thread-গুলোর শেষ হওয়ার অপেক্ষা করার আগে স্পষ্টভাবে `sender` drop করা হয়। Listing 21-23-এ স্পষ্টভাবে `sender` drop করতে `ThreadPool`-এর
পরিবর্তনগুলো দেখানো হয়েছে। thread-এর ক্ষেত্রের মতো নয়, এখানে `Option::take` দিয়ে `ThreadPool` থেকে `sender` বের করে নিতে আমাদের সত্যিই একটি `Option` ব্যবহার _করতে হবে_।

<Listing number="21-23" file-name="src/lib.rs" caption="`Worker` thread-গুলো join করার আগে স্পষ্টভাবে `sender` drop করা">

```rust,noplayground,not_desired_behavior
{{#rustdoc_include ../listings/ch21-web-server/listing-21-23/src/lib.rs:here}}
```

</Listing>

`sender` drop করলে channel বন্ধ হয়ে যায়, যা বোঝায় আর কোনো message পাঠানো হবে না। তা ঘটলে `Worker` instance-গুলো অসীম loop-এ `recv`-এর যে call করে, সেগুলো সব error return করবে। Listing 21-24-এ আমরা `Worker`-এর
loop বদলে দিই, যাতে সেই ক্ষেত্রে loop থেকে সুশৃঙ্খলভাবে বেরিয়ে আসে; অর্থাৎ `ThreadPool`-এর `drop` implementation thread-গুলোর ওপর `join` call করলে সেগুলো শেষ হবে।

<Listing number="21-24" file-name="src/lib.rs" caption="`recv` error return করলে স্পষ্টভাবে loop থেকে বেরিয়ে আসা">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/listing-21-24/src/lib.rs:here}}
```

</Listing>

এই কোড কাজ করতে দেখতে চলুন `main` বদলে দিই, যাতে server সুশৃঙ্খলভাবে বন্ধ হওয়ার আগে মাত্র দুটি request গ্রহণ করে, যেমনটি Listing 21-25-এ দেখানো হয়েছে।

<Listing number="21-25" file-name="src/main.rs" caption="loop থেকে বেরিয়ে দুটি request সামলানোর পর server বন্ধ করা">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/listing-21-25/src/main.rs:here}}
```

</Listing>

বাস্তব জগতের একটি web server মাত্র দুটি request সামলিয়ে বন্ধ হয়ে যাক, তা আপনি চাইবেন না। এই কোড শুধু দেখায় যে সুশৃঙ্খলভাবে বন্ধ করা ও পরিষ্কারের ব্যবস্থা ঠিকঠাক কাজ করছে।

`take` method `Iterator` trait-এ সংজ্ঞায়িত, এবং এটি iteration-কে সর্বোচ্চ প্রথম দুটি item-এ সীমিত রাখে। `main`-এর শেষে `ThreadPool` scope-এর বাইরে যাবে, আর `drop` implementation চলবে।

`cargo run` দিয়ে server চালু করে তিনটি request করুন। তৃতীয় request-এ error হওয়ার কথা, আর আপনার terminal-এ এর মতো output দেখার কথা:

<!-- manual-regeneration
cd listings/ch21-web-server/listing-21-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-->

```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
     Running `target/debug/hello`
Worker 0 got a job; executing.
Shutting down.
Shutting down worker 0
Worker 3 got a job; executing.
Worker 1 disconnected; shutting down.
Worker 2 disconnected; shutting down.
Worker 3 disconnected; shutting down.
Worker 0 disconnected; shutting down.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```

আপনি হয়তো `Worker` ID আর print হওয়া message-এর ভিন্ন ক্রম দেখবেন। message-গুলো থেকে বুঝতে পারি এই কোড কীভাবে কাজ করে: `Worker` instance 0 আর 3 প্রথম দুটি request পেয়েছে। দ্বিতীয় connection-এর পর
server connection গ্রহণ বন্ধ করে দিয়েছে, আর `Worker 3` তার job শুরু করার আগেই `ThreadPool`-এর `Drop` implementation চলা শুরু করে। `sender` drop করলে সব `Worker` instance-এর সংযোগ বিচ্ছিন্ন হয় এবং
সেগুলোকে বন্ধ হতে বলা হয়। সংযোগ বিচ্ছিন্ন হলে প্রতিটি `Worker` instance একটি message print করে, তারপর thread pool প্রতিটি `Worker` thread শেষ হওয়ার অপেক্ষা করতে `join` call করে।

এই নির্দিষ্ট চালনার একটি মজার দিক খেয়াল করুন: `ThreadPool` `sender` drop করেছে, আর কোনো `Worker` error পাওয়ার আগেই আমরা `Worker 0` join করার চেষ্টা করেছি। `Worker 0` তখনো `recv` থেকে error পায়নি, তাই
main thread `Worker 0` শেষ হওয়ার অপেক্ষায় block হয়ে ছিল। এর মধ্যে `Worker 3` একটি job পেয়েছে, তারপর সব thread error পেয়েছে। `Worker 0` শেষ হলে main thread বাকি `Worker` instance-গুলো শেষ হওয়ার
অপেক্ষা করেছে। সেই সময় সেগুলো সবাই নিজেদের loop থেকে বেরিয়ে থেমে গিয়েছিল।

অভিনন্দন! আমরা এখন project শেষ করেছি; আমাদের কাছে একটি সাধারণ web server আছে, যা asynchronous-ভাবে সাড়া দিতে একটি thread pool ব্যবহার করে। আমরা server সুশৃঙ্খলভাবে বন্ধ করতে পারি, যা pool-এর সব thread
পরিষ্কার করে।

তথ্যসূত্র হিসেবে পূর্ণ কোড এখানে দেওয়া হলো:

<Listing file-name="src/main.rs">

```rust,ignore
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/main.rs}}
```

</Listing>

<Listing file-name="src/lib.rs">

```rust,noplayground
{{#rustdoc_include ../listings/ch21-web-server/no-listing-07-final-code/src/lib.rs}}
```

</Listing>

এখানে আরও অনেক কিছু করা যেত! এই project আরও উন্নত করতে চাইলে এখানে কিছু ধারণা দেওয়া হলো:

- `ThreadPool` আর এর public method-গুলোতে আরও documentation যোগ করুন।
- library-র কার্যক্ষমতার test যোগ করুন।
- `unwrap`-এর call-গুলো বদলে আরও মজবুত error handling করুন।
- web request সামলানো ছাড়া অন্য কোনো কাজ করতে `ThreadPool` ব্যবহার করুন।
- [crates.io](https://crates.io/)-তে একটি thread pool crate খুঁজে নিয়ে তার বদলে সেই crate ব্যবহার করে একই রকম একটি web server implement করুন। তারপর এর API আর মজবুতি আমাদের implement করা thread pool-এর
  সঙ্গে তুলনা করুন।

## সারসংক্ষেপ {#summary}

দারুণ! আপনি বইয়ের শেষ পর্যন্ত পৌঁছে গেছেন! Rust-এর এই সফরে আমাদের সঙ্গী হওয়ার জন্য আপনাকে ধন্যবাদ জানাই। আপনি এখন নিজের Rust project implement করতে এবং অন্যদের project-এ সাহায্য করতে প্রস্তুত। মনে রাখবেন,
অন্য Rustacean-দের একটি স্বাগত জানানো community আছে, যারা আপনার Rust যাত্রায় যেকোনো চ্যালেঞ্জে আপনাকে সানন্দে সাহায্য করবে।
